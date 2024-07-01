use napi::{
  CallContext, Env, JsBoolean, JsObject, JsString, JsUndefined, Property, Result as NapiResult,
};

use virt;

// Add them when we need them
// use log::{error, info, warn};
use napi_derive::js_function;

use crate::connection::Connection;

#[napi]
pub struct NodeDevice {
	node: virt::nodedev::NodeDevice
}

impl NodeDevice {
	pub fn get(&self) -> &virt::nodedev::NodeDevice {
		&self.node
	}

	pub fn from_node(node: virt::nodedev::NodeDevice) -> Self {
		Self { node: node }
	}
}