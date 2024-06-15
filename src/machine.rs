use napi_derive::napi;
use virt::{Connect, Domain, DomainGraphicsType, DomainModify, DomainXmlFlags};
use std::sync::Mutex;
use xml::reader::{EventReader, XmlEvent};

#[napi]
pub struct Machine {
    domain: Mutex<Option<Domain>>,
}

#[napi]
impl Machine {
    #[napi(constructor)]
    pub fn new(conn: &Connect, domain_name: String) -> Result<Self, napi::Error> {
        match conn.lookup_domain_by_name(&domain_name) {
            Ok(domain) => Ok(Self {
                domain: Mutex::new(Some(domain)),
            }),
            Err(e) => Err(napi::Error::new(napi::Status::GenericFailure, format!("Failed to find domain: {}", e))),
        }
    }

    #[napi]
    pub fn start(&self) -> Result<(), napi::Error> {
        let mut domain = self.domain.lock().unwrap();
        if let Some(ref mut dom) = *domain {
            dom.create().map_err(|e| napi::Error::new(napi::Status::GenericFailure, format!("Failed to start domain: {}", e)))
        } else {
            Err(napi::Error::new(napi::Status::InvalidArg, "Domain not initialized".to_string()))
        }
    }

    #[napi]
    pub fn stop(&self) -> Result<(), napi::Error> {
        let mut domain = self.domain.lock().unwrap();
        if let Some(ref mut dom) = *domain {
            dom.shutdown().map_err(|e| napi::Error::new(napi::Status::GenericFailure, format!("Failed to stop domain: {}", e)))
        } else {
            Err(napi::Error::new(napi::Status::InvalidArg, "Domain not initialized".to_string()))
        }
    }

    #[napi]
    pub fn suspend(&self) -> Result<(), napi::Error> {
        let mut domain = self.domain.lock().unwrap();
        if let Some(ref mut dom) = *domain {
            dom.suspend().map_err(|e| napi::Error::new(napi::Status::GenericFailure, format!("Failed to suspend domain: {}", e)))
        } else {
            Err(napi::Error::new(napi::Status::InvalidArg, "Domain not initialized".to_string()))
        }
    }

    #[napi]
    pub fn resume(&self) -> Result<(), napi::Error> {
        let mut domain = self.domain.lock().unwrap();
        if let Some(ref mut dom) = *domain {
            dom.resume().map_err(|e| napi::Error::new(napi::Status::GenericFailure, format!("Failed to resume domain: {}", e)))
        } else {
            Err(napi::Error::new(napi::Status::InvalidArg, "Domain not initialized".to_string()))
        }
    }

    #[napi]
    pub fn set_vnc(&self, password: String) -> Result<(), napi::Error> {
        let xml_desc = format!("<graphics type='vnc' passwd='{}' autoport='yes'/>", password);
        let mut domain = self.domain.lock().unwrap();
        if let Some(ref mut dom) = *domain {
            dom.attach_device(&xml_desc)
                .map_err(|e| napi::Error::new(napi::Status::GenericFailure, format!("Failed to set VNC: {}", e)))
        } else {
            Err(napi::Error::new(napi::Status::InvalidArg, "Domain not initialized".to_string()))
        }
    }

    #[napi]
    pub fn set_spice(&self, password: String) -> Result<(), napi::Error> {
        let xml_desc = format!("<graphics type='spice' passwd='{}' autoport='yes'/>", password);
        let mut domain = self.domain.lock().unwrap();
        if let Some(ref mut dom) = *domain {
            dom.attach_device(&xml_desc)
                .map_err(|e| napi::Error::new(napi::Status::GenericFailure, format!("Failed to set SPICE: {}", e)))
        } else {
            Err(napi::Error::new(napi::Status::InvalidArg, "Domain not initialized".to_string()))
        }
    }

    #[napi]
    pub fn get_vnc_port(&self) -> Result<u32, napi::Error> {
        let domain = self.domain.lock().unwrap();
        if let Some(ref dom) = *domain {
            let xml_desc = dom.get_xml_desc(DomainXmlFlags::empty())
                .map_err(|e| napi::Error::new(napi::Status::GenericFailure, format!("Failed to get domain XML: {}", e)))?;
            parse_graphics_port(&xml_desc, "vnc")
        } else {
            Err(napi::Error::new(napi::Status::InvalidArg, "Domain not initialized".to_string()))
        }
    }

    #[napi]
    pub fn get_spice_port(&self) -> Result<u32, napi::Error> {
        let domain = self.domain.lock().unwrap();
        if let Some(ref dom) = *domain {
            let xml_desc = dom.get_xml_desc(DomainXmlFlags::empty())
                .map_err(|e| napi::Error::new(napi::Status::GenericFailure, format!("Failed to get domain XML: {}", e)))?;
            parse_graphics_port(&xml_desc, "spice")
        } else {
            Err(napi::Error::new(napi::Status::InvalidArg, "Domain not initialized".to_string()))
        }
    }

    fn parse_graphics_port(xml_desc: &str, graphics_type: &str) -> Result<u32, napi::Error> {
        let parser = EventReader::from_str(xml_desc);
        let mut in_graphics = false;
        for e in parser {
            match e {
                Ok(XmlEvent::StartElement { name, attributes, .. }) if name.local_name == "graphics" => {
                    let type_attr = attributes.iter().find(|a| a.name.local_name == "type").map(|a| a.value.as_str());
                    if type_attr == Some(graphics_type) {
                        in_graphics = true;
                    }
                },
                Ok(XmlEvent::EndElement { name }) if name.local_name == "graphics" => {
                    in_graphics = false;
                },
                Ok(XmlEvent::Attribute { name, value }) if in_graphics && name.local_name == "port" => {
                    return value.parse::<u32>().map_err(|_| napi::Error::new(napi::Status::GenericFailure, "Failed to parse port number".to_string()));
                },
                _ => continue,
            }
        }
        Err(napi::Error::new(napi::Status::GenericFailure, "Port not found".to_string()))
    }

