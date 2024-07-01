use napi::{
CallContext, Env, JsBoolean, JsObject, JsString, JsUndefined, Property,
Result as NapiResult,
};

use virt::{connect::Connect, domain::Domain};

// Add them when we need them
// use log::{error, info, warn};
use napi_derive::js_function;

use crate::connection::Connection;

#[napi]
pub struct Machine {
    domain: Domain,
    con: Connection
}

#[napi]
impl Machine {

    pub fn from_domain(domain: Domain, con: &Connection) -> Self {
        Self { domain: domain, con: con.clone() }
    }

    #[napi]
    pub fn from_name(name: String, con: &Connection) -> napi::Result<Self> {
        let domain_result = Domain::lookup_by_name(con.get_connection(), &name.to_owned());
        match domain_result {
            Ok(domain) => Ok(Self { domain: domain, con: con.clone() }),
            Err(err) => {
                Err(napi::Error::from_reason(err.to_string()))
            }
        }
    }

}