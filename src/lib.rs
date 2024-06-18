mod connection;
mod machine;

pub use connection::{Connection, ConnectionError};
pub use machine::{Machine, VmConfig};

use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
pub struct Libvirt {
    connection: Connection,
}

#[napi]
impl Libvirt {
    #[napi(constructor)]
    pub fn new(connection_string: Option<String>) -> napi::Result<Self> {
        let connection = match connection_string {
            Some(conn_str) => Connection::new(Some(conn_str)).map_err(|e| {
                napi::Error::from_reason(format!("Failed to create connection: {:?}", e))
            })?,
            None => Connection::new(Some("qemu:///system".to_string())).map_err(|e| {
                napi::Error::from_reason(format!("Failed to create default connection: {:?}", e))
            })?,
        };
        Ok(Libvirt { connection })
    }

    #[napi]
    pub fn connect(&mut self, connection_string: String) -> napi::Result<()> {
        self.connection.connect(connection_string).map_err(|e| {
            napi::Error::from_reason(format!("Failed to connect: {:?}", e))
        })
    }

    #[napi]
    pub fn disconnect(&mut self) -> napi::Result<()> {
        self.connection.disconnect().map_err(|e| {
            napi::Error::from_reason(format!("Failed to disconnect: {:?}", e))
        })
    }

    #[napi]
    pub fn find_machine(&self, name: String) -> napi::Result<Machine> {
        self.connection.find_machine(name).map_err(|e| {
            napi::Error::from_reason(format!("Failed to find machine: {:?}", e))
        }).and_then(|domain| {
            Machine::fromDomain(domain).map_err(|e| {
                napi::Error::from_reason(format!("Failed to convert domain to machine: {:?}", e))
            })
        })
    }

    #[napi]
    pub fn list_machines(&self) -> napi::Result<Vec<Machine>> {
        self.connection.list_machines().map_err(|e| {
            napi::Error::from_reason(format!("Failed to list machines: {:?}", e))
        }).and_then(|domains| {
            domains.into_iter().map(|domain| {
                Machine::fromDomain(domain).map_err(|e| {
                    napi::Error::from_reason(format!("Failed to convert domain to machine: {:?}", e))
                })
            }).collect::<napi::Result<Vec<_>>>()
        })
    }

    #[napi]
    pub fn create_machine(&self, config: VmConfig) -> napi::Result<Machine> {
        let mut machine = Machine::new(config);
        machine.deploy(&self.connection).map_err(|e| {
            napi::Error::from_reason(format!("Failed to create machine: {:?}", e))
        })?;
        Ok(machine)
    }

    #[napi]
    pub fn destroy_machine(&self, name: String) -> napi::Result<()> {
        let mut machine = self.find_machine(name)?;
        machine.destroy().map_err(|e| {
            napi::Error::from_reason(format!("Failed to destroy machine: {:?}", e))
        })
    }
}
