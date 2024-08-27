use napi::{
  CallContext, Env, JsBoolean, JsObject, JsString, JsUndefined, Property, Result,
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

#[napi]
impl Network {
	pub fn get(&self) -> &virt::network::Network {
		&self.network
	}

	pub fn from_network(network: virt::network::Network) -> Network {
		Network { network }
	}

	#[napi]
	pub fn lookup_by_name(conn: &Connection, name: String) -> napi::Result<Network> {
		let network = virt::network::Network::lookup_by_name(conn.get_connection(), &name);
		match network {
			Ok(network) => Ok(Network { network }),
			Err(e) => {
				Err(napi::Error::new(napi::Status::Unknown, e.to_string()))
			}
		}
	}

	#[napi]
	pub fn lookup_by_uuid_string(conn: &Connection, uuid: String) -> napi::Result<Network> {
		let network = virt::network::Network::lookup_by_uuid_string(conn.get_connection(), &uuid);
		match network {
			Ok(network) => Ok(Network { network }),
			Err(e) => {
				Err(napi::Error::new(napi::Status::Unknown, e.to_string()))
			}
		}
	}

	#[napi]
	pub fn get_name(&self) -> napi::Result<String> {
		let name = self.network.get_name();
		match name {
			Ok(name) => Ok(name),
			Err(e) => {
				Err(napi::Error::new(napi::Status::Unknown, e.to_string()))
			}
		}
	}

	#[napi]
	pub fn get_uuid_string(&self) -> napi::Result<String> {
		let uuid = self.network.get_uuid_string();
		match uuid {
			Ok(uuid) => Ok(uuid),
			Err(e) => {
				Err(napi::Error::new(napi::Status::Unknown, e.to_string()))
			}
		}
	}

	#[napi]
	pub fn get_bridge_name(&self) -> napi::Result<String> {
		let bridge_name = self.network.get_bridge_name();
		match bridge_name {
			Ok(bridge_name) => Ok(bridge_name),
			Err(e) => {
				Err(napi::Error::new(napi::Status::Unknown, e.to_string()))
			}
		}
	}

	#[napi]
	pub fn get_xml_desc(&self, flags: u32) -> napi::Result<String> {
		let xml_desc = self.network.get_xml_desc(flags);
		match xml_desc {
			Ok(xml_desc) => Ok(xml_desc),
			Err(e) => {
				Err(napi::Error::new(napi::Status::Unknown, e.to_string()))
			}
		}
	}

	#[napi]
	pub fn create(&self) -> napi::Result<u32> {
		let ret = self.network.create();
		match ret {
			Ok(ret) => Ok(ret),
			Err(e) => {
				Err(napi::Error::new(napi::Status::Unknown, e.to_string()))
			}
		}
	}

	#[napi]
	pub fn define_xml(conn: &Connection, xml: String) -> napi::Result<Network> {
		let ret = virt::network::Network::define_xml(conn.get_connection(), &xml);
		match ret {
			Ok(ret) => Ok(Network { network: ret }),
			Err(e) => {
				Err(napi::Error::new(napi::Status::Unknown, e.to_string()))
			}
		}
	}

	#[napi]
	pub fn create_xml(conn: &Connection, xml: String) -> napi::Result<Network> {
		let ret = virt::network::Network::create_xml(conn.get_connection(), &xml);
		match ret {
			Ok(ret) => Ok(Network { network: ret }),
			Err(e) => {
				Err(napi::Error::new(napi::Status::Unknown, e.to_string()))
			}
		}
	}

	#[napi]
	pub fn destroy(&self) -> napi::Result<()> {
		let ret = self.network.destroy();
		match ret {
			Ok(ret) => Ok(ret),
			Err(e) => {
				Err(napi::Error::new(napi::Status::Unknown, e.to_string()))
			}
		}
	}

	#[napi]
	pub fn undefine(&self) -> napi::Result<()> {
		let ret = self.network.undefine();
		match ret {
			Ok(ret) => Ok(ret),
			Err(e) => {
				Err(napi::Error::new(napi::Status::Unknown, e.to_string()))
			}
		}
	}

	#[napi]
	pub fn free(&mut self) -> napi::Result<()> {
		let ret = self.network.free();
		match ret {
			Ok(ret) => Ok(ret),
			Err(e) => {
				Err(napi::Error::new(napi::Status::Unknown, e.to_string()))
			}
		}
	}

	#[napi]
	pub fn is_active(&self) -> napi::Result<bool> {
		let ret = self.network.is_active();
		match ret {
			Ok(ret) => Ok(ret),
			Err(e) => {
				Err(napi::Error::new(napi::Status::Unknown, e.to_string()))
			}
		}
	}

	#[napi]
	pub fn is_persistent(&self) -> napi::Result<bool> {
		let ret = self.network.is_persistent();
		match ret {
			Ok(ret) => Ok(ret),
			Err(e) => {
				Err(napi::Error::new(napi::Status::Unknown, e.to_string()))
			}
		}
	}

	#[napi]
	pub fn get_autostart(&self) -> napi::Result<bool> {
		let ret = self.network.get_autostart();
		match ret {
			Ok(ret) => Ok(ret),
			Err(e) => {
				Err(napi::Error::new(napi::Status::Unknown, e.to_string()))
			}
		}
	}

	#[napi]
	pub fn set_autostart(&self, autostart: bool) -> napi::Result<u32> {
		let ret = self.network.set_autostart(autostart);
		match ret {
			Ok(ret) => Ok(ret),
			Err(e) => {
				Err(napi::Error::new(napi::Status::Unknown, e.to_string()))
			}
		}
	}

	#[napi]
	pub fn update(&self, cmd: u32, section: u32, index: i32, xml: String, flags: u32) -> napi::Result<()> {
		let ret = self.network.update(cmd, section, index, &xml, flags);
		match ret {
			Ok(ret) => Ok(ret),
			Err(e) => {
				Err(napi::Error::new(napi::Status::Unknown, e.to_string()))
			}
		}
	}
}