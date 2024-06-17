use serde::{Serialize, Deserialize};
use virt::{domain::Domain};
use anyhow::{Result, anyhow};
use std::fmt;
use log::{info, error, warn};
use crate::Connection;
use napi::{Env, JsNumber, JsObject, Result as NapiResult}; // Add JsObject import
use napi::bindgen_prelude::FromNapiValue; // Add FromNapiValue import
use napi_derive::napi;

#[derive(Serialize, Deserialize, Debug)]
#[napi(object)]
pub struct VmConfig {
    #[napi(js_name = "name")]
    pub name: String,
    #[napi(js_name = "ram")]
    pub ram: u64, // in MB
    #[napi(js_name = "disk")]
    pub disk: String, // disk size as String, e.g., "10G"
    #[napi(js_name = "tpm")]
    pub tpm: bool,
    #[napi(js_name = "spice")]
    pub spice: bool,
    #[napi(js_name = "vnc")]
    pub vnc: bool,
    #[napi(js_name = "osType")]
    pub os_type: String,
    #[napi(js_name = "arch")]
    pub arch: String, // Architecture, e.g., "x86_64"
    #[napi(js_name = "machineType")]
    pub machine_type: String, // Machine type, e.g., "pc-i440fx-2.9"
    #[napi(js_name = "diskImagePath")]
    pub disk_image_path: String, // Path to the disk image
}

impl FromNapiValue for VmConfig {
    unsafe fn from_napi_value(env: Env, value: napi::sys::napi_value) -> NapiResult<Self> {
        let js_object: JsObject = JsObject::from_raw(env.raw(), value)?;
        Ok(Self {
            name: js_object.get_named_property("name")?,
            ram: js_object.get_named_property("ram")?,
            disk: js_object.get_named_property("disk")?,
            tpm: js_object.get_named_property("tpm")?,
            spice: js_object.get_named_property("spice")?,
            vnc: js_object.get_named_property("vnc")?,
            os_type: js_object.get_named_property("osType")?,
            arch: js_object.get_named_property("arch")?,
            machine_type: js_object.get_named_property("machineType")?,
            disk_image_path: js_object.get_named_property("diskImagePath")?,
        })
    }
}

#[napi]
pub struct Machine {
    config: VmConfig,
    domain: Option<Domain>,
}

#[napi]
impl Machine {
    #[napi(constructor)]
    pub fn new(config: VmConfig) -> Self {
        Self { config, domain: None }
    }

    pub fn from_domain(domain: Domain) -> Result<Self> {
        // Convert Domain to Machine
        let config = VmConfig {
            name: domain.get_name()?,
            ram: domain.get_max_memory()?, // Correct method to get memory
            disk: String::new(), // Placeholder, actual disk info extraction needed
            tpm: false, // Placeholder, actual tpm info extraction needed
            spice: false, // Placeholder, actual spice info extraction needed
            vnc: false, // Placeholder, actual vnc info extraction needed
            os_type: String::new(), // Placeholder, actual os_type extraction needed
            arch: String::new(), // Placeholder, actual arch extraction needed
            machine_type: String::new(), // Placeholder, actual machine_type extraction needed
            disk_image_path: String::new(), // Placeholder, actual disk_image_path extraction needed
        };
        Ok(Self { config, domain: Some(domain) })
    }

    pub fn deploy(&mut self, conn: &Connection) -> Result<()> {
        let xml = self.generate_domain_xml()?;
        match conn.define_domain_from_xml(&xml) {
            Ok(domain) => {
                if let Err(e) = domain.create() {
                    error!("Error starting VM: {}", e);
                    return Err(anyhow!("Failed to start VM: {}", e));
                }
                self.domain = Some(domain);
                info!("VM successfully defined and started: {}", self.config.name);
                Ok(())
            },
            Err(e) => {
                error!("Error defining VM: {}", e);
                Err(anyhow!("Failed to define VM: {}", e))
            },
        }
    }

    pub fn destroy(&mut self) -> Result<()> {
        match &self.domain {
            Some(domain) => {
                if let Err(e) = domain.destroy() {
                    error!("Error destroying VM: {}", e);
                    return Err(anyhow!("Failed to destroy VM: {}", e));
                }
                info!("VM successfully destroyed: {}", self.config.name);
                Ok(())
            },
            None => {
                warn!("VM is not running: {}", self.config.name);
                Err(anyhow!("VM is not running"))
            },
        }
    }

