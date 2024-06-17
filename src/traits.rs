use napi::{sys, Result as NapiResult, JsNumber, JsString, JsObject};
use napi::bindgen_prelude::{FromNapiValue, ValidateNapiValue, ToNapiValue, FromNapiRef};
use virt::domain::Domain;
use virt::connect::Connect;
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
        let js_string = JsString::from_napi_value(env, napi_val)?;
        let domain_name = js_string.into_utf8()?.as_str()?.to_string();
        let conn = Connection::new(None).map_err(|e| napi::Error::from_reason(format!("Failed to create connection: {:?}", e)))?;
        Domain::lookup_by_name(conn.conn.as_ref().unwrap(), &domain_name).map_err(|e| napi::Error::from_reason(format!("Failed to find domain: {:?}", e)))
    }
}

// Implement FromNapiRef for Connection
impl FromNapiRef for Connection {
    unsafe fn from_napi_ref(env: napi::sys::napi_env, napi_val: napi::sys::napi_value) -> NapiResult<Self> {
        // Assuming Connection can be created from a string representation
        let js_string = JsString::from_napi_value(env, napi_val)?;
        let conn_str = js_string.into_utf8()?.as_str()?.to_string();
        Connection::new(Some(&conn_str)).map_err(|e| napi::Error::from_reason(format!("Failed to create connection: {:?}", e)))
    }
}

// Implement ToNapiValue for Domain
impl ToNapiValue for Domain {
    unsafe fn to_napi_value(env: napi::sys::napi_env, val: Self) -> NapiResult<napi::sys::napi_value> {
        let domain_name = val.get_name().map_err(|e| napi::Error::from_reason(format!("Failed to get domain name: {:?}", e)))?;
        JsString::from(env, domain_name).map(|js_str| js_str.raw())
    }
}

// Implement FromNapiValue for Option<Connect>
impl FromNapiValue for Option<Connect> {
    unsafe fn from_napi_value(env: napi::sys::napi_env, napi_val: sys::napi_value) -> NapiResult<Self> {
        let js_str = JsString::from_napi_value(env, napi_val)?;
        let uri = js_str.into_utf8()?.as_str()?.to_string();
        Ok(Some(Connect::open(&uri).map_err(|e| napi::Error::from_reason(format!("Failed to open connection: {:?}", e)))?))
    }
}

// Implement ToNapiValue for Option<Connect>
impl ToNapiValue for Option<Connect> {
    unsafe fn to_napi_value(env: napi::sys::napi_env, val: Self) -> NapiResult<napi::sys::napi_value> {
        match val {
            Some(conn) => JsString::from(env, conn.get_uri().unwrap_or_default()).map(|js_str| js_str.raw()),
            None => JsObject::new(env).map(|js_obj| js_obj.raw()),
        }
    }
}

// Implement FromNapiRef for &str
impl FromNapiRef for &str {
    unsafe fn from_napi_ref(env: napi::sys::napi_env, napi_val: napi::sys::napi_value) -> NapiResult<Self> {
        let js_str = JsString::from_napi_value(env, napi_val)?;
        Ok(js_str.into_utf8()?.as_str()?)
    }
}