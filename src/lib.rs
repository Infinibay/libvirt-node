#![deny(clippy::all)]

use napi_derive::napi;
use napi::{
  bindgen_prelude::{Buffer, ClassInstance, ObjectFinalize, This, Uint8Array, Unknown},
  Env, Property, Result,
};

#[napi]
pub struct Libvirt {
}