    fn generate_domain_xml(&self) -> Result<String> {
        let memory_kb = self.config.ram * 1024; // Convert MB to KB
        let xml = format!(r#"
<domain type='kvm'>
  <name>{name}</name>
  <memory unit='KiB'>{memory_kb}</memory>
  <vcpu placement='static'>1</vcpu>
  <os>
    <type arch='{arch}' machine='{machine_type}'>{os_type}</type>
    <boot dev='hd'/>
  </os>
  <devices>
    <disk type='file' device='disk'>
      <driver name='qemu' type='qcow2'/>
      <source file='{disk_image_path}'/>
      <target dev='vda' bus='virtio'/>
    </disk>
    <interface type='network'>
      <source network='default'/>
      <model type='virtio'/>
    </interface>
    {graphics}
  </devices>
</domain>
"#, name=self.config.name, memory_kb=memory_kb, arch=self.config.arch, machine_type=self.config.machine_type, os_type=self.config.os_type, disk_image_path=self.config.disk_image_path, graphics=self.generate_graphics_xml());
        Ok(xml)
    }

    fn generate_graphics_xml(&self) -> String {
        if self.config.spice {
            "<graphics type='spice' port='-1' autoport='yes'/>".to_string()
        } else if self.config.vnc {
            "<graphics type='vnc' port='-1' autoport='yes'/>".to_string()
        } else {
            "".to_string()
        }
    }

    #[napi]
    pub fn set_ram(&mut self, ram: u64) -> NapiResult<()> {
        info!("Setting RAM to {} MB", ram);
        if let Some(domain) = &self.domain {
            if let Err(e) = domain.set_memory(ram * 1024) {
                error!("Error setting RAM: {}", e);
                return Err(napi::Error::from_reason(format!("Failed to set RAM: {}", e)));
            }
        } else {
            warn!("VM is not running: {}", self.config.name);
            return Err(napi::Error::from_reason("VM is not running".to_string()));
        }
        self.config.ram = ram;
        Ok(())
    }

    #[napi]
    pub fn set_cpus(&mut self, cpus: u32) -> NapiResult<()> {
        info!("Setting CPUs to {}", cpus);
        if let Some(domain) = &self.domain {
            if let Err(e) = domain.set_vcpus(cpus) {
                error!("Error setting CPUs: {}", e);
                return Err(napi::Error::from_reason(format!("Failed to set CPUs: {}", e)));
            }
        } else {
            warn!("VM is not running: {}", self.config.name);
            return Err(napi::Error::from_reason("VM is not running".to_string()));
        }
        Ok(())
    }

    #[napi]
    pub fn add_cdrom(&mut self, source_file: String) -> NapiResult<()> {
        info!("Adding CDROM from {}", source_file);
        if let Some(_domain) = &self.domain {
            // Implementation to add a CDROM to the VM
            // This is a placeholder, actual implementation will depend on the virt crate's capabilities
            // _domain.attach_device(...);
        } else {
            warn!("VM is not running: {}", self.config.name);
            return Err(napi::Error::from_reason("VM is not running".to_string()));
        }
        Ok(())
    }

    #[napi]
    pub fn remove_cdrom(&mut self) -> NapiResult<()> {
        info!("Removing CDROM");
        if let Some(_domain) = &self.domain {
            // Implementation to remove the CDROM from the VM
            // This is a placeholder, actual implementation will depend on the virt crate's capabilities
            // _domain.detach_device(...);
        } else {
            warn!("VM is not running: {}", self.config.name);
            return Err(napi::Error::from_reason("VM is not running".to_string()));
        }
        Ok(())
    }

    #[napi]
    pub fn add_storage(&mut self, size: String, path: String) -> NapiResult<()> {
        info!("Adding storage of size {} at {}", size, path);
        if let Some(_domain) = &self.domain {
            // Implementation to add extra storage to the VM
            // This is a placeholder, actual implementation will depend on the virt crate's capabilities
            // _domain.attach_device(...);
        } else {
            warn!("VM is not running: {}", self.config.name);
            return Err(napi::Error::from_reason("VM is not running".to_string()));
        }
        Ok(())
    }

    #[napi]
    pub fn remove_storage(&mut self, path: String) -> NapiResult<()> {
        info!("Removing storage at {}", path);
        if let Some(_domain) = &self.domain {
            // Implementation to remove storage from the VM
            // This is a placeholder, actual implementation will depend on the virt crate's capabilities
            // _domain.detach_device(...);
        } else {
            warn!("VM is not running: {}", self.config.name);
            return Err(napi::Error::from_reason("VM is not running".to_string()));
        }
        Ok(())
    }

    #[napi]
    pub fn power_on(&mut self) -> NapiResult<()> {
        info!("Powering on VM");
        if let Some(domain) = &self.domain {
            if let Err(e) = domain.create() {
                error!("Error powering on VM: {}", e);
                return Err(napi::Error::from_reason(format!("Failed to power on VM: {}", e)));
            }
        } else {
            warn!("VM is not running: {}", self.config.name);
            return Err(napi::Error::from_reason("VM is not running".to_string()));
        }
        Ok(())
    }

    #[napi]
    pub fn power_off(&mut self, acpi: bool) -> NapiResult<()> {
        info!("Powering off VM with ACPI={}", acpi);
        if let Some(domain) = &self.domain {
            if acpi {
                if let Err(e) = domain.shutdown() {
                    error!("Error shutting down VM with ACPI: {}", e);
                    return Err(napi::Error::from_reason(format!("Failed to shut down VM with ACPI: {}", e)));
                }
            } else {
                if let Err(e) = domain.destroy() {
                    error!("Error forcing off VM: {}", e);
                    return Err(napi::Error::from_reason(format!("Failed to force off VM: {}", e)));
                }
            }
        } else {
            warn!("VM is not running: {}", self.config.name);
            return Err(napi::Error::from_reason("VM is not running".to_string()));
        }
        Ok(())
    }

    #[napi]
    pub fn set_boot_order(&mut self, boot_order: Vec<String>) -> NapiResult<()> {
        info!("Setting boot order to {:?}", boot_order);
        if let Some(_domain) = &self.domain {
            // Implementation to set the VM's boot order
            // This is a placeholder, actual implementation will depend on the virt crate's capabilities
            // _domain.set_boot_order(...);
        } else {
            warn!("VM is not running: {}", self.config.name);
            return Err(napi::Error::from_reason("VM is not running".to_string()));
        }
        Ok(())
    }
}

impl fmt::Debug for Machine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Machine")
         .field("config", &self.config)
         .field("domain", &self.domain)
         .finish()
    }
}