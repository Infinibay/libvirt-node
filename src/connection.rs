use napi;
use virt::{connect::Connect};

use crate::machine::Machine;

#[napi]
pub struct Connection {
  con: Connect,
}

impl Clone for Connection {
  fn clone(&self) -> Self {
    let uri = self.con.get_uri().expect("Failed to get URI for cloning");
    let new_connection = Connect::open(Some(&uri)).expect("Failed to clone connection");
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

  #[napi]
  pub fn open(name: String) -> Option<Connection> {
    let con = Connect::open(Some(&name));
    match con {
      Ok(connection) => Some(Self { con: connection }),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn close(&mut self) -> i32 {
    match self.con.close() {
      Ok(_) => 0,
      Err(_) => -1
    }
  }

  #[napi]
  pub fn is_alive(&self) -> Option<bool> {
    match self.con.is_alive() {
      Ok(alive) => Some(alive),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn get_sys_info(&self, flags: u32) -> Option<String> {
    // Implement
    match self.con.get_sys_info(flags) {
      Ok(info) => Some(info),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn get_max_vcpus(&self, attr: String) -> Option<u32> {
    match self.con.get_max_vcpus(Some(&attr)) {
      Ok(vcpus) => Some(vcpus),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn get_cpu_models_names(&self, arch: String, flags: u32) -> Option<Vec<String>> {
    match self.con.get_cpu_models_names(&arch, flags) {
      Ok(models) => Some(models),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn is_encrypted(&self) -> Option<bool> {
    match self.con.is_encrypted() {
      Ok(encrypted) => Some(encrypted),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn is_secure(&self) -> Option<bool> {
    match self.con.is_secure() {
      Ok(secure) => Some(secure),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn list_active_domain_ids(&self) -> Option<Vec<u32>> {
    // use list_domains
    match self.con.list_domains() {
      Ok(domains) => Some(domains),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn list_interfaces(&self) -> Option<Vec<String>> {
    match self.con.list_interfaces() {
      Ok(interfaces) => Some(interfaces),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn list_networks(&self) -> Option<Vec<String>> {
    match self.con.list_networks() {
      Ok(networks) => Some(networks),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn list_nw_filters(&self) -> Option<Vec<String>> {
    match self.con.list_nw_filters() {
      Ok(filters) => Some(filters),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn list_secrets(&self) -> Option<Vec<String>> {
    match self.con.list_secrets() {
      Ok(secrets) => Some(secrets),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn list_storage_pools(&self) -> Option<Vec<String>> {
    match self.con.list_storage_pools() {
      Ok(pools) => Some(pools),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn list_all_domains(&self, flags: u32) -> Option<Vec<Machine>> {
    match self.con.list_all_domains(flags) {
      Ok(domains) => {
        let mut machines = Vec::new();
        for domain in domains {
          // Clone the domain to avoid moving out of the shared reference
          let domain_clone = domain;
          machines.push(Machine::from_domain(domain_clone, &self));
        }
        Some(machines)
      }
      Err(_) => None,
    }
  }

  #[napi]
  pub fn list_all_networks(&self, flags: u32) -> Option<Vec<crate::network::Network>> {
    match self.con.list_all_networks(flags) {
      Ok(networks) => {
        let mut network_wrappers = Vec::new();
        for network in networks {
          network_wrappers.push(crate::network::Network::from_network(network));
        }
        Some(network_wrappers)
      }
      Err(_) => None,
    }
  }

  #[napi]
  pub fn list_all_interfaces(&self, flags: u32) -> Option<Vec<crate::interface::Interface>> {
    match self.con.list_all_interfaces(flags) {
      Ok(interfaces) => {
        let mut interface_wrappers = Vec::new();
        for interface in interfaces {
          interface_wrappers.push(crate::interface::Interface::from_interface(interface));
        }
        Some(interface_wrappers)
      }
      Err(_) => None,
    }
  }

  #[napi]
  pub fn list_all_node_devices(
    &self,
    flags: u32,
  ) -> Option<Vec<crate::node_device::NodeDevice>> {
    match self.con.list_all_node_devices(flags) {
      Ok(node_devices) => {
        let mut node_device_wrappers = Vec::new();
        for node_device in node_devices {
          node_device_wrappers.push(crate::node_device::NodeDevice::from_node(node_device));
        }
        Some(node_device_wrappers)
      }
      Err(_) => None,
    }
  }

  #[napi]
  pub fn list_all_secrets(&self, flags: u32) -> Option<Vec<crate::secret::Secret>> {
    match self.con.list_all_secrets(flags) {
      Ok(secrets) => {
        let mut secret_wrappers = Vec::new();
        for secret in secrets {
          secret_wrappers.push(crate::secret::Secret::from_secret(secret));
        }
        Some(secret_wrappers)
      }
      Err(_) => None,
    }
  }

  #[napi]
  pub fn list_all_storage_pools(
    &self,
    flags: u32,
  ) -> Option<Vec<crate::storage_pool::StoragePool>> {
    match self.con.list_all_storage_pools(flags) {
      Ok(storage_pools) => {
        let mut storage_pool_wrappers = Vec::new();
        for storage_pool in storage_pools {
          storage_pool_wrappers.push(crate::storage_pool::StoragePool::from_storage_pool(
            storage_pool,
          ));
        }
        Some(storage_pool_wrappers)
      }
      Err(_) => None,
    }
  }

  #[napi]
  pub fn list_all_nw_filters(&self, flags: u32) -> Option<Vec<crate::nw_filter::NWFilter>> {
    match self.con.list_all_nw_filters(flags) {
      Ok(nw_filters) => {
        let mut nw_filter_wrappers = Vec::new();
        for nw_filter in nw_filters {
          nw_filter_wrappers.push(crate::nw_filter::NWFilter::from_nw_filter(nw_filter));
        }
        Some(nw_filter_wrappers)
      }
      Err(_) => None,
    }
  }

  #[napi]
  pub fn list_defined_domains(&self) -> Option<Vec<String>> {
    match self.con.list_defined_domains() {
      Ok(domains) => {
        let mut domain_names = Vec::new();
        for domain in domains {
          domain_names.push(domain);
        }
        Some(domain_names)
      }
      Err(_) => None,
    }
  }

  #[napi]
  pub fn list_defined_interfaces(&self) -> Option<Vec<String>> {
    match self.con.list_defined_interfaces() {
      Ok(interfaces) => {
        let mut interface_names = Vec::new();
        for interface in interfaces {
          interface_names.push(interface);
        }
        Some(interface_names)
      }
      Err(_) => None,
    }
  }

  #[napi]
  pub fn list_defined_storage_pools(&self) -> Option<Vec<String>> {
    match self.con.list_defined_storage_pools() {
      Ok(pools) => {
        let mut pool_names = Vec::new();
        for pool in pools {
          pool_names.push(pool);
        }
        Some(pool_names)
      }
      Err(_) => None,
    }
  }

  #[napi]
  pub fn list_defined_networks(&self) -> Option<Vec<String>> {
    match self.con.list_defined_networks() {
      Ok(networks) => {
        let mut network_names = Vec::new();
        for network in networks {
          network_names.push(network);
        }
        Some(network_names)
      }
      Err(_) => None,
    }
  }

  #[napi]
  pub fn num_of_domains(&self) -> Option<u32> {
    match self.con.num_of_domains() {
      Ok(num) => Some(num),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn num_of_interfaces(&self) -> Option<u32> {
    match self.con.num_of_interfaces() {
      Ok(num) => Some(num),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn num_of_networks(&self) -> Option<u32> {
    match self.con.num_of_networks() {
      Ok(num) => Some(num),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn num_of_storage_pools(&self) -> Option<u32> {
    match self.con.num_of_storage_pools() {
      Ok(num) => Some(num),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn num_of_nw_filters(&self) -> Option<u32> {
    match self.con.num_of_nw_filters() {
      Ok(num) => Some(num),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn num_of_secrets(&self) -> Option<u32> {
    match self.con.num_of_secrets() {
      Ok(num) => Some(num),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn num_of_node_devices(&self) -> Option<u32> {
    match self.con.num_of_defined_domains() {
      Ok(num) => Some(num),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn num_of_defined_domains(&self) -> Option<u32> {
    match self.con.num_of_defined_domains() {
      Ok(num) => Some(num),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn num_of_defined_interfaces(&self) -> Option<u32> {
    match self.con.num_of_defined_interfaces() {
      Ok(num) => Some(num),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn num_of_defined_networks(&self) -> Option<u32> {
    match self.con.num_of_defined_networks() {
      Ok(num) => Some(num),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn get_hyp_version(&self) -> Option<u32> {
    match self.con.get_hyp_version() {
      Ok(version) => Some(version),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn compare_cpu(
    &self,
    xml: String,
    flags: u32,
  ) -> Option<i32> {
    match self.con.compare_cpu(&xml, flags) {
      Ok(result) => Some(result),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn get_free_memory(&self) -> Option<u64> {
    match self.con.get_free_memory() {
      Ok(memory) => Some(memory),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn get_node_info(&self) -> Option<crate::node_info::NodeInfo> {
    match self.con.get_node_info() {
      Ok(info) => Some(crate::node_info::NodeInfo::from_node_info(info)),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn set_keep_alive(&self, interval: i32, count: u32) -> Option<i32> {
    match self.con.set_keep_alive(interval, count) {
      Ok(result) => Some(result),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn domain_xml_from_native(
    &self,
    nformat: String,
    nconfig: String,
    flags: u32,
  ) -> Option<String> {
    match self.con.domain_xml_from_native(&nformat, &nconfig, flags) {
      Ok(xml) => Some(xml),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn domain_xml_to_native(
    &self,
    nformat: String,
    dxml: String,
    flags: u32,
  ) -> Option<String> {
    match self.con.domain_xml_to_native(&nformat, &dxml, flags) {
      Ok(xml) => Some(xml),
      Err(_) => None,
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
  ) -> Option<String> {
    match self.con.get_domain_capabilities(Some(&emulatorbin), Some(&arch), Some(&machine), Some(&virttype), flags) {
      Ok(xml) => Some(xml),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn get_all_domain_stats(
    &self,
    stats: u32,
    flags: u32,
  ) -> Option<Vec<crate::domain_stats_record::DomainStatsRecord>> {
    match self.con.get_all_domain_stats(stats, flags) {
      Ok(stats) => {
        let mut stats_wrappers = Vec::new();
        for stat in stats {
          stats_wrappers.push(crate::domain_stats_record::DomainStatsRecord::from_stat(stat));
        }
        Some(stats_wrappers)
      }
      Err(_) => None,
    }
  }

  #[napi]
  pub fn baseline_cpu(
    &self,
    xmlcpus: Vec<String>,
    flags: u32,
  ) -> Option<String> {
    let xmlcpus_refs: Vec<&str> = xmlcpus.iter().map(|s| s.as_str()).collect();
    match self.con.baseline_cpu(&xmlcpus_refs, flags) {
      Ok(xml) => Some(xml),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn find_storage_pool_sources(
    &self,
    kind: String,
    spec: String,
    flags: u32,
  ) -> Option<String> {
    match self.con.find_storage_pool_sources(&kind, Some(&spec), flags) {
      Ok(xml) => Some(xml),
      Err(_) => None,
    }
  }
}
