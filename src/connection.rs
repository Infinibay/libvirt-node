use log::{warn, info, error};
use virt::{connect::Connect, domain::Domain};
use napi::{Env, JsObject, Result as NapiResult, JsString, JsUndefined, CallContext, Property};
use napi_derive::js_function;
use napi_derive::napi;

#[derive(Debug)]
pub enum ConnectionError {
    ConnectionFailure(String),
    DisconnectionFailure(String),
    DomainDefineError(String),
    DomainListError(String),
    DisconnectedInstanceError,
}

pub struct Connection {
    conn: Option<Connect>,
}

impl Connection {
   pub fn connect(&mut self, uri: String) -> Result<(), ConnectionError> {
        if self.conn.is_some() {
            warn!("Connection already exists.");
            return Ok(());
        }

        match Connect::open(&uri) {
            Ok(conn) => {
                self.conn = Some(conn);
                info!("Connection established to {}", uri);
                Ok(())
            },
            Err(e) => {
                error!("Failed to establish a connection to {}: {}", uri, e);
                Err(ConnectionError::ConnectionFailure(e.to_string()))
            }
        }
    }

    pub fn disconnect(&mut self) -> Result<(), ConnectionError> {
        if let Some(mut conn) = self.conn.take() {
            match conn.close() {
                Ok(_) => {
                    info!("Connection successfully closed.");
                    Ok(())
                },
                Err(e) => {
                    error!("Failed to close the connection: {}", e);
                    Err(ConnectionError::DisconnectionFailure(e.to_string()))
                }
            }
        } else {
            warn!("Attempted to disconnect a non-existent connection.");
            Ok(())
        }
    }

    pub fn define_domain_from_xml(&self, xml_desc: String) -> Result<Domain, ConnectionError> {
        match &self.conn {
            Some(conn) => {
                match Domain::define_xml(&conn, &xml_desc) {
                    Ok(domain) => Ok(domain),
                    Err(e) => {
                        error!("Failed to define domain from XML: {}", e);
                        Err(ConnectionError::DomainDefineError(e.to_string()))
                    }
                }
            },
            None => {
                warn!("Attempted to define a domain without an active connection.");
                Err(ConnectionError::DisconnectedInstanceError)
            }
        }
    }

    pub fn list_machines(&self) -> Result<Vec<Domain>, ConnectionError> {
        match &self.conn {
            Some(conn) => {
                match conn.list_all_domains(0) {
                    Ok(domains) => Ok(domains),
                    Err(e) => {
                        error!("Failed to list domains: {}", e);
                        Err(ConnectionError::DomainListError(e.to_string()))
                    }
                }
            },
            None => {
                warn!("Attempted to list domains without an active connection.");
                Err(ConnectionError::DisconnectedInstanceError)
            }
        }
    }
}
