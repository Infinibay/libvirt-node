use crate::Connection;
use anyhow::{anyhow, Result};
use log::{error, info, warn};
use napi::{Env, JsNumber, JsObject, Result as NapiResult, JsUndefined, JsBoolean, JsString, CallContext, Property};
use napi_derive::js_function;
use napi_derive::napi;
use serde::{Deserialize, Serialize};
use std::fmt;
use virt::domain::Domain;

#[derive(Serialize, Deserialize, Debug)]
#[napi(object)]
pub struct VmConfig {
  #[napi(js_name = "name")]
  pub name: String,
  #[napi(js_name = "ram")]
  pub ram: i64, // in MB, changed from i64 to i64
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

pub struct Machine {
  config: VmConfig,
  domain: Option<Domain>,
}

impl Machine {
  pub fn new(config: VmConfig) -> Self {
    Self {
      config,
      domain: None,
    }
  }

  // Internal Method
  pub fn fromDomain(domain: Domain) -> napi::Result<Self> {
    let config = VmConfig {
      name: domain
        .get_name()
        .map_err(|e| napi::Error::from_reason(format!("Failed to get domain name: {:?}", e)))?,
      ram: domain
        .get_max_memory()
        .map_err(|e| napi::Error::from_reason(format!("Failed to get max memory: {:?}", e)))?
        as i64,
      disk: String::new(),    // Placeholder, actual disk info extraction needed
      tpm: false,             // Placeholder, actual tpm info extraction needed
      spice: false,           // Placeholder, actual spice info extraction needed
      vnc: false,             // Placeholder, actual vnc info extraction needed
      os_type: String::new(), // Placeholder, actual os_type extraction needed
      arch: String::new(),    // Placeholder, actual arch extraction needed
      machine_type: String::new(), // Placeholder, actual machine_type extraction needed
      disk_image_path: String::new(), // Placeholder, actual disk_image_path extraction needed
    };
    Ok(Self {
      config,
      domain: Some(domain),
    })
  }

  pub fn deploy(&mut self, xml: String, conn: &Connection) -> napi::Result<()> {
    match conn.define_domain_from_xml_internal(xml) {
      Ok(domain) => {
        if let Err(e) = domain.create() {
          error!("Error starting VM: {}", e);
          return Err(napi::Error::from_reason(format!(
            "Failed to start VM: {:?}",
            e
          )));
        }
        self.domain = Some(domain);
        info!("VM successfully defined and started: {}", self.config.name);
        Ok(())
      }
      Err(e) => {
        error!("Error defining VM: {}", e);
        Err(napi::Error::from_reason(format!(
          "Failed to define VM: {:?}",
          e
        )))
      }
    }
  }

  pub fn destroy(&mut self) -> napi::Result<()> {
    match &self.domain {
      Some(domain) => {
        if let Err(e) = domain.destroy() {
          error!("Error destroying VM: {}", e);
          return Err(napi::Error::from_reason(format!(
            "Failed to destroy VM: {:?}",
            e
          )));
        }
        info!("VM successfully destroyed: {}", self.config.name);
        Ok(())
      }
      None => {
        warn!("VM is not running: {}", self.config.name);
        Err(napi::Error::from_reason("VM is not running".to_string()))
      }
    }
  }

  pub fn power_on(&mut self) -> napi::Result<()> {
    info!("Powering on VM");
    if let Some(domain) = &self.domain {
      if let Err(e) = domain.create() {
        error!("Error powering on VM: {}", e);
        return Err(napi::Error::from_reason(format!(
          "Failed to power on VM: {:?}",
          e
        )));
      }
    } else {
      warn!("VM is not running: {}", self.config.name);
      return Err(napi::Error::from_reason("VM is not running".to_string()));
    }
    Ok(())
  }

