use napi::{
  CallContext, Env, JsBoolean, JsObject, JsString, JsUndefined, Property, Result as NapiResult,
};

use virt;

// Add them when we need them
// use log::{error, info, warn};
use napi_derive::js_function;

use crate::connection::Connection;

#[napi]
pub struct Network {
	network: virt::network::Network
}

impl Network {
	pub fn get(&self) -> &virt::network::Network {
		&self.network
	}

	pub fn from_network(net: virt::network::Network) -> Self {
		Self { network: net }
	}
}