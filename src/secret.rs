use napi::{
  CallContext, Env, JsBoolean, JsObject, JsString, JsUndefined, Property, Result as NapiResult,
};

use virt;

// Add them when we need them
// use log::{error, info, warn};
use napi_derive::js_function;

use crate::connection::Connection;

#[napi]
pub struct Secret {
	secret: virt::secret::Secret
}

impl Secret {
	pub fn get(&self) -> &virt::secret::Secret {
		&self.secret
	}

	pub fn from_secret(secret: virt::secret::Secret) -> Self {
		Self { secret: secret }
	}
}