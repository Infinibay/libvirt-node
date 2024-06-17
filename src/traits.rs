use napi::{sys, Result as NapiResult, JsNumber, FromNapiValue, ValidateNapiValue};

impl ValidateNapiValue for u64 {
    fn validate(env: sys::napi_env, napi_val: sys::napi_value) -> NapiResult<()> {
        JsNumber::validate(env, napi_val)
    }
}

impl FromNapiValue for u64 {
    unsafe fn from_napi_value(env: sys::napi_env, napi_val: sys::napi_value) -> NapiResult<Self> {
        let js_number = JsNumber::from_napi_value(env, napi_val)?;
        Ok(js_number.get_uint32()?.into())
    }
}