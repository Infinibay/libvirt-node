use napi::{
  CallContext, Env, JsBoolean, JsObject, JsString, JsUndefined, Property, Result as NapiResult,
};

use virt;

// Add them when we need them
// use log::{error, info, warn};
use napi_derive::js_function;

use crate::connection::Connection;

#[napi]
pub struct Interface {
	interface: virt::interface::Interface
}

impl Interface {
	pub fn get(&self) -> &virt::interface::Interface {
		&self.interface
	}

	pub fn from_interface(int: virt::interface::Interface) -> Self {
		Self { interface: int }
	}
}