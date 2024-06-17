use napi::{sys, Result as NapiResult, JsNumber};
use napi::bindgen_prelude::{FromNapiValue, ValidateNapiValue};
use virt::domain::Domain;
use crate::connection::Connection;

impl ValidateNapiValue for u64 {
    fn validate(env: sys::napi_env, napi_val: sys::napi_value) -> NapiResult<()> {
        JsNumber::validate(env, napi_val).map(|_| ())
    }
}

impl FromNapiValue for u64 {
    unsafe fn from_napi_value(env: sys::napi_env, napi_val: sys::napi_value) -> NapiResult<Self> {
        let js_number = JsNumber::from_napi_value(env, napi_val)?;
        Ok(js_number.get_uint32()?.into())
    }
}

// Implement FromNapiValue for Domain
impl FromNapiValue for Domain {
    unsafe fn from_napi_value(env: sys::napi_env, napi_val: sys::napi_value) -> NapiResult<Self> {
        // Assuming Domain can be created from a string representation
        let js_string = napi::JsString::from_napi_value(env, napi_val)?;
        let domain_name = js_string.into_utf8()?.as_str()?.to_string();
        let conn = Connection::new(None).map_err(|e| napi::Error::from_reason(format!("Failed to create connection: {:?}", e)))?;
        Domain::lookup_by_name(conn.conn.as_ref().unwrap(), &domain_name).map_err(|e| napi::Error::from_reason(format!("Failed to find domain: {:?}", e)))
    }
}

// Implement FromNapiRef for Connection
impl napi::bindgen_prelude::FromNapiRef for Connection {
    fn from_napi_ref(env: napi::sys::napi_env, napi_val: napi::sys::napi_value) -> NapiResult<Self> {
        // Assuming Connection can be created from a string representation
        let js_string = napi::JsString::from_napi_value(env, napi_val)?;
        let conn_str = js_string.into_utf8()?.as_str()?.to_string();
        Connection::new(Some(&conn_str)).map_err(|e| napi::Error::from_reason(format!("Failed to create connection: {:?}", e)))
    }
}