use napi::{
  bindgen_prelude::*,
};

use virt;

#[napi]
pub struct Error {
  code: virt::error::ErrorNumber,
  domain: virt::error::ErrorDomain,
  message: String,
	level: virt::error::ErrorLevel,
}

#[napi]
impl Error {
  #[napi]
	pub fn last_error() -> Self {
		let err = virt::error::Error::last_error();
		Error {
			code: err.code(),
			domain: err.domain(),
			message: err.to_string(),
			level: err.level(),
		}
	}
}
