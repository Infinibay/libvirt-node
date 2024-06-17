mod connection;
mod machine; // Include the machine module

pub use connection::{Connection, ConnectionError};
pub use machine::{Machine, VmConfig}; // Export the Machine and VmConfig

use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
pub struct Libvirt {
    connection: Connection,
}

#[napi]
impl Libvirt {
    #[napi(constructor)]
    pub fn new(connection_string: Option<String>) -> Result<Self> {
        let connection = match connection_string {
            Some(conn_str) => Connection::new(Some(&conn_str)).map_err(|e| {
                napi::Error::from_reason(format!("Failed to create connection: {:?}", e))
            })?,
            None => Connection::new(Some("qemu:///system")).map_err(|e| {
                napi::Error::from_reason(format!("Failed to create default connection: {:?}", e))
            })?,
        };
        Ok(Libvirt { connection })
    }

    #[napi]
    pub fn connect(&mut self, connection_string: String) -> Result<()> {
        self.connection.connect(&connection_string).map_err(|e| {
            napi::Error::from_reason(format!("Failed to connect: {:?}", e))
        })
    }

    #[napi]
    pub fn disconnect(&mut self) -> Result<()> {
        self.connection.disconnect().map_err(|e| {
            napi::Error::from_reason(format!("Failed to disconnect: {:?}", e))
        })
    }

    #[napi]
    pub fn find_machine(&self, name: String) -> Result<Machine> {
        self.connection.find_machine(&name).map_err(|e| {
            napi::Error::from_reason(format!("Failed to find machine: {:?}", e))
        }).and_then(|domain| {
            Machine::from_domain(domain).map_err(|e| {
                napi::Error::from_reason(format!("Failed to convert domain to machine: {:?}", e))
            })
        })
    }

    #[napi]
    pub fn list_machines(&self) -> Result<Vec<Machine>> {
        self.connection.list_machines().map_err(|e| {
            napi::Error::from_reason(format!("Failed to list machines: {:?}", e))
        }).and_then(|domains| {
            domains.into_iter().map(|domain| {
                Machine::from_domain(domain).map_err(|e| {
                    napi::Error::from_reason(format!("Failed to convert domain to machine: {:?}", e))
                })
            }).collect::<Result<Vec<_>, _>>()
        })
    }

    #[napi]
    pub fn create_machine(&self, config: VmConfig) -> Result<Machine> {
        let mut machine = Machine::new(config);
        machine.deploy(&self.connection).map_err(|e| {
            napi::Error::from_reason(format!("Failed to create machine: {:?}", e))
        })?;
        Ok(machine)
    }

    #[napi]
    pub fn destroy_machine(&self, name: String) -> Result<()> {
        let mut machine = self.find_machine(name)?;
        machine.destroy().map_err(|e| {
            napi::Error::from_reason(format!("Failed to destroy machine: {:?}", e))
        })
    }
}