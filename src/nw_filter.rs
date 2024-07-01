use napi::{CallContext, Env, JsObject, JsString, JsUndefined, Property, Result as NapiResult};
use virt;

use napi_derive::js_function;

use crate::connection::Connection;

#[napi]
pub struct NWFilter {
    nw_filter: virt::nwfilter::NWFilter
}

impl NWFilter {
    pub fn get(&self) -> &virt::nwfilter::NWFilter {
        &self.nw_filter
    }

    pub fn from_nw_filter(nw_filter: virt::nwfilter::NWFilter) -> Self {
        Self { nw_filter }
    }
}
