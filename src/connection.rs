use napi::{
    bindgen_prelude::{FromNapiValue, ToNapiValue}, CallContext, Env, JsBoolean, JsObject, JsString, JsUndefined, NapiRaw, Property, Result as NapiResult
};

use virt::{connect::Connect, domain::Domain};

// Add them when we need them
// use log::{error, info, warn};
use napi_derive::js_function;

use crate::machine::Machine;

#[napi]
pub struct Connection {
    con: Connect
}

impl Clone for Connection {
    fn clone(&self) -> Self {
        let uri = self.con.get_uri().expect("Failed to get URI for cloning");
        let new_connection = Connect::open(&uri).expect("Failed to clone connection");
        Connection { con: new_connection }
    }
}

#[napi]
impl Connection {

    pub fn get_connection(&self) -> &Connect {
        return &self.con
    }

    #[napi(factory)]
    pub fn open(name: String) -> napi::Result<Self> {
        let con = Connect::open(&name);
        match con {
            Ok(connection) => Ok( Self { con: connection }),
            Err(err) => {
                Err(napi::Error::from_reason(err.to_string()))
            }
        }
    }

    #[napi]
    pub fn is_alive(&self) -> napi::Result<bool> {
        match self.con.is_alive() {
            Ok(alive) => Ok(alive),
            Err(err) => Err(napi::Error::from_reason(err.to_string()))
        }
    }

    #[napi]
    pub fn get_sys_info(&self, flags: u32) -> napi::Result<String> {
        // Implement
        match self.con.get_sys_info(flags) {
            Ok(info) => Ok(info),
            Err(err) => Err(napi::Error::from_reason(err.to_string()))
        }
    }

    #[napi]
    pub fn get_max_vcpus(&self, attr: String) -> napi::Result<u32> {
        match self.con.get_max_vcpus(&attr) {
            Ok(vcpus) => Ok(vcpus),
            Err(err) => Err(napi::Error::from_reason(err.to_string()))
        }
    }

    #[napi]
    pub fn get_cpu_models_names(&self, arch: String, flags: u32) -> napi::Result<Vec<String>> {
        match self.con.get_cpu_models_names(&arch, flags) {
            Ok(models) => Ok(models),
            Err(err) => Err(napi::Error::from_reason(err.to_string()))
        }
    }

    #[napi]
    pub fn is_encrypted(&self) -> napi::Result<bool> {
        match self.con.is_encrypted() {
            Ok(encrypted) => Ok(encrypted),
            Err(err) => Err(napi::Error::from_reason(err.to_string()))
        }
    }

    #[napi]
    pub fn is_secure(&self) -> napi::Result<bool> {
        match self.con.is_secure() {
            Ok(secure) => Ok(secure),
            Err(err) => Err(napi::Error::from_reason(err.to_string()))
        }
    }

    #[napi]
    pub fn list_active_domain_ids(&self) -> napi::Result<Vec<u32>> {
        // use list_domains
        match self.con.list_domains() {
            Ok(domains) => {
                Ok(domains)
            },
            Err(err) => Err(napi::Error::from_reason(err.to_string()))
        }
    }

    #[napi]
    pub fn list_interfaces(&self) -> napi::Result<Vec<String>> {
        match self.con.list_interfaces() {
            Ok(interfaces) => Ok(interfaces),
            Err(err) => Err(napi::Error::from_reason(err.to_string()))
        }
    }

    #[napi]
    pub fn list_networks(&self) -> napi::Result<Vec<String>> {
        match self.con.list_networks() {
            Ok(networks) => Ok(networks),
            Err(err) => Err(napi::Error::from_reason(err.to_string()))
        }
    }

    #[napi]
    pub fn list_nw_filters(&self) -> napi::Result<Vec<String>> {
        match self.con.list_nw_filters() {
            Ok(filters) => Ok(filters),
            Err(err) => Err(napi::Error::from_reason(err.to_string()))
        }
    }

    #[napi]
    pub fn list_secrets(&self) -> napi::Result<Vec<String>> {
        match self.con.list_secrets() {
            Ok(secrets) => Ok(secrets),
            Err(err) => Err(napi::Error::from_reason(err.to_string()))
        }
    }

    #[napi]
    pub fn list_storage_pools(&self) -> napi::Result<Vec<String>> {
        match self.con.list_storage_pools() {
            Ok(pools) => Ok(pools),
            Err(err) => Err(napi::Error::from_reason(err.to_string()))
        }
    }

    #[napi]
    pub fn list_all_domains(
        &self,
        flags: u32,
    ) -> napi::Result<Vec<Machine>> {
        match self.con.list_all_domains(flags) {
            Ok(domains) => {
                let mut machines = Vec::new();
                for domain in domains {
                    // Clone the domain to avoid moving out of the shared reference
                    let domain_clone = domain;
                    machines.push(Machine::from_domain(domain_clone, &self));
                }
                Ok(machines)
            },
            Err(err) => Err(napi::Error::from_reason(err.to_string()))
        }
    }
}
