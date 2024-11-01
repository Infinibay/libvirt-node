use napi::{
  bindgen_prelude::BigInt, JsObject,
};

use virt::domain::Domain;

use napi::bindgen_prelude::*;

use crate::connection::Connection;

/// Represents a virtual machine.
#[napi]
pub struct Machine {
  domain: Domain,
  con: Connection,
}

/// Contains information about a virtual machine.
#[napi]
pub struct MachineInfo {
  /// The running state, one of virDomainState.
  pub state: u32,
  /// The maximum memory in KBytes allowed.
  pub max_mem: BigInt, // Is u64, but napi does not support it
  /// The memory in KBytes used by the domain.
  pub memory: BigInt, // Is u64, but napi does not support it
  /// The number of virtual CPUs for the domain.
  pub nr_virt_cpu: u32,
  /// The CPU time used in nanoseconds.
  pub cpu_time: BigInt, // Is u64, but napi does not support it
}

/// Represents the time structure.
#[napi]
pub struct Time {
  /// The seconds part of the time.
  pub seconds: i64,
  /// The nanoseconds part of the time.
  pub nseconds: i32,
}

/// Represents the state result.
/// Check https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainState
#[napi]
pub struct StateResult {
  /// The result of the state. 0 if success, 1 if failure.
  pub result: u32,
  /// The reason of the state. It's a flag, Check libvirt documentation for more info.
  pub reason: i32,
}

/// Represents the block info.
#[napi]
pub struct BlockInfo {
  /// Logical size in bytes of the image (how much storage the guest
  /// will see).
  pub capacity: BigInt,
  /// Host storage in bytes occupied by the image (such as highest
  /// allocated extent if there are no holes, similar to 'du').
  pub allocation: BigInt,
  /// Host physical size in bytes of the image container (last
  /// offset, similar to 'ls')
  pub physical: BigInt,
}

#[napi]
pub struct InterfaceStats {
  pub rx_bytes: i64,
  pub rx_packets: i64,
  pub rx_errs: i64,
  pub rx_drop: i64,
  pub tx_bytes: i64,
  pub tx_packets: i64,
  pub tx_errs: i64,
  pub tx_drop: i64,
}

#[napi]
pub struct MemoryStat {
  pub tag: u32,
  pub val: BigInt,
}

#[derive(Clone, Debug, Default)]
#[napi]
pub struct NUMAParameters {
    /// Lists the numa nodeset of a domain.
    pub node_set: Option<String>,
    /// Numa mode of a domain, as an int containing a
    /// DomainNumatuneMemMode value.
    pub mode: Option<u32>,
}

#[napi]
pub struct MemoryParameters {
    /// Represents the maximum memory the guest can use.
    pub hard_limit: Option<BigInt>,
    /// Represents the memory upper limit enforced during memory
    /// contention.
    pub soft_limit: Option<BigInt>,
    /// Represents the minimum memory guaranteed to be reserved for
    /// the guest.
    pub min_guarantee: Option<BigInt>,
    /// Represents the maximum swap plus memory the guest can use.
    pub swap_hard_limit: Option<BigInt>,
}

impl FromNapiValue for MemoryParameters {
  unsafe fn from_napi_value(env: sys::napi_env, napi_val: sys::napi_value) -> Result<Self> {    
    let obj = JsObject::from_napi_value(env, napi_val)?;
    let hard_limit: Option<BigInt> = obj.get("hardLimit")?;
    let soft_limit: Option<BigInt> = obj.get("softLimit")?;
    let min_guarantee: Option<BigInt> = obj.get("minGuarantee")?;
    let swap_hard_limit: Option<BigInt> = obj.get("swapHardLimit")?;

    Ok(Self {
      hard_limit,
      soft_limit,
      min_guarantee,
      swap_hard_limit,
    })
  }
}

impl FromNapiValue for NUMAParameters {
  unsafe fn from_napi_value(env: sys::napi_env, napi_val: sys::napi_value) -> Result<Self> {
    let obj = JsObject::from_napi_value(env, napi_val)?;
    let node_set: Option<String> = obj.get("nodeSet")?;
    let mode: Option<u32> = obj.get("mode")?;
    Ok(Self { node_set, mode })
  }
}

#[napi]
impl Machine {
  pub fn from_domain(domain: Domain, con: &Connection) -> Self {
    Self {
      domain: domain,
      con: con.clone(),
    }
  }

  /// Looks up a domain by its name.
  ///
  /// # Arguments
  ///
  /// * `con` - A reference to the Connection object.
  /// * `name` - A String that holds the name of the domain.
  ///
  /// # Returns
  ///
  /// This function returns:
  /// * `Machine` - If the domain is found.
  /// * `null` - If there is an error during the lookup or the domain is not found.
  ///
  /// You can check `Error::lastError()` to get more information about the error.
  ///
  /// # Example (in JavaScript)
  ///
  /// ```javascript
  /// const { Connection, Machine, Error } = require('your-node-package');
  ///
  /// async function lookupDomain() {
  ///   const conn = await Connection.open('qemu:///system');
  ///   const machine = await Machine.lookupByName(conn, 'your-domain-name');
  ///   if (machine) {
  ///     console.log('Domain found:', machine);
  ///   } else {
  ///     console.error('Error looking up domain:', Error.lastError());
  ///   }
  /// }
  ///
  /// lookupDomain();
  /// ```
  #[napi]
  pub fn lookup_by_name(con: &Connection, name: String) -> Option<Machine> {
    let domain_result = Domain::lookup_by_name(con.get_connection(), &name.to_owned());
    match domain_result {
      Ok(domain) => Some(Self {
        domain,
        con: con.clone(),
      }),
      Err(_) => None
    }
  }

