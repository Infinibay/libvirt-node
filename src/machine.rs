use crate::Connection;
use log::{error, info, warn};
use napi::{CallContext, Env, JsObject, JsString, JsUndefined, JsBoolean, Status, Result as NapiResult, Error as NapiError, Property};

use napi_derive::js_function;
use napi_derive::napi;
use serde::{Deserialize, Serialize};
use std::fmt;
use virt::domain::Domain;
use serde_json::Value;
use napi::bindgen_prelude::*;


#[derive(Serialize, Deserialize, Debug)]
pub struct VmConfig {
  pub name: String,
  pub ram: i64, // in MB, changed from i64 to i64
  pub disk: String, // disk size as String, e.g., "10G"
  pub tpm: bool,
  pub spice: bool,
  pub vnc: bool,
  pub os_type: String,
  pub arch: String, // Architecture, e.g., "x86_64"
  pub machine_type: String, // Machine type, e.g., "pc-i440fx-2.9"
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
        error!("Error defining VM: {:?}", e);
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
