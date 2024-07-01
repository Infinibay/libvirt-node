use napi::{
  bindgen_prelude::BigInt, CallContext, Env, JsBoolean, JsObject, JsString, JsUndefined, Property,
  Result as NapiResult,
};

use virt::{connect::Connect, domain::Domain};

use napi::{bindgen_prelude::*, Error as NapiError};

// Add them when we need them
// use log::{error, info, warn};
use napi_derive::js_function;

use crate::connection::Connection;

#[napi]
pub struct Machine {
  domain: Domain,
  con: Connection,
}

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

#[napi]
pub struct Time {
  pub seconds: i64,
  pub nseconds: i32,
}

#[napi]
pub struct StateResult {
  pub result: u32,
  pub reason: i32,
}

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
impl Machine {
  pub fn from_domain(domain: Domain, con: &Connection) -> Self {
    Self {
      domain: domain,
      con: con.clone(),
    }
  }

  #[napi]
  pub fn lookup_by_name(name: String, con: &Connection) -> napi::Result<Self> {
    let domain_result = Domain::lookup_by_name(con.get_connection(), &name.to_owned());
    match domain_result {
      Ok(domain) => Ok(Self {
        domain: domain,
        con: con.clone(),
      }),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn lookup_by_id(conn: &crate::connection::Connection, id: u32) -> napi::Result<Machine> {
    let domain_result = Domain::lookup_by_id(conn.get_connection(), id);
    match domain_result {
      Ok(domain) => Ok(Self {
        domain: domain,
        con: conn.clone(),
      }),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn lookup_by_uuid_string(
    conn: &crate::connection::Connection,
    uuid: String,
  ) -> napi::Result<Machine> {
    let domain_result = Domain::lookup_by_uuid_string(conn.get_connection(), &uuid);
    match domain_result {
      Ok(domain) => Ok(Self {
        domain: domain,
        con: conn.clone(),
      }),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn get_state(&self) -> napi::Result<StateResult> {
    let state_result = self.domain.get_state();
    match state_result {
      Ok(state) => Ok(StateResult {
        result: state.0,
        reason: state.1,
      }),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn get_name(&self) -> napi::Result<String> {
    let name_result = self.domain.get_name();
    match name_result {
      Ok(name) => Ok(name),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn get_os_type(&self) -> napi::Result<String> {
    let os_type_result = self.domain.get_os_type();
    match os_type_result {
      Ok(os_type) => Ok(os_type),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn get_hostname(&self, flags: u32) -> napi::Result<String> {
    let hostname_result = self.domain.get_hostname(flags);
    match hostname_result {
      Ok(hostname) => Ok(hostname),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn get_uuid_string(&self) -> napi::Result<String> {
    let uuid_result = self.domain.get_uuid_string();
    match uuid_result {
      Ok(uuid) => Ok(uuid),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn get_id(&self) -> Option<u32> {
    self.domain.get_id()
  }

  #[napi]
  pub fn get_xml_desc(&self, flags: u32) -> napi::Result<String> {
    let xml_result = self.domain.get_xml_desc(flags);
    match xml_result {
      Ok(xml) => Ok(xml),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn create(&self) -> napi::Result<u32> {
    let id_result = self.domain.create();
    match id_result {
      Ok(id) => Ok(id),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn create_with_flags(&self, flags: u32) -> napi::Result<u32> {
    let id_result = self.domain.create_with_flags(flags);
    match id_result {
      Ok(id) => Ok(id),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn get_info(&self) -> napi::Result<MachineInfo> {
    let info_result = self.domain.get_info();
    match info_result {
      Ok(info) => Ok(MachineInfo {
        state: info.state,
        max_mem: info.max_mem.into(),
        memory: info.memory.into(),
        nr_virt_cpu: info.nr_virt_cpu,
        cpu_time: info.cpu_time.into(),
      }),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn create_xml(conn: &Connection, xml: String, flags: u32) -> napi::Result<Machine> {
    let domain_result = Domain::create_xml(conn.get_connection(), &xml, flags);
    match domain_result {
      Ok(domain) => Ok(Self {
        domain: domain,
        con: conn.clone(),
      }),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn define_xml(conn: &Connection, xml: String) -> napi::Result<Machine> {
    let domain_result = Domain::define_xml(conn.get_connection(), &xml);
    match domain_result {
      Ok(domain) => Ok(Self {
        domain: domain,
        con: conn.clone(),
      }),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn define_xml_flags(conn: &Connection, xml: String, flags: u32) -> napi::Result<Machine> {
    let domain_result = Domain::define_xml_flags(conn.get_connection(), &xml, flags);
    match domain_result {
      Ok(domain) => Ok(Self {
        domain: domain,
        con: conn.clone(),
      }),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn destroy(&self) -> napi::Result<()> {
    let result = self.domain.destroy();
    match result {
      Ok(_) => Ok(()),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn reset(&self) -> napi::Result<u32> {
    let result = self.domain.reset();
    match result {
      Ok(result) => Ok(result),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn destroy_flags(&self, flags: u32) -> napi::Result<u32> {
    let result = self.domain.destroy_flags(flags);
    match result {
      Ok(result) => Ok(result),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn shutdown(&self) -> napi::Result<u32> {
    let result = self.domain.shutdown();
    match result {
      Ok(result) => Ok(result),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn reboot(&self, flags: u32) -> napi::Result<()> {
    let result = self.domain.reboot(flags);
    match result {
      Ok(_) => Ok(()),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn suspend(&self) -> napi::Result<u32> {
    let result = self.domain.suspend();
    match result {
      Ok(result) => Ok(result),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn resume(&self) -> napi::Result<u32> {
    let result = self.domain.resume();
    match result {
      Ok(result) => Ok(result),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn is_active(&self) -> napi::Result<bool> {
    let result = self.domain.is_active();
    match result {
      Ok(result) => Ok(result),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn undefine(&self) -> napi::Result<()> {
    let result = self.domain.undefine();
    match result {
      Ok(_) => Ok(()),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn undefine_flags(&self, flags: u32) -> napi::Result<()> {
    let result = self.domain.undefine_flags(flags);
    match result {
      Ok(_) => Ok(()),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn free(&mut self) -> napi::Result<()> {
    let result = self.domain.free();
    match result {
      Ok(_) => Ok(()),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn is_updated(&self) -> napi::Result<bool> {
    let result = self.domain.is_updated();
    match result {
      Ok(result) => Ok(result),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn get_autostart(&self) -> napi::Result<bool> {
    let result = self.domain.get_autostart();
    match result {
      Ok(result) => Ok(result),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn set_autostart(&self, autostart: bool) -> napi::Result<bool> {
    let result = self.domain.set_autostart(autostart);
    match result {
      Ok(result) => Ok(result),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn set_max_memory(&self, memory: BigInt) -> napi::Result<bool> {
    let memory_u64: u64 = memory.get_u64().1; // WARNING we are ignoring signed overflow here
    let result = self.domain.set_max_memory(memory_u64);
    match result {
      Ok(result) => Ok(result),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn get_max_vcpus(&self) -> napi::Result<u64> {
    let result = self.domain.get_max_vcpus();
    match result {
      Ok(result) => Ok(result),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn set_memory(&self, memory: BigInt) -> napi::Result<bool> {
    let memory_u64: u64 = memory.get_u64().1; // WARNING we are ignoring signed overflow here
    let result = self.domain.set_memory(memory_u64);
    match result {
      Ok(result) => Ok(result),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn set_memory_flags(&self, memory: BigInt, flags: u32) -> napi::Result<bool> {
    let memory_u64: u64 = memory.get_u64().1; // WARNING we are ignoring signed overflow here
    let result = self.domain.set_memory_flags(memory_u64, flags);
    match result {
      Ok(result) => Ok(result),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn set_memory_stats_period(&self, period: i32, flags: u32) -> napi::Result<bool> {
    let result = self.domain.set_memory_stats_period(period, flags);
    match result {
      Ok(result) => Ok(result),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn set_vcpus(&self, vcpus: u32) -> napi::Result<bool> {
    let result = self.domain.set_vcpus(vcpus);
    match result {
      Ok(result) => Ok(result),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn set_vcpus_flags(&self, vcpus: u32, flags: u32) -> napi::Result<bool> {
    let result = self.domain.set_vcpus_flags(vcpus, flags);
    match result {
      Ok(result) => Ok(result),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn domain_restore(conn: &Connection, path: String) -> napi::Result<()> {
    let result = Domain::domain_restore(conn.get_connection(), &path);
    match result {
      Ok(_) => Ok(()),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn domain_restore_flags(conn: &Connection, path: String, flags: u32) -> napi::Result<()> {
    let result = Domain::domain_restore_flags(conn.get_connection(), &path, flags);
    match result {
      Ok(_) => Ok(()),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn get_vcpus_flags(&self, flags: u32) -> napi::Result<u32> {
    let result = self.domain.get_vcpus_flags(flags);
    match result {
      Ok(result) => Ok(result),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn migrate_set_max_speed(&self, bandwidth: BigInt, flags: u32) -> napi::Result<u32> {
    let bandwidth_u64: u64 = bandwidth.get_u64().1; // WARNING we are ignoring signed overflow here
    let result = self.domain.migrate_set_max_speed(bandwidth_u64, flags);
    match result {
      Ok(result) => Ok(result),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn migrate_get_max_speed(&self, flags: u32) -> napi::Result<u64> {
    let result = self.domain.migrate_get_max_speed(flags);
    match result {
      Ok(result) => Ok(result),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn migrate_set_compression_cache(&self, size: BigInt, flags: u32) -> napi::Result<u32> {
    let size_u64: u64 = size.get_u64().1; // WARNING we are ignoring signed overflow here
    let result = self.domain.migrate_set_compression_cache(size_u64, flags);
    match result {
      Ok(result) => Ok(result),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn migrate_get_compression_cache(&self, flags: u32) -> napi::Result<u64> {
    let result = self.domain.migrate_get_compression_cache(flags);
    match result {
      Ok(result) => Ok(result),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn migrate_set_max_downtime(&self, downtime: BigInt, flags: u32) -> napi::Result<u32> {
    let downtime_u64: u64 = downtime.get_u64().1; // WARNING we are ignoring signed overflow here
    let result = self.domain.migrate_set_max_downtime(downtime_u64, flags);
    match result {
      Ok(result) => Ok(result),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn set_time(&self, seconds: i64, nseconds: i32, flags: u32) -> napi::Result<u32> {
    let result = self.domain.set_time(seconds, nseconds, flags);
    match result {
      Ok(result) => Ok(result),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn get_time(&self, flags: u32) -> napi::Result<Time> {
    let result = self.domain.get_time(flags);
    match result {
      Ok(result) => Ok(Time {
        seconds: result.0,
        nseconds: result.1,
      }),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn get_block_info(&self, disk: String, flags: u32) -> napi::Result<BlockInfo> {
    let result = self.domain.get_block_info(&disk, flags);
    match result {
      Ok(result) => Ok(BlockInfo {
        capacity: result.capacity.into(),
        allocation: result.allocation.into(),
        physical: result.physical.into(),
      }),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn pin_vcpu(&self, vcpu: u32, cpumap: &[u8]) -> napi::Result<u32> {
    let result = self.domain.pin_vcpu(vcpu, cpumap);
    match result {
      Ok(result) => Ok(result),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn pin_vcpu_flags(&self, vcpu: u32, cpumap: &[u8], flags: u32) -> napi::Result<u32> {
    let result = self.domain.pin_vcpu_flags(vcpu, cpumap, flags);
    match result {
      Ok(result) => Ok(result),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn pin_emulator(&self, cpumap: &[u8], flags: u32) -> napi::Result<u32> {
    let result = self.domain.pin_emulator(cpumap, flags);
    match result {
      Ok(result) => Ok(result),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn rename(&self, new_name: String, flags: u32) -> napi::Result<u32> {
    let result = self.domain.rename(&new_name, flags);
    match result {
      Ok(result) => Ok(result),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn set_user_password(&self, user: String, password: String, flags: u32) -> napi::Result<u32> {
    let result = self.domain.set_user_password(&user, &password, flags);
    match result {
      Ok(result) => Ok(result),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn set_block_threshold(&self, dev: String, threshold: BigInt, flags: u32) -> napi::Result<u32> {
    let threshold_u64: u64 = threshold.get_u64().1; // WARNING we are ignoring signed overflow here
    let result = self.domain.set_block_threshold(&dev, threshold_u64, flags);
    match result {
      Ok(result) => Ok(result),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn open_graphics(&self, idx: u32, fd: i32, flags: u32) -> napi::Result<u32> {
    let result = self.domain.open_graphics(idx, fd, flags);
    match result {
      Ok(result) => Ok(result),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  #[napi]
  pub fn open_graphics_fd(&self, idx: u32, flags: u32) -> napi::Result<u32> {
    let result = self.domain.open_graphics_fd(idx, flags);
    match result {
      Ok(result) => Ok(result),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
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

    
}