  /// Looks up a domain by its ID.
  ///
  /// # Arguments
  ///
  /// * `conn` - A reference to the Connection object.
  /// * `id` - The ID of the domain.
  ///
  /// # Returns
  ///
  /// This function returns:
  /// * `Machine` - If the domain is found.
  /// * `null` - If there is an error during the lookup or the domain is not found.
  ///
  /// You can check `Error::lastError()` to get more information about the error.
  ///
  /// # Example (in JavaScript)
  ///
  /// ```javascript
  /// const { Connection, Machine, Error } = require('your-node-package');
  ///
  /// async function lookupDomainById() {
  ///   const conn = await Connection.open('qemu:///system');
  ///   const machine = await Machine.lookupById(conn, 1); // Replace 1 with your domain ID
  ///   if (machine) {
  ///     console.log('Domain found:', machine);
  ///   } else {
  ///     console.error('Error looking up domain by ID:', Error.lastError());
  ///   }
  /// }
  ///
  /// lookupDomainById();
  /// ```
  #[napi]
  pub fn lookup_by_id(conn: &crate::connection::Connection, id: u32) -> Option<Machine> {
    let domain_result = Domain::lookup_by_id(conn.get_connection(), id);
    match domain_result {
      Ok(domain) => Some(Self {
        domain,
        con: conn.clone(),
      }),
      Err(_) => None,
    }
  }

  /// Looks up a domain by its UUID.
  ///
  /// # Arguments
  ///
  /// * `conn` - A reference to the Connection object.
  /// * `uuid` - The UUID of the domain as a string.
  ///
  /// # Returns
  ///
  /// This function returns:
  /// * `Machine` - If the domain is found.
  /// * `null` - If there is an error during the lookup or the domain is not found.
  ///
  /// You can check `Error::lastError()` to get more information about the error.
  ///
  /// # Example (in JavaScript)
  ///
  /// ```javascript
  /// const { Connection, Machine, Error } = require('your-node-package');
  ///
  /// async function lookupDomainByUuid() {
  ///   const conn = await Connection.open('qemu:///system');
  ///   const machine = await Machine.lookupByUuidString(conn, 'your-domain-uuid');
  ///   if (machine) {
  ///     console.log('Domain found:', machine);
  ///   } else {
  ///     console.error('Error looking up domain by UUID:', Error.lastError());
  ///   }
  /// }
  ///
  /// lookupDomainByUuid();
  /// ```
  #[napi]
  pub fn lookup_by_uuid_string(
    conn: &crate::connection::Connection,
    uuid: String,
  ) -> Option<Machine> {
    let domain_result = Domain::lookup_by_uuid_string(conn.get_connection(), &uuid);
    match domain_result {
      Ok(domain) => Some(Self {
        domain,
        con: conn.clone(),
      }),
      Err(_) => None,
    }
  }

  /// Get the state of the domain.
  ///
  /// # Returns
  ///
  /// This function returns:
  /// * `StateResult` - If the state is found.
  /// * `null` - If there is an error during the lookup.
  ///
  /// # Example (in JavaScript)
  ///
  /// ```javascript
  /// const { Connection, Machine } = require('your-node-package');
  /// const VIR_DOMAIN_RUNNING = 1;
  ///
  /// async function getDomainState() {
  ///   const conn = await Connection.open('qemu:///system');
  ///   const machine = await Machine.lookupByName(conn, 'your-domain-name');
  ///   const state = machine.getState();
  ///   if (state) {
  ///     if (state.result === VIR_DOMAIN_RUNNING) {
  ///       console.log('Domain is running');
  ///     } else {
  ///       console.log('Domain is not running');
  ///     }
  ///   } else {
  ///     console.error('Error getting domain state');
  ///   }
  /// }
  ///
  /// getDomainState();
  /// ```
  #[napi]
  pub fn get_state(&self) -> Option<StateResult> {
    let state_result = self.domain.get_state();
    match state_result {
      Ok(state) => Some(StateResult {
        result: state.0,
        reason: state.1,
      }),
      Err(_) =>None,
    }
  }

  /// Get the name of the domain.
  ///
  /// # Returns
  ///
  /// This function returns:
  /// * `String` - If the name is found.
  /// * `null` - If there is an error during the lookup.
  ///
  /// # Example (in JavaScript)
  ///
  /// ```javascript
  /// const { Connection, Machine } = require('your-node-package');
  ///
  /// async function getDomainName() {
  ///   const conn = await Connection.open('qemu:///system');
  ///   const machine = await Machine.lookupByName(conn, 'your-domain-name');
  ///   const name = machine.getName();
  ///   if (name) {
  ///     console.log('Domain name:', name);
  ///   } else {
  ///     console.error('Error getting domain name');
  ///   }
  /// }
  ///
  /// getDomainName();
  /// ```
  #[napi]
  pub fn get_name(&self) -> Option<String> {
    let name_result = self.domain.get_name();
    match name_result {
      Ok(name) => Some(name),
      Err(_) => None,
    }
  }

  /// Get the OS type of the domain.
  ///
  /// # Returns
  ///
  /// This function returns:
  /// * `String` - If the OS type is found.
  /// * `null` - If there is an error during the lookup.
  ///
  /// # Example (in JavaScript)
  ///
  /// ```javascript
  /// const { Connection, Machine } = require('your-node-package');
  ///
  /// async function getDomainOsType() {
  ///   const conn = await Connection.open('qemu:///system');
  ///   const machine = await Machine.lookupByName(conn, 'your-domain-name');
  ///   const osType = machine.getOsType();
  ///   if (osType) {
  ///     console.log('Domain OS type:', osType);
  ///   } else {
  ///     console.error('Error getting domain OS type');
  ///   }
  /// }
  ///
  /// getDomainOsType();
  /// ```
  #[napi]
  pub fn get_os_type(&self) -> Option<String> {
    let os_type_result = self.domain.get_os_type();
    match os_type_result {
      Ok(os_type) => Some(os_type),
      Err(_) => None,
    }
  }

