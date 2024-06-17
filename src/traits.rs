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

// Implement ToNapiValue for Domain
impl napi::bindgen_prelude::ToNapiValue for Domain {
    unsafe fn to_napi_value(env: napi::sys::napi_env, val: Self) -> NapiResult<napi::sys::napi_value> {
        let domain_name = val.get_name().map_err(|e| napi::Error::from_reason(format!("Failed to get domain name: {:?}", e)))?;
        napi::JsString::from_str(env, &domain_name).map(|js_str| js_str.raw())
    }
}

// Implement FromNapiValue for Option<Connect>
impl napi::bindgen_prelude::FromNapiValue for Option<Connect> {
    unsafe fn from_napi_value(env: napi::sys::napi_env, napi_val: napi::sys::napi_value) -> NapiResult<Self> {
        let js_str = napi::JsString::from_napi_value(env, napi_val)?;
        let uri = js_str.into_utf8()?.as_str()?.to_string();
        Ok(Some(Connect::open(&uri).map_err(|e| napi::Error::from_reason(format!("Failed to open connection: {:?}", e)))?))
    }
}

// Implement ToNapiValue for Option<Connect>
impl napi::bindgen_prelude::ToNapiValue for Option<Connect> {
    unsafe fn to_napi_value(env: napi::sys::napi_env, val: Self) -> NapiResult<napi::sys::napi_value> {
        match val {
            Some(conn) => napi::JsString::from_str(env, &conn.get_uri().unwrap_or("")).map(|js_str| js_str.raw()),
            None => napi::JsObject::new(env).map(|js_obj| js_obj.raw()),
        }
    }
}

// Implement FromNapiRef for &str
impl napi::bindgen_prelude::FromNapiRef for &str {
    fn from_napi_ref(env: napi::sys::napi_env, napi_val: napi::sys::napi_value) -> NapiResult<Self> {
        let js_str = napi::JsString::from_napi_value(env, napi_val)?;
        Ok(js_str.into_utf8()?.as_str()?)
    }
}
```
------------------------end_of_format---------------------------
