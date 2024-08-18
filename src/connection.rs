use napi::{
  bindgen_prelude::{FromNapiValue, ToNapiValue},
  CallContext, Env, JsBoolean, JsObject, JsString, JsUndefined, NapiRaw, Property,
  Result as NapiResult,
};

use virt::{connect::Connect, domain::Domain};

// Add them when we need them
// use log::{error, info, warn};
use napi_derive::js_function;

use crate::machine::Machine;

#[napi]
pub struct Connection {
  con: Connect,
}

impl Clone for Connection {
  fn clone(&self) -> Self {
    let uri = self.con.get_uri().expect("Failed to get URI for cloning");
    let new_connection = Connect::open(&uri).expect("Failed to clone connection");
    Connection {
      con: new_connection,
    }
  }
}

#[napi]
impl Connection {
  pub fn get_connection(&self) -> &Connect {
    return &self.con;
  }

  #[napi(factory)]
  pub fn open(name: String) -> napi::Result<Self> {
    let con = Connect::open(&name);
    match con {
      Ok(connection) => Ok(Self { con: connection }),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  // close
  #[napi]
  pub fn close(&mut self) -> napi::Result<()> {
    match self.con.close() {
      Ok(_) => Ok(()),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn is_alive(&self) -> napi::Result<bool> {
    match self.con.is_alive() {
      Ok(alive) => Ok(alive),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn get_sys_info(&self, flags: u32) -> napi::Result<String> {
    // Implement
    match self.con.get_sys_info(flags) {
      Ok(info) => Ok(info),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn get_max_vcpus(&self, attr: String) -> napi::Result<u32> {
    match self.con.get_max_vcpus(&attr) {
      Ok(vcpus) => Ok(vcpus),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn get_cpu_models_names(&self, arch: String, flags: u32) -> napi::Result<Vec<String>> {
    match self.con.get_cpu_models_names(&arch, flags) {
      Ok(models) => Ok(models),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn is_encrypted(&self) -> napi::Result<bool> {
    match self.con.is_encrypted() {
      Ok(encrypted) => Ok(encrypted),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn is_secure(&self) -> napi::Result<bool> {
    match self.con.is_secure() {
      Ok(secure) => Ok(secure),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn list_active_domain_ids(&self) -> napi::Result<Vec<u32>> {
    // use list_domains
    match self.con.list_domains() {
      Ok(domains) => Ok(domains),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn list_interfaces(&self) -> napi::Result<Vec<String>> {
    match self.con.list_interfaces() {
      Ok(interfaces) => Ok(interfaces),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn list_networks(&self) -> napi::Result<Vec<String>> {
    match self.con.list_networks() {
      Ok(networks) => Ok(networks),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn list_nw_filters(&self) -> napi::Result<Vec<String>> {
    match self.con.list_nw_filters() {
      Ok(filters) => Ok(filters),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn list_secrets(&self) -> napi::Result<Vec<String>> {
    match self.con.list_secrets() {
      Ok(secrets) => Ok(secrets),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn list_storage_pools(&self) -> napi::Result<Vec<String>> {
    match self.con.list_storage_pools() {
      Ok(pools) => Ok(pools),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn list_all_domains(&self, flags: u32) -> napi::Result<Vec<Machine>> {
    match self.con.list_all_domains(flags) {
      Ok(domains) => {
        let mut machines = Vec::new();
        for domain in domains {
          // Clone the domain to avoid moving out of the shared reference
          let domain_clone = domain;
          machines.push(Machine::from_domain(domain_clone, &self));
        }
        Ok(machines)
      }
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn list_all_networks(&self, flags: u32) -> napi::Result<Vec<crate::network::Network>> {
    match self.con.list_all_networks(flags) {
      Ok(networks) => {
        let mut network_wrappers = Vec::new();
        for network in networks {
          network_wrappers.push(crate::network::Network::from_network(network));
        }
        Ok(network_wrappers)
      }
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn list_all_interfaces(&self, flags: u32) -> napi::Result<Vec<crate::interface::Interface>> {
    match self.con.list_all_interfaces(flags) {
      Ok(interfaces) => {
        let mut interface_wrappers = Vec::new();
        for interface in interfaces {
          interface_wrappers.push(crate::interface::Interface::from_interface(interface));
        }
        Ok(interface_wrappers)
      }
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn list_all_node_devices(
    &self,
    flags: u32,
  ) -> napi::Result<Vec<crate::node_device::NodeDevice>> {
    match self.con.list_all_node_devices(flags) {
      Ok(node_devices) => {
        let mut node_device_wrappers = Vec::new();
        for node_device in node_devices {
          node_device_wrappers.push(crate::node_device::NodeDevice::from_node(node_device));
        }
        Ok(node_device_wrappers)
      }
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn list_all_secrets(&self, flags: u32) -> napi::Result<Vec<crate::secret::Secret>> {
    match self.con.list_all_secrets(flags) {
      Ok(secrets) => {
        let mut secret_wrappers = Vec::new();
        for secret in secrets {
          secret_wrappers.push(crate::secret::Secret::from_secret(secret));
        }
        Ok(secret_wrappers)
      }
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn list_all_storage_pools(
    &self,
    flags: u32,
  ) -> napi::Result<Vec<crate::storage_pool::StoragePool>> {
    match self.con.list_all_storage_pools(flags) {
      Ok(storage_pools) => {
        let mut storage_pool_wrappers = Vec::new();
        for storage_pool in storage_pools {
          storage_pool_wrappers.push(crate::storage_pool::StoragePool::from_storage_pool(
            storage_pool,
          ));
        }
        Ok(storage_pool_wrappers)
      }
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn list_all_nw_filters(&self, flags: u32) -> napi::Result<Vec<crate::nw_filter::NWFilter>> {
    match self.con.list_all_nw_filters(flags) {
      Ok(nw_filters) => {
        let mut nw_filter_wrappers = Vec::new();
        for nw_filter in nw_filters {
          nw_filter_wrappers.push(crate::nw_filter::NWFilter::from_nw_filter(nw_filter));
        }
        Ok(nw_filter_wrappers)
      }
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn list_defined_domains(&self) -> napi::Result<Vec<String>> {
    match self.con.list_defined_domains() {
      Ok(domains) => {
        let mut domain_names = Vec::new();
        for domain in domains {
          domain_names.push(domain);
        }
        Ok(domain_names)
      }
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn list_defined_interfaces(&self) -> napi::Result<Vec<String>> {
    match self.con.list_defined_interfaces() {
      Ok(interfaces) => {
        let mut interface_names = Vec::new();
        for interface in interfaces {
          interface_names.push(interface);
        }
        Ok(interface_names)
      }
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn list_defined_storage_pools(&self) -> napi::Result<Vec<String>> {
    match self.con.list_defined_storage_pools() {
      Ok(pools) => {
        let mut pool_names = Vec::new();
        for pool in pools {
          pool_names.push(pool);
        }
        Ok(pool_names)
      }
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn list_defined_networks(&self) -> napi::Result<Vec<String>> {
    match self.con.list_defined_networks() {
      Ok(networks) => {
        let mut network_names = Vec::new();
        for network in networks {
          network_names.push(network);
        }
        Ok(network_names)
      }
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn num_of_domains(&self) -> napi::Result<u32> {
    match self.con.num_of_domains() {
      Ok(num) => Ok(num),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn num_of_interfaces(&self) -> napi::Result<u32> {
    match self.con.num_of_interfaces() {
      Ok(num) => Ok(num),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn num_of_networks(&self) -> napi::Result<u32> {
    match self.con.num_of_networks() {
      Ok(num) => Ok(num),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn num_of_storage_pools(&self) -> napi::Result<u32> {
    match self.con.num_of_storage_pools() {
      Ok(num) => Ok(num),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn num_of_nw_filters(&self) -> napi::Result<u32> {
    match self.con.num_of_nw_filters() {
      Ok(num) => Ok(num),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn num_of_secrets(&self) -> napi::Result<u32> {
    match self.con.num_of_secrets() {
      Ok(num) => Ok(num),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn num_of_node_devices(&self) -> napi::Result<u32> {
    match self.con.num_of_defined_domains() {
      Ok(num) => Ok(num),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn num_of_defined_domains(&self) -> napi::Result<u32> {
    match self.con.num_of_defined_domains() {
      Ok(num) => Ok(num),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn num_of_defined_interfaces(&self) -> napi::Result<u32> {
    match self.con.num_of_defined_interfaces() {
      Ok(num) => Ok(num),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn num_of_defined_networks(&self) -> napi::Result<u32> {
    match self.con.num_of_defined_networks() {
      Ok(num) => Ok(num),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn get_hyp_version(&self) -> napi::Result<u32> {
    match self.con.get_hyp_version() {
      Ok(version) => Ok(version),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn compare_cpu(
    &self,
    xml: String,
    flags: u32,
  ) -> napi::Result<i32> {
    match self.con.compare_cpu(&xml, flags) {
      Ok(result) => Ok(result),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn get_free_memory(&self) -> napi::Result<u64> {
    match self.con.get_free_memory() {
      Ok(memory) => Ok(memory),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn get_node_info(&self) -> napi::Result<crate::node_info::NodeInfo> {
    match self.con.get_node_info() {
      Ok(info) => Ok(crate::node_info::NodeInfo::from_node_info(info)),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn set_keep_alive(&self, interval: i32, count: u32) -> napi::Result<i32> {
    match self.con.set_keep_alive(interval, count) {
      Ok(result) => Ok(result),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn domain_xml_from_native(
    &self,
    nformat: String,
    nconfig: String,
    flags: u32,
  ) -> napi::Result<String> {
    match self.con.domain_xml_from_native(&nformat, &nconfig, flags) {
      Ok(xml) => Ok(xml),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn domain_xml_to_native(
    &self,
    nformat: String,
    dxml: String,
    flags: u32,
  ) -> napi::Result<String> {
    match self.con.domain_xml_to_native(&nformat, &dxml, flags) {
      Ok(xml) => Ok(xml),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn get_domain_capabilities(
    &self,
    emulatorbin: String,
    arch: String,
    machine: String,
    virttype: String,
    flags: u32,
  ) -> napi::Result<String> {
    match self.con.get_domain_capabilities(&emulatorbin, &arch, &machine, &virttype, flags) {
      Ok(xml) => Ok(xml),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn get_all_domain_stats(
    &self,
    stats: u32,
    flags: u32,
  ) -> napi::Result<Vec<crate::domain_stats_record::DomainStatsRecord>> {
    match self.con.get_all_domain_stats(stats, flags) {
      Ok(stats) => {
        let mut stats_wrappers = Vec::new();
        for stat in stats {
          stats_wrappers.push(crate::domain_stats_record::DomainStatsRecord::from_stat(stat));
        }
        Ok(stats_wrappers)
      }
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn baseline_cpu(
    &self,
    xmlcpus: Vec<String>,
    flags: u32,
  ) -> napi::Result<String> {
    let xmlcpus_refs: Vec<&str> = xmlcpus.iter().map(|s| s.as_str()).collect();
    match self.con.baseline_cpu(&xmlcpus_refs, flags) {
      Ok(xml) => Ok(xml),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn find_storage_pool_sources(
    &self,
    kind: String,
    spec: String,
    flags: u32,
  ) -> napi::Result<String> {
    match self.con.find_storage_pool_sources(&kind, &spec, flags) {
      Ok(xml) => Ok(xml),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }
}