  /// Get the hostname of the domain.
  ///
  /// # Returns
  ///
  /// This function returns:
  /// * `String` - If the hostname is found.
  /// * `null` - If there is an error during the lookup.
  ///
  /// # Example (in JavaScript)
  ///
  /// ```javascript
  /// const { Connection, Machine } = require('your-node-package');
  ///
  /// async function getDomainHostname() {
  ///   const conn = await Connection.open('qemu:///system');
  ///   const machine = await Machine.lookupByName(conn, 'your-domain-name');
  ///   const hostname = machine.getHostname();
  ///   if (hostname) {
  ///     console.log('Domain hostname:', hostname);
  ///   } else {
  ///     console.error('Error getting domain hostname');
  ///   }
  /// }
  ///
  /// getDomainHostname();
  /// ```
  #[napi]
  pub fn get_hostname(&self, flags: u32) -> Option<String> {
    let hostname_result = self.domain.get_hostname(flags);
    match hostname_result {
      Ok(hostname) => Some(hostname),
      Err(_) => None,
    }
  }

  /// Get the UUID of the domain.
  ///
  /// # Returns
  ///
  /// This function returns:
  /// * `String` - If the UUID is found.
  /// * `null` - If there is an error during the lookup.
  ///
  /// # Example (in JavaScript)
  ///
  /// ```javascript
  /// const { Connection, Machine } = require('your-node-package');
  ///
  /// async function getDomainUuid() {
  ///   const conn = await Connection.open('qemu:///system');
  ///   const machine = await Machine.lookupByName(conn, 'your-domain-name');
  ///   const uuid = machine.getUuidString();
  ///   if (uuid) {
  ///     console.log('Domain UUID:', uuid);
  ///   } else {
  ///     console.error('Error getting domain UUID');
  ///   }
  /// }
  ///
  /// getDomainUuid();
  /// ```
  #[napi]
  pub fn get_uuid_string(&self) -> Option<String> {
    let uuid_result = self.domain.get_uuid_string();
    match uuid_result {
      Ok(uuid) =>Some(uuid),
      Err(_) => None,
    }
  }

  /// Get the ID of the domain.
  ///
  /// # Returns
  ///
  /// This function returns:
  /// * `u32` - If the ID is found.
  /// * `null` - If there is an error during the lookup or the domain is not running.
  ///
  /// # Example (in JavaScript)
  ///
  /// ```javascript
  /// const { Connection, Machine } = require('your-node-package');
  ///
  /// async function getDomainId() {
  ///   const conn = await Connection.open('qemu:///system');
  ///   const machine = await Machine.lookupByName(conn, 'your-domain-name');
  ///   const id = machine.getId();
  ///   if (id !== null) {
  ///     console.log('Domain ID:', id);
  ///   } else {
  ///     console.error('Error getting domain ID');
  ///   }
  /// }
  ///
  /// getDomainId();
  /// ```
  #[napi]
  pub fn get_id(&self) -> Option<u32> {
    self.domain.get_id()
  }

  /// Get the XML description of the domain.
  ///
  /// # Arguments
  ///
  /// * `flags` - The flags to use for the lookup. Use VirDomainXMLFlags enum.
  ///
  /// # Returns
  ///
  /// This function returns:
  /// * `String` - If the XML description is found.
  /// * `null` - If there is an error during the lookup.
  ///
  /// # Example (in JavaScript)
  ///
  /// ```javascript
  /// const { Connection, Machine } = require('your-node-package');
  ///
  /// async function getDomainXml() {
  ///   const conn = await Connection.open('qemu:///system');
  ///   const machine = await Machine.lookupByName(conn, 'your-domain-name');
  ///   const xml = machine.getXmlDesc(0); // Pass appropriate flags
  ///   if (xml) {
  ///     console.log('Domain XML:', xml);
  ///   } else {
  ///     console.error('Error getting domain XML');
  ///   }
  /// }
  ///
  /// getDomainXml();
  /// ```
  #[napi]
  pub fn get_xml_desc(&self, flags: u32) -> Option<String> {
    match self.domain.get_xml_desc(flags) {
      Ok(xml) => Some(xml),
      Err(_) => None,
    }
  }

  /// Create/power-on the domain.
  ///
  /// # Returns
  ///
  /// This function returns:
  /// * `u32` - If the domain is created successfully, returns the domain ID.
  /// * `null` - If there is an error during the creation.
  ///
  /// # Example (in JavaScript)
  ///
  /// ```javascript
  /// const { Connection, Machine } = require('your-node-package');
  ///
  /// async function createDomain() {
  ///   const conn = await Connection.open('qemu:///system');
  ///   const machine = await Machine.lookupByName(conn, 'your-domain-name');
  ///   const domainId = machine.create();
  ///   if (domainId !== null) {
  ///     console.log('Domain created with ID:', domainId);
  ///   } else {
  ///     console.error('Error creating domain');
  ///   }
  /// }
  ///
  /// createDomain();
  /// ```
  #[napi]
  pub fn create(&self) -> Option<u32> {
    match self.domain.create() {
      Ok(id) => Some(id),
      Err(_) => None,
    }
  }

  /// Create/power-on the domain with flags.
  ///
  /// # Arguments
  ///
  /// * `flags` - The flags to use for the creation. Use VirDomainCreateFlags enum.
  ///
  /// # Returns
  ///
  /// This function returns:
  /// * `u32` - If the domain is created successfully, returns the domain ID.
  /// * `null` - If there is an error during the creation.
  ///
  /// # Example (in JavaScript)
  ///
  /// ```javascript
  /// const { Connection, Machine } = require('your-node-package');
  ///
  /// const VIR_DOMAIN_START_PAUSED = 1;
  ///
  /// async function createDomainWithFlags() {
  ///   const conn = await Connection.open('qemu:///system');
  ///   const machine = await Machine.lookupByName(conn, 'your-domain-name');
  ///   const domainId = machine.createWithFlags(VIR_DOMAIN_START_PAUSED);
  ///   if (domainId !== null) {
  ///     console.log('Domain created with ID:', domainId);
  ///   } else {
  ///     console.error('Error creating domain');
  ///   }
  /// }
  ///
  /// createDomainWithFlags();
  /// ```
  #[napi]
  pub fn create_with_flags(&self, flags: u32) -> Option<u32> {
    match self.domain.create_with_flags(flags) {
      Ok(id) => Some(id),
      Err(_) => None,
    }
  }

