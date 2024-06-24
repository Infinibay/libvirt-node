use log::{warn, info, error};
use virt::{connect::Connect, domain::Domain};
use napi::{Env, JsObject, Result as NapiResult, JsString, JsUndefined, JsNumber, CallContext, Property};
use napi_derive::js_function;

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

#[js_function(1)] // constructor takes 1 argument
fn constructor(ctx: CallContext) -> NapiResult<JsUndefined> {
    let arg = ctx.get::<JsString>(0)?;
    let uri = arg.into_utf8()?.as_str()?.to_owned();
    let conn = if uri.is_empty() {
        None
    } else {
        match Connect::open(&uri) {
            Ok(conn) => Some(conn),
            Err(_) => None, // Handle error appropriately
        }
    };

    let mut this: JsObject = ctx.this_unchecked();
    ctx.env.wrap(&mut this, conn)?;

    ctx.env.get_undefined()
}

#[js_function(1)]
fn connect(ctx: CallContext) -> NapiResult<JsUndefined> {
    let uri = ctx.get::<JsString>(0)?.into_utf8()?.as_str()?.to_owned();
		let this: JsObject = ctx.this_unchecked();
    let connection: &mut Connection = ctx.env.unwrap(&this)?;
    connection.connect_internal(uri)?;
    ctx.env.get_undefined()
}

#[js_function(0)]
fn disconnect(ctx: CallContext) -> NapiResult<JsUndefined> {
	let this: JsObject = ctx.this_unchecked();
	let connection: &mut Connection = ctx.env.unwrap(&this)?;
    connection.disconnect_internal()?;
    ctx.env.get_undefined()
}

#[js_function(1)]
fn define_domain_from_xml(ctx: CallContext) -> NapiResult<JsObject> {
    let xml_desc = ctx.get::<JsString>(0)?.into_utf8()?.as_str()?.to_owned();
		let this: JsObject = ctx.this_unchecked();
    let connection: &mut Connection = ctx.env.unwrap(&this)?;
    let domain = connection.define_domain_from_xml_internal(xml_desc)?;
    let js_domain = ctx.env.create_object()?;
    // Set properties on js_domain as needed
    Ok(js_domain)
}

#[js_function(0)]
fn list_machines(ctx: CallContext) -> NapiResult<JsObject> {
	let this: JsObject = ctx.this_unchecked();
	let connection: &mut Connection = ctx.env.unwrap(&this)?;
	let domains = connection.list_machines_internal()?;
	let js_domains = ctx.env.create_object()?;
	// Set properties on js_domains as needed
	Ok(js_domains)
}

impl Connection {
    fn connect_internal(&mut self, uri: String) -> Result<(), ConnectionError> {
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

    fn define_domain_from_xml_internal(&self, xml_desc: String) -> Result<Domain, ConnectionError> {
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
}

pub fn register_js(exports: &mut JsObject, env: Env) -> NapiResult<()> {
    let connection_class = env.define_class(
        "Connection",
        constructor,
        &[
						Property::new("connect")?.with_method(connect),
            Property::new("disconnect")?.with_method(disconnect),
            Property::new("defineDomainFromXml")?.with_method(define_domain_from_xml),
            Property::new("listMachines")?.with_method(list_machines),
        ],
    )?;
    exports.set_named_property("Connection", connection_class)?;
    Ok(())
}

impl From<ConnectionError> for napi::Error {
    fn from(err: ConnectionError) -> Self {
        napi::Error::new(napi::Status::GenericFailure, format!("{:?}", err))
    }
}