    #[napi]
    pub fn set_boot_order(&self, boot_devices: Vec<String>) -> Result<(), napi::Error> {
        let mut domain = self.domain.lock().unwrap();
        if let Some(ref mut dom) = *domain {
            dom.set_boot_order(&boot_devices).map_err(|e| napi::Error::new(napi::Status::GenericFailure, format!("Failed to set boot order: {}", e)))
        } else {
            Err(napi::Error::new(napi::Status::InvalidArg, "Domain not initialized".to_string()))
        }
    }

    #[napi]
    pub fn attach_iso(&self, iso_path: String) -> Result<(), napi::Error> {
        let mut domain = self.domain.lock().unwrap();
        if let Some(ref mut dom) = *domain {
            dom.attach_disk(&iso_path, "cdrom", true).map_err(|e| napi::Error::new(napi::Status::GenericFailure, format!("Failed to attach ISO: {}", e)))
        } else {
            Err(napi::Error::new(napi::Status::InvalidArg, "Domain not initialized".to_string()))
        }
    }

    #[napi]
    pub fn detach_iso(&self, iso_path: String) -> Result<(), napi::Error> {
        let mut domain = self.domain.lock().unwrap();
        if let Some(ref mut dom) = *domain {
            dom.detach_disk(&iso_path).map_err(|e| napi::Error::new(napi::Status::GenericFailure, format!("Failed to detach ISO: {}", e)))
        } else {
            Err(napi::Error::new(napi::Status::InvalidArg, "Domain not initialized".to_string()))
        }
    }

    #[napi]
    pub fn add_tpm2(&self, tpm_model: String, tpm_type: String) -> Result<(), napi::Error> {
        let mut domain = self.domain.lock().unwrap();
        if let Some(ref mut dom) = *domain {
            let xml_desc = format!("<tpm model='{}'><backend type='{}'/></tpm>", tpm_model, tpm_type);
            dom.attach_device(&xml_desc)
                .map_err(|e| napi::Error::new(napi::Status::GenericFailure, format!("Failed to add TPM2: {}", e)))
        } else {
            Err(napi::Error::new(napi::Status::InvalidArg, "Domain not initialized".to_string()))
        }
    }

    #[napi]
    pub fn set_memory(&self, memory_kb: u64) -> Result<(), napi::Error> {
        let mut domain = self.domain.lock().unwrap();
        if let Some(ref mut dom) = *domain {
            dom.set_memory(memory_kb * 1024)  // Adjusted to convert KB to bytes as expected by the API
                .map_err(|e| napi::Error::new(napi::Status::GenericFailure, format!("Failed to set memory: {}", e)))
        } else {
            Err(napi::Error::new(napi::Status::InvalidArg, "Domain not initialized".to_string()))
        }
    }

    #[napi]
    pub fn set_vcpus(&self, vcpus: u32) -> Result<(), napi::Error> {
        let mut domain = self.domain.lock().unwrap();
        if let Some(ref mut dom) = *domain {
            dom.set_vcpus(vcpus)
                .map_err(|e| napi::Error::new(napi::Status::GenericFailure, format!("Failed to set vCPUs: {}", e)))
        } else {
            Err(napi::Error::new(napi::Status::InvalidArg, "Domain not initialized".to_string()))
        }
    }

    #[napi]
    pub fn set_disk_size(&self, disk_path: String, size_gb: u64) -> Result<(), napi::Error> {
        let mut domain = self.domain.lock().unwrap();
        if let Some(ref mut dom) = *domain {
            dom.resize_disk(&disk_path, size_gb * 1024 * 1024 * 1024, true)
                .map_err(|e| napi::Error::new(napi::Status::GenericFailure, format!("Failed to resize disk: {}", e)))
        } else {
            Err(napi::Error::new(napi::Status::InvalidArg, "Domain not initialized".to_string()))
        }
    }

    #[napi]
    pub fn get_hardware_info(&self) -> Result<String, napi::Error> {
        let domain = self.domain.lock().unwrap();
        if let Some(ref dom) = *domain {
            dom.get_xml_desc(DomainXmlFlags::empty())
                .map_err(|e| napi::Error::new(napi::Status::GenericFailure, format!("Failed to get hardware info: {}", e)))
        } else {
            Err(napi::Error::new(napi::Status::InvalidArg, "Domain not initialized".to_string()))
        }
    }

    #[napi]
    pub fn set_raw_xml(&self, xml_desc: String) -> Result<(), napi::Error> {
        let mut domain = self.domain.lock().unwrap();
        if let Some(ref mut dom) = *domain {
            dom.define_xml(&xml_desc)
                .map_err(|e| napi::Error::new(napi::Status::GenericFailure, format!("Failed to set raw XML: {}", e)))
        } else {
            Err(napi::Error::new(napi::Status::InvalidArg, "Domain not initialized".to_string()))
        }
    }

    #[napi]
    pub fn get_raw_xml(&self) -> Result<String, napi::Error> {
        let domain = self.domain.lock().unwrap();
        if let Some(ref dom) = *domain {
            dom.get_xml_desc(virt::DomainXmlFlags::empty())
                .map_err(|e| napi::Error::new(napi::Status::GenericFailure, format!("Failed to get raw XML: {}", e)))
        } else {
            Err(napi::Error::new(napi::Status::InvalidArg, "Domain not initialized".to_string()))
        }
    }
}