  /// Get the information of the domain.
  ///
  /// # Returns
  ///
  /// This function returns:
  /// * `MachineInfo` - If the information is retrieved successfully.
  /// * `null` - If there is an error during the retrieval.
  ///
  /// # Example (in JavaScript)
  ///
  /// ```javascript
  /// const { Connection, Machine } = require('your-node-package');
  ///
  /// async function getDomainInfo() {
  ///   const conn = await Connection.open('qemu:///system');
  ///   const machine = await Machine.lookupByName(conn, 'your-domain-name');
  ///   const info = machine.getInfo();
  ///   if (info) {
  ///     console.log('Domain info:', info);
  ///   } else {
  ///     console.error('Error getting domain info');
  ///   }
  /// }
  ///
  /// getDomainInfo();
  /// ```
  #[napi]
  pub fn get_info(&self) -> Option<MachineInfo> {
    match  self.domain.get_info() {
      Ok(info) => Some(MachineInfo {
        state: info.state,
        max_mem: info.max_mem.into(),
        memory: info.memory.into(),
        nr_virt_cpu: info.nr_virt_cpu,
        cpu_time: info.cpu_time.into(),
      }),
      Err(_) => None,
    }
  }

  /// Create a domain from an XML description.
  ///
  /// # Arguments
  ///
  /// * `conn` - The Connection object to use.
  /// * `xml` - The XML description of the domain.
  /// * `flags` - The flags to use for the creation. Use VirDomainCreateFlags enum.
  ///
  /// # Returns
  ///
  /// This function returns:
  /// * `Machine` - If the domain is created successfully.
  /// * `null` - If there is an error during the creation.
  ///
  /// # Example (in JavaScript)
  ///
  /// ```javascript
  /// const { Connection, Machine } = require('your-node-package');
  ///
  /// async function createDomainFromXml() {
  ///   const conn = await Connection.open('qemu:///system');
  ///   const machine = await Machine.createXml(conn, 'your-domain-xml', 0);
  ///   if (machine) {
  ///     console.log('Domain created:', machine);
  ///   } else {
  ///     console.error('Error creating domain from XML');
  ///   }
  /// }
  ///
  /// createDomainFromXml();
  /// ```
  #[napi]
  pub fn create_xml(conn: &Connection, xml: String, flags: u32) -> Option<Machine> {
    match Domain::create_xml(conn.get_connection(), &xml, flags) {
      Ok(domain) => Some(Machine {
        domain,
        con: conn.clone(),
      }),
      Err(_) => None,
    }
  }

  /// Define a domain from an XML description.
  ///
  /// # Arguments
  ///
  /// * `conn` - The Connection object to use.
  /// * `xml` - The XML description of the domain.
  ///
  /// # Returns
  ///
  /// This function returns:
  /// * `Machine` - If the domain is defined successfully.
  /// * `null` - If there is an error during the definition.
  ///
  /// # Example (in JavaScript)
  ///
  /// ```javascript
  /// const { Connection, Machine } = require('your-node-package');
  ///
  /// async function defineDomainFromXml() {
  ///   const conn = await Connection.open('qemu:///system');
  ///   const machine = await Machine.defineXml(conn, 'your-domain-xml');
  ///   if (machine) {
  ///     console.log('Domain defined successfully:', machine);
  ///   } else {
  ///     console.error('Error defining domain from XML');
  ///   }
  /// }
  ///
  /// defineDomainFromXml();
  /// ```
  #[napi]
  pub fn define_xml(conn: &Connection, xml: String) -> Option<Machine> {
    match Domain::define_xml(conn.get_connection(), &xml) {
      Ok(domain) => Some(Machine {
        domain,
        con: conn.clone(),
      }),
      Err(_) => None,
    }
  }

  /// Define a domain from an XML description with flags.
  ///
  /// # Arguments
  ///
  /// * `conn` - The Connection object to use.
  /// * `xml` - The XML description of the domain.
  /// * `flags` - The flags to use for the definition. Use VirDomainDefineFlags enum.
  ///
  /// # Returns
  ///
  /// This function returns a `Result` which is:
  /// * `Ok(Machine)` - If the domain is defined successfully.
  /// * `Err(napi::Error)` - If there is an error during the definition.
  ///
  /// # Example (in JavaScript)
  ///
  /// ```javascript
  /// const { Connection, Machine, VirDomainDefineFlags } = require('your-node-package');
  ///
  /// async function defineDomainFromXmlWithFlags() {
  ///   const conn = Connection.open('qemu:///system');
  ///   const flags = VirDomainDefineFlags.VIR_DOMAIN_DEFINE_VALIDATE;
  ///   const machine = Machine.defineXmlFlags(conn, 'your-domain-xml', flags);
  ///   console.log('Domain defined successfully with flags');
  /// }
  ///
  /// defineDomainFromXmlWithFlags().catch(console.error);
  /// ```
  #[napi]
  pub fn define_xml_flags(conn: &Connection, xml: String, flags: u32) -> Option<Machine> {
    match Domain::define_xml_flags(conn.get_connection(), &xml, flags) {
      Ok(domain) => Some(Machine {
        domain,
        con: conn.clone(),
      }),
      Err(_) => None,
    }
  }

