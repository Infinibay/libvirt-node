use napi;

use virt;

use crate::connection::Connection;

#[napi]
pub struct Network {
  network: virt::network::Network,
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
  pub fn lookup_by_name(conn: &Connection, name: String) -> Option<Network> {
    match virt::network::Network::lookup_by_name(conn.get_connection(), &name) {
      Ok(network) => Some(Network { network }),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn lookup_by_uuid_string(conn: &Connection, uuid: String) -> Option<Network> {
    match virt::network::Network::lookup_by_uuid_string(conn.get_connection(), &uuid) {
      Ok(network) => Some(Network { network }),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn get_name(&self) -> Option<String> {
    match self.network.get_name() {
      Ok(name) => Some(name),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn get_uuid_string(&self) -> Option<String> {
    match self.network.get_uuid_string() {
      Ok(uuid) => Some(uuid),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn get_bridge_name(&self) -> Option<String> {
    match self.network.get_bridge_name() {
      Ok(bridge_name) => Some(bridge_name),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn get_xml_desc(&self, flags: u32) -> Option<String> {
    match self.network.get_xml_desc(flags) {
      Ok(xml_desc) => Some(xml_desc),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn create(&self) -> Option<u32> {
    match self.network.create() {
      Ok(_ret) => Some(0),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn define_xml(conn: &Connection, xml: String) -> Option<Network> {
    match virt::network::Network::define_xml(conn.get_connection(), &xml) {
      Ok(ret) => Some(Network { network: ret }),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn create_xml(conn: &Connection, xml: String) -> Option<Network> {
    match virt::network::Network::create_xml(conn.get_connection(), &xml) {
      Ok(ret) => Some(Network { network: ret }),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn destroy(&self) -> Option<u32> {
    match self.network.destroy() {
      Ok(_ret) => Some(0),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn undefine(&self) -> Option<u32> {
    match self.network.undefine() {
      Ok(_ret) => Some(0),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn free(&mut self) -> Option<u32> {
    match self.network.free() {
      Ok(_ret) => Some(0),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn is_active(&self) -> Option<bool> {
    match self.network.is_active() {
      Ok(ret) => Some(ret),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn is_persistent(&self) -> Option<bool> {
    match self.network.is_persistent() {
      Ok(ret) => Some(ret),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn get_autostart(&self) -> Option<bool> {
    match self.network.get_autostart() {
      Ok(ret) => Some(ret),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn set_autostart(&self, autostart: bool) -> Option<u32> {
    match self.network.set_autostart(autostart) {
      Ok(ret) => Some(ret),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn update(&self, cmd: u32, section: u32, index: i32, xml: String, flags: u32) -> Option<u32> {
    match self.network.update(cmd, section, index, &xml, flags) {
      Ok(_ret) => Some(0),
      Err(_) => None,
    }
  }
}
