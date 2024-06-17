use virt::{connect::Connect, domain::Domain};
use anyhow::{Result, anyhow};
use thiserror::Error;
use log::{info, error, warn};
use napi_derive::napi;
use napi::{ToNapiValue, FromNapiValue, FromNapiRef, Env, JsUnknown};

#[derive(Debug, Error)]
pub enum ConnectionError {
    #[error("Failed to establish a connection: {0}")]
    ConnectionFailure(String),
    #[error("Failed to disconnect: {0}")]
    DisconnectionFailure(String),
    #[error("Failed to define a domain from XML: {0}")]
    DomainDefineError(String),
    #[error("Operation attempted on a disconnected instance")]
    DisconnectedInstanceError,
    #[error("Failed to find domain: {0}")]
    DomainNotFoundError(String),
    #[error("Failed to list domains: {0}")]
    DomainListError(String),
}

#[napi]
pub struct Connection {
    pub conn: Option<Connect>, // Change visibility to pub
}

impl Clone for Connect {
    fn clone(&self) -> Self {
        Connect::open(self.get_uri().unwrap_or("")).unwrap()
    }
}

#[napi]
impl Connection {
    #[napi(constructor)]
    pub fn new(uri: Option<&str>) -> napi::Result<Self> {
        Self::new_internal(uri).map_err(|e| napi::Error::from_reason(format!("Failed to establish a connection: {:?}", e)))
    }

    fn new_internal(uri: Option<&str>) -> Result<Self> {
        match Connect::open(uri.unwrap_or("")) {
            Ok(conn) => {
                info!("Connection established to {}", uri.unwrap_or("default URI"));
                Ok(Self { conn: Some(conn) })
            },
            Err(e) => {
                error!("Failed to establish a connection: {}", e);
                Err(anyhow!(ConnectionError::ConnectionFailure(e.to_string())))
            }
        }
    }

    #[napi]
    pub fn connect(&mut self, uri: &str) -> napi::Result<()> {
        self.connect_internal(uri).map_err(|e| napi::Error::from_reason(format!("Failed to connect: {:?}", e)))
    }

    fn connect_internal(&mut self, uri: &str) -> Result<(), ConnectionError> {
        if self.conn.is_some() {
            warn!("Connection already exists.");
            return Ok(());
        }

        match Connect::open(uri) {
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

    #[napi]
    pub fn disconnect(&mut self) -> napi::Result<()> {
        self.disconnect_internal().map_err(|e| napi::Error::from_reason(format!("Failed to disconnect: {:?}", e)))
    }

    fn disconnect_internal(&mut self) -> Result<(), ConnectionError> {
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

    #[napi]
    pub fn define_domain_from_xml(&self, xml_desc: &str) -> napi::Result<Domain> {
        self.define_domain_from_xml_internal(xml_desc).map_err(|e| napi::Error::from_reason(format!("Failed to define domain from XML: {:?}", e)))
    }

    fn define_domain_from_xml_internal(&self, xml_desc: &str) -> Result<Domain, ConnectionError> {
        match &self.conn {
            Some(conn) => {
                match Domain::define_xml(&conn, xml_desc) {
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

    #[napi]
    pub fn find_machine(&self, name: &str) -> napi::Result<Domain> {
        self.find_machine_internal(name).map_err(|e| napi::Error::from_reason(format!("Failed to find domain: {:?}", e)))
    }

    fn find_machine_internal(&self, name: &str) -> Result<Domain, ConnectionError> {
        match &self.conn {
            Some(conn) => {
                match Domain::lookup_by_name(&conn, name) {
                    Ok(domain) => Ok(domain),
                    Err(e) => {
                        error!("Failed to find domain by name {}: {}", name, e);
                        Err(ConnectionError::DomainNotFoundError(e.to_string()))
                    }
                }
            },
            None => {
                warn!("Attempted to find a domain without an active connection.");
                Err(ConnectionError::DisconnectedInstanceError)
            }
        }
    }

    #[napi]
    pub fn list_machines(&self) -> napi::Result<Vec<Domain>> {
        self.list_machines_internal().map_err(|e| napi::Error::from_reason(format!("Failed to list domains: {:?}", e)))
    }

    fn list_machines_internal(&self) -> Result<Vec<Domain>, ConnectionError> {
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

    pub fn get_conn(&self) -> Option<&Connect> {
        self.conn.as_ref()
    }
}

// Implement ToNapiValue and FromNapiValue for Option<Connect>
impl napi::bindgen_prelude::ToNapiValue for Option<Connect> {
    unsafe fn to_napi_value(env: napi::sys::napi_env, val: Self) -> napi::Result<napi::sys::napi_value> {
        match val {
            Some(conn) => napi::JsString::from_str(env, &conn.get_uri().unwrap_or("")).map(|js_str| js_str.raw()),
            None => napi::JsObject::new(env).map(|js_obj| js_obj.raw()),
        }
    }
}

impl napi::bindgen_prelude::FromNapiValue for Option<Connect> {
    unsafe fn from_napi_value(env: napi::sys::napi_env, napi_val: napi::sys::napi_value) -> napi::Result<Self> {
        let js_str = napi::JsString::from_napi_value(env, napi_val)?;
        let uri = js_str.into_utf8()?.as_str()?.to_string();
        Ok(Some(Connect::open(&uri).map_err(|e| napi::Error::from_reason(format!("Failed to open connection: {:?}", e)))?))
    }
}

// Implement FromNapiRef for &str
impl napi::bindgen_prelude::FromNapiRef for &str {
    fn from_napi_ref(env: napi::sys::napi_env, napi_val: napi::sys::napi_value) -> napi::Result<Self> {
        let js_str = napi::JsString::from_napi_value(env, napi_val)?;
        Ok(js_str.into_utf8()?.as_str()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_connection() {
        let connection = Connection::new(None);
        assert!(connection.is_ok());
        info!("Test for new connection passed.");
    }

    #[test]
    fn test_disconnect_success() {
        let mut connection = Connection::new(None).unwrap();
        assert!(connection.disconnect().is_ok());
        info!("Test for disconnect passed.");
    }

    #[test]
    fn test_define_domain_from_xml_failure_no_connection() {
        let connection = Connection { conn: None };
        let result = connection.define_domain_from_xml("<domain></domain>");
        assert!(result.is_err());
        info!("Test for define_domain_from_xml failure with no connection passed.");
    }

    #[test]
    fn test_find_machine_failure_no_connection() {
        let connection = Connection { conn: None };
        let result = connection.find_machine("test");
        assert!(result.is_err());
        info!("Test for find_machine failure with no connection passed.");
    }

    #[test]
    fn test_list_machines_failure_no_connection() {
        let connection = Connection { conn: None };
        let result = connection.list_machines();
        assert!(result.is_err());
        info!("Test for list_machines failure with no connection passed.");
    }
}