  /// Destroy/power-off the domain.
  ///
  /// # Returns
  ///
  /// This function returns a `Result` which is:
  /// * `Ok(())` - If the domain is destroyed.
  /// * `Err(napi::Error)` - If there is an error during the destruction.
  ///
  /// # Example (in JavaScript)
  ///
  /// ```javascript
  /// const { Connection, Machine } = require('your-node-package');
  ///
  /// async function destroyDomain() {
  ///   const conn = Connection.open('quemu:///system');
  ///   const machine = await Machine.lookupByName(conn, 'your-domain-name');
  ///   await machine.destroy();
  /// }
  ///
  /// destroyDomain();
  /// ```
  #[napi]
  pub fn destroy(&self) -> Option<()> {
    match self.domain.destroy() {
      Ok(_) => Some(()),
      Err(_) => None,
    }
  }

  /// Reset the domain.
  ///
  /// # Returns
  ///
  /// This function returns a `Result` which is:
  /// * `Ok(u32)` - If the domain is reset.
  /// * `Err(napi::Error)` - If there is an error during the reset.
  ///
  /// # Example (in JavaScript)
  ///
  /// ```javascript
  /// const { Connection, Machine } = require('your-node-package');
  ///
  /// async function resetDomain() {
  ///   const conn = Connection.open('quemu:///system');
  ///   const machine = Machine.lookupByName(conn, 'your-domain-name');
  ///   await machine.reset();
  /// }
  ///
  /// resetDomain();
  /// ```
  #[napi]
  pub fn reset(&self) -> Option<u32> {
    match self.domain.reset() {
      Ok(id) => Some(id),
      Err(_) => None,
    }
  }

  /// Destroy/power-off the domain with flags.
  ///
  /// # Arguments
  ///
  /// * `flags` - The flags to use for the destruction. Use VirDomainDestroyFlags enum
  ///
  /// # Returns
  ///
  /// This function returns a `Result` which is:
  /// * `Ok(u32)` - If the domain is destroyed.
  /// * `Err(napi::Error)` - If there is an error during the destruction.
  ///
  /// # Example (in JavaScript)
  ///
  /// ```javascript
  /// const { Connection, Machine } = require('your-node-package');
  /// 
  /// VIR_DOMAIN_DESTROY_GRACEFUL = 1 (0x1; 1 << 0)
  ///
  /// async function destroyDomain() {
  ///   const conn = Connection.open('quemu:///system');
  ///   const machine = await Machine.lookupByName(conn, 'your-domain-name');
  ///   await machine.destroyFlags(VIR_DOMAIN_DESTROY_GRACEFUL);
  /// }
  ///
  /// destroyDomain();
  /// ```
  #[napi]
  pub fn destroy_flags(&self, flags: u32) -> Option<u32> {
    match self.domain.destroy_flags(flags) {
      Ok(id) => Some(id),
      Err(_) => None,
    }
  }

  /// Shutdown the domain.
  ///
  /// # Returns
  ///
  /// This function returns a `Result` which is:
  /// * `Ok(u32)` - If the domain is shutdown.
  /// * `Err(napi::Error)` - If there is an error during the shutdown.
  ///
  /// # Example (in JavaScript)
  ///
  /// ```javascript
  /// const { Connection, Machine } = require('your-node-package');
  ///
  /// async function shutdownDomain() {
  ///   const conn = Connection.open('quemu:///system');
  ///   const machine = await Machine.lookupByName(conn, 'your-domain-name');
  ///   await machine.shutdown();
  /// }
  ///
  /// shutdownDomain();
  /// ```
  #[napi]
  pub fn shutdown(&self) -> Option<u32> {
    match self.domain.shutdown() {
      Ok(id) => Some(id),
      Err(_) => None,
    }
  }

  /// Reboot the domain with flags.
  /// Useful if you want to send ACPI events to the domain.
  ///
  /// # Arguments
  ///
  /// * `flags` - The flags to use for the reboot. Use VirDomainRebootFlag enum
  ///
  /// # Returns
  ///
  /// This function returns a `Result` which is:
  /// * `Ok(())` - If the domain is rebooted.
  /// * `Err(napi::Error)` - If there is an error during the reboot.
  ///
  /// # Example (in JavaScript)
  ///
  /// ```javascript
  /// const { Connection, Machine } = require('your-node-package');
  /// 
  /// const VIR_DOMAIN_REBOOT_ACPI_POWER_BTN = 1 (0x1; 1 << 1)
  ///
  /// async function rebootDomain() {
  ///   const conn = Connection.open('quemu:///system');
  ///   const machine = await Machine.lookupByName(conn, 'your-domain-name');
  ///   await machine.reboot(VIR_DOMAIN_REBOOT_ACPI_POWER_BTN);
  /// }
  ///
  /// rebootDomain();
  /// ```
  #[napi]
  pub fn reboot(&self, flags: u32) -> Option<()> {
    match self.domain.reboot(flags) {
      Ok(_) => Some(()),
      Err(_) => None,
    }
  }

  /// Suspend the domain.
  /// When machine is suspended, the process is frozen without further access to 
  /// CPU resources and I/O but the memory used by the domain at the hypervisor level 
  /// will stay allocated. 
  ///
  /// # Returns
  ///
  /// This function returns a `Result` which is:
  /// * `Ok(u32)` - If the domain is suspended.
  /// * `Err(napi::Error)` - If there is an error during the suspension.
  ///
  /// # Example (in JavaScript)
  ///
  /// ```javascript
  /// const { Connection, Machine } = require('your-node-package');
  ///
  /// async function suspendDomain() {
  ///   const conn = Connection.open('quemu:///system');
  ///   const machine = await Machine.lookupByName(conn, 'your-domain-name');
  ///   await machine.suspend();
  /// }
  ///
  /// suspendDomain();
  /// ```
  #[napi]
  pub fn suspend(&self) -> Option<u32> {
    match self.domain.suspend() {
      Ok(id) => Some(id),
      Err(_) => None,
    }
  }

