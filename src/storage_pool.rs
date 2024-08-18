use napi::{Env, JsObject, Property, Result as NapiResult};
use virt;
use virt::connect::Connect;

#[napi]
pub struct StoragePool {
    storage_pool: virt::storage_pool::StoragePool
}
#[napi]
impl StoragePool {
    pub fn get(&self) -> &virt::storage_pool::StoragePool {
        &self.storage_pool
    }

    pub fn from_storage_pool(storage_pool: virt::storage_pool::StoragePool) -> Self {
        Self { storage_pool }
    }

    // define_xml
    #[napi]
    pub fn define_xml(
        conn: &crate::connection::Connection,
        xml: String
    ) -> napi::Result<StoragePool> {
        match virt::storage_pool::StoragePool::define_xml(conn.get_connection(), &xml, 0) {
            Ok(pool) => Ok(StoragePool::from_storage_pool(pool)),
            Err(err) => Err(napi::Error::from_reason(err.to_string())),
        }
    }

    // create_xml
    #[napi]
    pub fn create_xml(
        conn: &crate::connection::Connection,
        xml: String,
        flags: u32
    ) -> napi::Result<StoragePool> {
        match virt::storage_pool::StoragePool::create_xml(conn.get_connection(), &xml, flags) {
            Ok(pool) => Ok(StoragePool::from_storage_pool(pool)),
            Err(err) => Err(napi::Error::from_reason(err.to_string())),
        }
    }

    // lookup_by_name
    #[napi]
    pub fn lookup_by_name(
        conn: &crate::connection::Connection,
        name: String
    ) -> napi::Result<StoragePool> {
        match virt::storage_pool::StoragePool::lookup_by_name(conn.get_connection(), &name) {
            Ok(pool) => Ok(StoragePool::from_storage_pool(pool)),
            Err(err) => Err(napi::Error::from_reason(err.to_string())),
        }
    }

    // TODO: implement lookup_by_volume

    // lookup_by_uuid_string
    #[napi]
    pub fn lookup_by_uuid_string(
        conn: &crate::connection::Connection,
        uuid: String
    ) -> napi::Result<StoragePool> {
        match virt::storage_pool::StoragePool::lookup_by_uuid_string(conn.get_connection(), &uuid) {
            Ok(pool) => Ok(StoragePool::from_storage_pool(pool)),
            Err(err) => Err(napi::Error::from_reason(err.to_string())),
        }
    }

    // get_name
    #[napi]
    pub fn get_name(&self) -> napi::Result<String> {
        match self.storage_pool.get_name() {
            Ok(name) => Ok(name),
            Err(err) => Err(napi::Error::from_reason(err.to_string())),
        }
    }

    // num_of_volumes
    #[napi]
    pub fn num_of_volumes(&self) -> napi::Result<u32> {
        match self.storage_pool.num_of_volumes() {
            Ok(num) => Ok(num),
            Err(err) => Err(napi::Error::from_reason(err.to_string())),
        }
    }

    // list_volumes
    #[napi]
    pub fn list_volumes(&self) -> napi::Result<Vec<String>> {
        match self.storage_pool.list_volumes() {
            Ok(volumes) => Ok(volumes),
            Err(err) => Err(napi::Error::from_reason(err.to_string())),
        }
    }

    // ...

    // get_uuid_string
    #[napi]
    pub fn get_uuid_string(&self) -> napi::Result<String> {
        match self.storage_pool.get_uuid_string() {
            Ok(uuid) => Ok(uuid),
            Err(err) => Err(napi::Error::from_reason(err.to_string())),
        }
    }

    // get_xml_desc
    #[napi]
    pub fn get_xml_desc(&self) -> napi::Result<String> {
        match self.storage_pool.get_xml_desc(0) {
            Ok(xml) => Ok(xml),
            Err(err) => Err(napi::Error::from_reason(err.to_string())),
        }
    }

    // pub fn create(&self, flags: sys::virStoragePoolCreateFlags) -> Result<u32, Error> {
    #[napi]
    pub fn create(&self, flags: u32) -> napi::Result<u32> {
        match self.storage_pool.create(flags) {
            Ok(num) => Ok(num),
            Err(err) => Err(napi::Error::from_reason(err.to_string())),
        }
    }

    // build
    #[napi]
    pub fn build(&self, flags: u32) -> napi::Result<u32> {
        match self.storage_pool.build(flags) {
            Ok(num) => Ok(num),
            Err(err) => Err(napi::Error::from_reason(err.to_string())),
        }
    }

    // destroy
    #[napi]
    pub fn destroy(&self) -> napi::Result<()> {
        match self.storage_pool.destroy() {
            Ok(_) => Ok(()),
            Err(err) => Err(napi::Error::from_reason(err.to_string())),
        }
    }

    #[napi]
    pub fn undefine(&self) -> napi::Result<()> {
        match self.storage_pool.undefine() {
            Ok(_) => Ok(()),
            Err(err) => Err(napi::Error::from_reason(err.to_string())),
        }
    }

    #[napi]
    pub fn free(&mut self) -> napi::Result<()> {
        match self.storage_pool.free() {
            Ok(_) => Ok(()),
            Err(err) => Err(napi::Error::from_reason(err.to_string())),
        }
    }

    #[napi]
    pub fn is_active(&self) -> napi::Result<bool> {
        match self.storage_pool.is_active() {
            Ok(active) => Ok(active),
            Err(err) => Err(napi::Error::from_reason(err.to_string())),
        }
    }

    #[napi]
    pub fn is_persistent(&self) -> napi::Result<bool> {
        match self.storage_pool.is_persistent() {
            Ok(persistent) => Ok(persistent),
            Err(err) => Err(napi::Error::from_reason(err.to_string())),
        }
    }

    // TODO: create enum for this flags
    #[napi]
    pub fn refresh(&self, flags: u32) -> napi::Result<()> {
        match self.storage_pool.refresh(flags) {
            Ok(_) => Ok(()),
            Err(err) => Err(napi::Error::from_reason(err.to_string())),
        }
    }

    #[napi]
    pub fn get_autostart(&self) -> napi::Result<bool> {
        match self.storage_pool.get_autostart() {
            Ok(autostart) => Ok(autostart),
            Err(err) => Err(napi::Error::from_reason(err.to_string())),
        }
    }

    #[napi]
    pub fn set_autostart(&self, autostart: bool) -> napi::Result<()> {
        match self.storage_pool.set_autostart(autostart) {
            Ok(_) => Ok(()),
            Err(err) => Err(napi::Error::from_reason(err.to_string())),
        }
    }

    // get_info -> return a json/hash object
    #[napi]
    pub fn get_info(&self) -> napi::Result<serde_json::Value> {
        match self.storage_pool.get_info() {
            Ok(info) => {
                let mut json = serde_json::Map::new();
                json.insert("state".to_string(), serde_json::Value::Number(info.state.into()));
                json.insert("capacity".to_string(), serde_json::Value::Number(info.capacity.into()));
                json.insert("allocation".to_string(), serde_json::Value::Number(info.allocation.into()));
                json.insert("available".to_string(), serde_json::Value::Number(info.available.into()));
                Ok(serde_json::Value::Object(json))
            },
            Err(err) => Err(napi::Error::from_reason(err.to_string())),
        }
    }
}