  pub fn power_off(&mut self, acpi: bool) -> napi::Result<()> {
    info!("Powering off VM with ACPI={}", acpi);
    if let Some(domain) = &self.domain {
      if acpi {
        if let Err(e) = domain.shutdown() {
          error!("Error shutting down VM with ACPI: {}", e);
          return Err(napi::Error::from_reason(format!(
            "Failed to shut down VM with ACPI: {:?}",
            e
          )));
        }
      } else {
        if let Err(e) = domain.destroy() {
          error!("Error forcing off VM: {}", e);
          return Err(napi::Error::from_reason(format!(
            "Failed to force off VM: {:?}",
            e
          )));
        }
      }
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

#[js_function(1)] // Constructor takes 1 argument: VmConfig object
fn constructor(ctx: CallContext) -> NapiResult<JsUndefined> {
  let config: VmConfig = ctx.get::<JsObject>(0)?.into_serde()?;
  let machine = Machine {
    config,
    domain: None,
  };

  let mut this: JsObject = ctx.this_unchecked();
  ctx.env.wrap(&mut this, machine)?;

  ctx.env.get_undefined()
}

#[js_function(2)]
pub fn machine_deploy(ctx: napi::CallContext) -> napi::Result<JsObject> {
  let machine: &mut Machine = ctx.env.unwrap(ctx.this_unchecked())?;
  let obj = ctx.get::<JsString>(0)?;
  if obj.is_err() {
    return Err(napi::Error::new(
      napi::Status::InvalidArg,
      "The first argument must be the XML string to deploy".to_string(),
    ));
  }
  let xml = obj?.into_utf8()?.as_str()?.to_string();
  let arg = ctx.get::<JsObject>(1)?;

  // Determine if the argument is a string or an object
  let connection = match arg.get_type()? {
    ValueType::String => {
      let conn_str = arg.cast::<JsString>().into_utf8()?.as_str()?.to_string();
      crate::Connection::new(conn_str)?;
    }
    ValueType::Object => {
      if let Ok(connection) = ctx.env.unwrap::<crate::Connection>(&arg) {
        connection
      } else {
        // If not, throw a custom JavaScript error
        let error_message = "Expected a Connection object or connection string".to_string();
        return Err(NapiError::new(Status::InvalidArg, error_message));
      }
    }
    _ => {
      return Err(napi::Error::new(
        napi::Status::InvalidArg,
        "Expected a connection object or string".to_string(),
      ))
    }
  };

  machine.deploy(xml, &connection)?;
  ctx.env.get_undefined()
}

#[js_function(0)]
pub fn machine_destroy(ctx: napi::CallContext) -> napi::Result<JsUndefined> {
  let machine: &mut Machine = ctx.env.unwrap(ctx.this_unchecked());
  machine.destroy()?;
  ctx.env.get_undefined()
}

#[js_function(0)]
pub fn machine_power_on(ctx: napi::CallContext) -> napi::Result<JsUndefined> {
  let machine: &mut Machine = ctx.env.unwrap(ctx.this_unchecked());
  machine.power_on()?;
  ctx.env.get_undefined()
}

#[js_function(1)]
pub fn machine_power_off(ctx: napi::CallContext) -> napi::Result<JsUndefined> {
  let machine: &mut Machine = ctx.env.unwrap(ctx.this_unchecked());
  let acpi = ctx.get::<JsBoolean>(0)?.get_value()?;
  machine.power_off(acpi)?;
  ctx.env.get_undefined()
}

pub fn register_js(exports: &mut JsObject, env: Env) -> NapiResult<()> {
  let connection_class = env.define_class(
    "Machine",
    machine_constructor,
    &[
      Property::new("deploy")?.with_method(machine_deploy),
      Property::new("destroy")?.with_method(machine_destroy),
      Property::new("powerOn")?.with_method(machine_power_on),
      Property::new("powerOff")?.with_method(machine_power_off),
    ],
  )?;
  exports.set_named_property("Connection", connection_class)?;
  Ok(())
}
