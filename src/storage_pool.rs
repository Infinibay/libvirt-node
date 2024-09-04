use virt;

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
    ) -> Option<StoragePool> {
        match virt::storage_pool::StoragePool::define_xml(conn.get_connection(), &xml, 0) {
            Ok(pool) => Some(StoragePool::from_storage_pool(pool)),
            Err(_) => None,
        }
    }

    // create_xml
    #[napi]
    pub fn create_xml(
        conn: &crate::connection::Connection,
        xml: String,
        flags: u32
    ) -> Option<StoragePool> {
        match virt::storage_pool::StoragePool::create_xml(conn.get_connection(), &xml, flags) {
            Ok(pool) => Some(StoragePool::from_storage_pool(pool)),
            Err(_) => None,
        }
    }

    // lookup_by_name
    #[napi]
    pub fn lookup_by_name(
        conn: &crate::connection::Connection,
        name: String
    ) -> Option<StoragePool> {
        match virt::storage_pool::StoragePool::lookup_by_name(conn.get_connection(), &name) {
            Ok(pool) => Some(StoragePool::from_storage_pool(pool)),
            Err(_) => None,
        }
    }

    // TODO: implement lookup_by_volume

    // lookup_by_uuid_string
    #[napi]
    pub fn lookup_by_uuid_string(
        conn: &crate::connection::Connection,
        uuid: String
    ) -> Option<StoragePool> {
        match virt::storage_pool::StoragePool::lookup_by_uuid_string(conn.get_connection(), &uuid) {
            Ok(pool) => Some(StoragePool::from_storage_pool(pool)),
            Err(_) => None,
        }
    }

    // get_name
    #[napi]
    pub fn get_name(&self) -> Option<String> {
        match self.storage_pool.get_name() {
            Ok(name) => Some(name),
            Err(_) => None,
        }
    }

    // num_of_volumes
    #[napi]
    pub fn num_of_volumes(&self) -> Option<u32> {
        match self.storage_pool.num_of_volumes() {
            Ok(num) => Some(num),
            Err(_) => None,
        }
    }

    // list_volumes
    #[napi]
    pub fn list_volumes(&self) -> Option<Vec<String>> {
        match self.storage_pool.list_volumes() {
            Ok(volumes) => Some(volumes),
            Err(_) => None,
        }
    }

    // ...

    // get_uuid_string
    #[napi]
    pub fn get_uuid_string(&self) -> Option<String> {
        match self.storage_pool.get_uuid_string() {
            Ok(uuid) => Some(uuid),
            Err(_) => None,
        }
    }

    // get_xml_desc
    #[napi]
    pub fn get_xml_desc(&self) -> Option<String> {
        match self.storage_pool.get_xml_desc(0) {
            Ok(xml) => Some(xml),
            Err(_) => None,
        }
    }

    // pub fn create(&self, flags: sys::virStoragePoolCreateFlags) -> Result<u32, Error> {
    #[napi]
    pub fn create(&self, flags: u32) -> Option<u32> {
        match self.storage_pool.create(flags) {
            Ok(num) => Some(num),
            Err(_) => None,
        }
    }

    // build
    #[napi]
    pub fn build(&self, flags: u32) -> Option<u32> {
        match self.storage_pool.build(flags) {
            Ok(num) => Some(num),
            Err(_) => None,
        }
    }

    // destroy
    #[napi]
    pub fn destroy(&self) -> Option<u32> {
        match self.storage_pool.destroy() {
            Ok(_) => Some(0),
            Err(_) => None,
        }
    }

    #[napi]
    pub fn undefine(&self) -> Option<u32> {
        match self.storage_pool.undefine() {
            Ok(_) => Some(0),
            Err(_) => None,
        }
    }

    #[napi]
    pub fn free(&mut self) -> Option<u32> {
        match self.storage_pool.free() {
            Ok(_) => Some(0),
            Err(_) => None,
        }
    }

    #[napi]
    pub fn is_active(&self) -> Option<bool> {
        match self.storage_pool.is_active() {
            Ok(active) => Some(active),
            Err(_) => None,
        }
    }

    #[napi]
    pub fn is_persistent(&self) -> Option<bool> {
        match self.storage_pool.is_persistent() {
            Ok(persistent) => Some(persistent),
            Err(_) => None,
        }
    }

    // TODO: create enum for this flags
    #[napi]
    pub fn refresh(&self, flags: u32) -> Option<u32> {
        match self.storage_pool.refresh(flags) {
            Ok(_) => Some(0),
            Err(_) => None,
        }
    }

    #[napi]
    pub fn get_autostart(&self) -> Option<bool> {
        match self.storage_pool.get_autostart() {
            Ok(autostart) => Some(autostart),
            Err(_) => None,
        }
    }

    #[napi]
    pub fn set_autostart(&self, autostart: bool) -> Option<u32> {
        match self.storage_pool.set_autostart(autostart) {
            Ok(_) => Some(0),
            Err(_) => None,
        }
    }

    // get_info -> return a json/hash object
    #[napi]
    pub fn get_info(&self) -> Option<serde_json::Value> {
        match self.storage_pool.get_info() {
            Ok(info) => {
                let mut json = serde_json::Map::new();
                json.insert("state".to_string(), serde_json::Value::Number(info.state.into()));
                json.insert("capacity".to_string(), serde_json::Value::Number(info.capacity.into()));
                json.insert("allocation".to_string(), serde_json::Value::Number(info.allocation.into()));
                json.insert("available".to_string(), serde_json::Value::Number(info.available.into()));
                Some(serde_json::Value::Object(json))
            },
            Err(_) => None,
        }
    }
}