  /// Resume the suspended domain.
  ///
  /// # Returns
  ///
  /// This function returns a `Result` which is:
  /// * `Ok(u32)` - If the domain is resumed.
  /// * `Err(napi::Error)` - If there is an error during the resumption.
  ///
  /// # Example (in JavaScript)
  ///
  /// ```javascript
  /// const { Connection, Machine } = require('your-node-package');
  ///
  /// async function resumeDomain() {
  ///   const conn = Connection.open('quemu:///system');
  ///   const machine = await Machine.lookupByName(conn, 'your-domain-name');
  ///   await machine.resume();
  /// }
  ///
  /// resumeDomain();
  /// ```
  #[napi]
  pub fn resume(&self) -> Option<u32> {
    match self.domain.resume() {
      Ok(id) => Some(id),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn is_active(&self) -> Option<bool> {
    match self.domain.is_active() {
      Ok(active) => Some(active),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn undefine(&self) -> Option<u32> {
    match self.domain.undefine() {
      Ok(_) => Some(0),
      Err(_) => None,
    }
  }

  ///
  /// # Arguments
  ///
  /// * `flags` - The flags to use for the undefinition. Use VirDomainUndefineFlags enum
  #[napi]
  pub fn undefine_flags(&self, flags: u32) -> Option<u32> {
    match self.domain.undefine_flags(flags) {
      Ok(_) => Some(0),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn free(&mut self) -> Option<u32> {
    match self.domain.free() {
      Ok(_) => Some(0),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn is_updated(&self) -> Option<bool> {
    match self.domain.is_updated() {
      Ok(updated) => Some(updated),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn get_autostart(&self) -> Option<bool> {
    match self.domain.get_autostart() {
      Ok(autostart) => Some(autostart),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn set_autostart(&self, autostart: bool) -> Option<bool> {
    match self.domain.set_autostart(autostart) {
      Ok(result) => Some(result),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn set_max_memory(&self, memory: BigInt) -> Option<bool> {
    let (_signed, memory_u64, lossless) = memory.get_u64();
    if !lossless {
      return None;
    }
    let result = self.domain.set_max_memory(memory_u64);
    match result {
      Ok(result) => Some(result),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn get_max_vcpus(&self) -> Option<u64> {
    match self.domain.get_max_vcpus() {
      Ok(vcpus) => Some(vcpus),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn set_memory(&self, memory: BigInt) -> Option<bool> {
    let (_signed, memory_u64, lossless) = memory.get_u64();
    if !lossless {
      return None;
    }
    let result = self.domain.set_memory(memory_u64);
    match result {
      Ok(result) => Some(result),
      Err(_) => None,
    }
  }

  ///
  /// # Arguments
  ///
  /// * `flags` - The flags to use for the memory modification. Use VirDomainMemoryModFlags enum
  #[napi]
  pub fn set_memory_flags(&self, memory: BigInt, flags: u32) -> Option<bool> {
    let (_signed, memory_u64, lossless) = memory.get_u64();
    if !lossless {
      return None;
    }
    let result = self.domain.set_memory_flags(memory_u64, flags as u32);
    match result {
      Ok(result) => Some(result),
      Err(_) => None,
    }
  }

  ///
  /// # Arguments
  ///
  /// * `flags` - The flags to use for the memory modification. Use VirDomainMemoryModFlags enum
  #[napi]
  pub fn set_memory_stats_period(&self, period: i32, flags: u32) -> Option<bool> {
    match self.domain.set_memory_stats_period(period, flags) {
      Ok(result) => Some(result),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn set_vcpus(&self, vcpus: u32) -> Option<bool> {
    match self.domain.set_vcpus(vcpus) {
      Ok(result) => Some(result),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn set_vcpus_flags(&self, vcpus: u32, flags: u32) -> Option<bool> {
    match self.domain.set_vcpus_flags(vcpus, flags) {
      Ok(result) => Some(result),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn domain_restore(conn: &Connection, path: String) -> Option<u32> {
    match Domain::domain_restore(conn.get_connection(), &path) {
      Ok(_) => Some(0),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn domain_restore_flags(conn: &Connection, path: String, flags: u32) -> Option<u32> {
    match  Domain::domain_restore_flags(conn.get_connection(), &path, None, flags) {
      Ok(_) => Some(0),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn get_vcpus_flags(&self, flags: u32) -> Option<u32> {
    match self.domain.get_vcpus_flags(flags) {
      Ok(vcpus) => Some(vcpus),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn migrate_set_max_speed(&self, bandwidth: BigInt, flags: u32) -> Option<u32> {
    let (_signed, bandwidth_u64, lossless) = bandwidth.get_u64();
    if !lossless {
      return None;
    }
    let result = self.domain.migrate_set_max_speed(bandwidth_u64, flags);
    match result {
      Ok(result) => Some(result),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn migrate_get_max_speed(&self, flags: u32) -> Option<u64> {
    match self.domain.migrate_get_max_speed(flags) {
      Ok(speed) => Some(speed),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn migrate_set_compression_cache(&self, size: BigInt, flags: u32) -> Option<u32> {
    let (_signed, size_u64, lossless) = size.get_u64();
    if !lossless {
      return None;
    }
    let result = self.domain.migrate_set_compression_cache(size_u64, flags);
    match result {
      Ok(result) => Some(result),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn migrate_get_compression_cache(&self, flags: u32) -> Option<u64> {
    match self.domain.migrate_get_compression_cache(flags) {
      Ok(cache) => Some(cache),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn migrate_set_max_downtime(&self, downtime: BigInt, flags: u32) -> Option<u32> {
    let (_signed, downtime_u64, lossless) = downtime.get_u64();
    if !lossless {
      return None;
    }
    let result = self.domain.migrate_set_max_downtime(downtime_u64, flags);
    match result {
      Ok(result) => Some(result),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn set_time(&self, seconds: i64, nseconds: i32, flags: u32) -> Option<u32> {
    match self.domain.set_time(seconds, nseconds, flags) {
      Ok(result) => Some(result),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn get_time(&self, flags: u32) -> Option<Time> {
    match self.domain.get_time(flags) {
      Ok(result) => Some(Time {
        seconds: result.0,
        nseconds: result.1,
      }),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn get_block_info(&self, disk: String, flags: u32) -> Option<BlockInfo> {
    match self.domain.get_block_info(&disk, flags) {
      Ok(result) => Some(BlockInfo {
        capacity: result.capacity.into(),
        allocation: result.allocation.into(),
        physical: result.physical.into(),
      }),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn pin_vcpu(&self, vcpu: u32, cpumap: &[u8]) -> Option<u32> {
    match self.domain.pin_vcpu(vcpu, cpumap) {
      Ok(result) => Some(result),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn pin_vcpu_flags(&self, vcpu: u32, cpumap: &[u8], flags: u32) -> Option<u32> {
    match self.domain.pin_vcpu_flags(vcpu, cpumap, flags) {
      Ok(result) => Some(result),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn pin_emulator(&self, cpumap: &[u8], flags: u32) -> Option<u32> {
    match self.domain.pin_emulator(cpumap, flags) {
      Ok(result) => Some(result),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn rename(&self, new_name: String, flags: u32) -> Option<u32> {
    match self.domain.rename(&new_name, flags) {
      Ok(result) => Some(result),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn set_user_password(&self, user: String, password: String, flags: u32) -> Option<u32> {
    match self.domain.set_user_password(&user, &password, flags) {
      Ok(result) => Some(result),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn set_block_threshold(&self, dev: String, threshold: BigInt, flags: u32) -> Option<u32> {
    let (_signed, threshold_u64, lossless) = threshold.get_u64();
    if !lossless {
      return None;
    }
    let result = self.domain.set_block_threshold(&dev, threshold_u64, flags);
    match result {
      Ok(result) => Some(result),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn open_graphics(&self, idx: u32, fd: i32, flags: u32) -> Option<u32> {
    match self.domain.open_graphics(idx, fd, flags) {
      Ok(result) => Some(result),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn open_graphics_fd(&self, idx: u32, flags: u32) -> Option<u32> {
    match self.domain.open_graphics_fd(idx, flags) {
      Ok(result) => Some(result),
      Err(_) => None,
    }
  }

  // TODO: implement open_channel, we need to check Stream struct and implement tons of things
  // before being able to implement this method
//   #[napi]
//   pub fn open_channel(&self, name: &str, stream: &Stream, flags: u32) -> Result<u32, Error> {
    // pub fn open_console(&self, name: &str, stream: &Stream, flags: u32) -> Result<u32, Error> {

//   #[napi]
//   pub fn interface_addresses(
//     &self,
//     source: u32,
//     flags: u32,
//   ) -> napi::Result<Vec<crate::interface::Interface>> {
  
  #[napi]
  pub fn interface_stats(&self, path: String) -> Option<InterfaceStats> {
    match self.domain.interface_stats(&path) {
      Ok(stats) => Some(InterfaceStats {
        rx_bytes: stats.rx_bytes,
        rx_packets: stats.rx_packets,
        rx_errs: stats.rx_errs,
        rx_drop: stats.rx_drop,
        tx_bytes: stats.tx_bytes,
        tx_packets: stats.tx_packets,
        tx_errs: stats.tx_errs,
        tx_drop: stats.tx_drop,
      }),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn memory_stats(&self, flags: u32) -> Option<Vec<MemoryStat>> {
    match self.domain.memory_stats(flags) {
      Ok(stats) => {
        let mut memory_stats = Vec::new();
        for stat in stats {
          memory_stats.push(MemoryStat {
            tag: stat.tag,
            val: stat.val.into(),
          });
        }
        Some(memory_stats)
      },
      Err(_) => None,
    }
  }

  #[napi]
  pub fn save_image_get_xml_desc(
    conn: &Connection,
    file: String,
    flags: u32,
  ) -> Option<String> {
    match Domain::save_image_get_xml_desc(conn.get_connection(), &file, flags) {
      Ok(result) => Some(result),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn save_image_define_xml(
    conn: &Connection,
    file: String,
    dxml: String,
    flags: u32,
  ) -> Option<u32> {
    match Domain::save_image_define_xml(conn.get_connection(), &file, &dxml, flags) {
      Ok(result) => Some(result),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn attach_device(&self, xml: String) -> Option<u32> {
    match self.domain.attach_device(&xml) {
      Ok(result) => Some(result),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn attach_device_flags(&self, xml: String, flags: u32) -> Option<u32> {
    match self.domain.attach_device_flags(&xml, flags) {
      Ok(result) => Some(result),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn detach_device(&self, xml: String) -> Option<u32> {
    match self.domain.detach_device(&xml) {
      Ok(result) => Some(result),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn detach_device_flags(&self, xml: String, flags: u32) -> Option<u32> {
    match self.domain.detach_device_flags(&xml, flags) {
      Ok(result) => Some(result),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn update_device_flags(&self, xml: String, flags: u32) -> Option<u32> {
    match self.domain.update_device_flags(&xml, flags) {
      Ok(result) => Some(result),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn managed_save(&self, flags: u32) -> Option<u32> {
    match self.domain.managed_save(flags) {
      Ok(result) => Some(result),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn has_managed_save(&self, flags: u32) -> Option<bool> {
    match self.domain.has_managed_save(flags) {
      Ok(result) => Some(result),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn managed_save_remove(&self, flags: u32) -> Option<u32> {
    match self.domain.managed_save_remove(flags) {
      Ok(result) => Some(result),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn core_dump(&self, to: String, flags: u32) -> Option<u32> {
    match self.domain.core_dump(&to, flags) {
      Ok(result) => Some(result),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn core_dump_with_format(&self, to: String, format: u32, flags: u32) -> Option<u32> {
    match self.domain.core_dump_with_format(&to, format, flags) {
      Ok(result) => Some(result),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn set_metadata(
    &self,
    kind: i32,
    metadata: String,
    key: String,
    uri: String,
    flags: u32,
  ) -> Option<u32> {
    match self.domain.set_metadata(kind, Some(&metadata), Some(&key), Some(&uri), flags) {
      Ok(result) => Some(result),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn get_metadata(&self, kind: i32, uri: String, flags: u32) -> Option<String> {
    match self.domain.get_metadata(kind, Some(&uri), flags) {
      Ok(result) => Some(result),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn block_resize(&self, disk: String, size: BigInt, flags: u32) -> Option<u32> {
    let (_signed, size_u64, lossless) = size.get_u64();
    if !lossless {
      return None;
    }
    match self.domain.block_resize(&disk, size_u64, flags) {
      Ok(result) => Some(result),
      Err(_) => None,
    }
  }

 #[napi]
 pub fn get_memory_parameters(&self, flags: u32) -> Option<MemoryParameters> {
    match self.domain.get_memory_parameters(flags) {
      Ok(result) => Some(MemoryParameters {
        hard_limit: result.hard_limit.map(|v| BigInt::from(v)),
        soft_limit: result.soft_limit.map(|v| BigInt::from(v)),
        min_guarantee: result.min_guarantee.map(|v| BigInt::from(v)),
        swap_hard_limit: result.swap_hard_limit.map(|v| BigInt::from(v)),
      }),
      Err(_) => None,
    }
 }

 #[napi]
 pub fn set_memory_parameters(
  &self,
  params: crate::machine::MemoryParameters,
  flags: u32,
 ) -> Option<u32> {
    // TODO: Check params overflow, it should be u64 but BigInt is used because u64 is not supported by N-API
    let mem_param: virt::domain::MemoryParameters = virt::domain::MemoryParameters {
      hard_limit: params.hard_limit.map(|v| v.get_u64().1),
      soft_limit: params.soft_limit.map(|v| v.get_u64().1),
      min_guarantee: params.min_guarantee.map(|v| v.get_u64().1),
      swap_hard_limit: params.swap_hard_limit.map(|v| v.get_u64().1),
    };
    match self.domain.set_memory_parameters(mem_param, flags) {
      Ok(result) => Some(result),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn migrate(
    &self,
    dconn: &Connection,
    flags: u32,
    uri: String,
    bandwidth: BigInt,
  ) -> Option<Machine> {
    let (_signed, bandwidth_u64, lossless) = bandwidth.get_u64();
    if !lossless {
      return None;
    }
    match self.domain.migrate(dconn.get_connection(), flags, None, Some(&uri), bandwidth_u64) {
      Ok(result) => Some(Machine::from_domain(result, &dconn)),
      Err(_) => None,
    }
  }

  // Renamed, originally called migrate2
  #[napi]
  pub fn migrate_with_xml(
    &self,
    dconn: &Connection,
    dxml: String,
    flags: u32,
    uri: String,
    bandwidth: BigInt,
  ) -> Option<Machine> {
    let (_signed, bandwidth_u64, lossless) = bandwidth.get_u64();
    if !lossless {
      return None;
    }
    match self.domain.migrate2(dconn.get_connection(), Some(&dxml), flags, None, Some(&uri), bandwidth_u64) {
      Ok(result) => Some(Machine::from_domain(result, &dconn)),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn migrate_to_uri(&self, uri: String, flags: u32, bandwidth: BigInt) -> Option<u32> {
    let (_signed, bandwidth_u64, lossless) = bandwidth.get_u64();
    if !lossless {
      return None;
    }
    match self.domain.migrate_to_uri(&uri, flags, Some(""), bandwidth_u64) {
      Ok(_) => Some(0),
      Err(_) => None,
    }
  }

  // Renamed, originally called migrate_to_uri2
  #[napi]
  pub fn migrate_to_uri_with_xml(
    &self,
    dconn_uri: String,
    mig_uri: String,
    dxml: String,
    flags: u32,
    bandwidth: BigInt,
  ) -> Option<u32> {
    let (_signed, bandwidth_u64, lossless) = bandwidth.get_u64();
    if !lossless {
      return None;
    }
    match self.domain.migrate_to_uri2(Some(&dconn_uri), Some(&mig_uri), Some(&dxml), flags, None, bandwidth_u64) {
      Ok(_) => Some(0),
      Err(_) =>None,
    }
  }

  #[napi]
  pub fn get_numa_parameters(&self, flags: u32) -> Option<crate::machine::NUMAParameters> {
    match self.domain.get_numa_parameters(flags) {
      Ok(result) => Some(NUMAParameters {
        node_set: result.node_set.map(|v| v.to_string()),
        mode: result.mode.map(|v| v as u32),
      }),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn set_numa_parameters(&self, params: crate::machine::NUMAParameters, flags: u32) -> Option<u32> {
    let params: virt::domain::NUMAParameters = virt::domain::NUMAParameters {
      node_set: params.node_set.map(|v| v.to_string()),
      mode: params.mode.map(|v| v as i32),
    };
    match self.domain.set_numa_parameters(params, flags) {
      Ok(result) => Some(result),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn qemu_agent_command(&self, cmd: String, timeout: i32, flags: u32) -> Option<String> {
    match self.domain.qemu_agent_command(&cmd, timeout, flags) {
      Ok(result) => Some(result),
      Err(_) => None,
    }
  }

  #[napi]
  pub fn qemu_monitor_command(&self, cmd: String, flags: u32) -> Option<String> {
    match self.domain.qemu_monitor_command(&cmd, flags) {
      Ok(result) => Some(result),
      Err(_) => None,
    }
  }
}
