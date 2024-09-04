use crate::connection::Connection;
use crate::storage_pool::StoragePool;
use napi;
use serde_json::json;
use virt::storage_vol::StorageVol as Vol;

#[napi]
pub struct StorageVol {
    vol: Vol,
}

#[napi]
impl StorageVol {
    /// Creates a new storage volume in the given storage pool.
    ///
    /// # Arguments
    ///
    /// * `pool` - A reference to the StoragePool where the volume will be created.
    /// * `xml` - The XML description of the storage volume to create.
    /// * `flags` - Bitwise-OR of virStorageVolCreateFlags.
    ///
    /// # Returns
    ///
    /// A Result containing the newly created StorageVol on success, or an Error on failure.
    ///
    /// # Example
    ///
    /// ```javascript
    /// const libvirt = require('libvirt');
    ///
    /// async function createQcow2Disk() {
    ///   const conn = await libvirt.Connection.open('qemu:///system');
    ///   const pool = await conn.storagePoolLookupByName('default');
    ///
    ///   const volumeXml = `
    ///     <volume>
    ///       <name>mydisk.qcow2</name>
    ///       <allocation>0</allocation>
    ///       <capacity unit="G">20</capacity>
    ///       <target>
    ///         <format type='qcow2'/>
    ///       </target>
    ///     </volume>
    ///   `;
    ///
    ///   const volume = await StorageVol.createXml(pool, volumeXml, 0);
    ///   console.log(`Created volume: ${await volume.getName()}`);
    ///
    ///   await conn.close();
    /// }
    ///
    /// createQcow2Disk().catch(console.error);
    /// ```
    #[napi]
    pub fn create_xml(
        pool: &StoragePool,
        xml: String,
        flags: u32,
    ) -> Option<StorageVol> {
        match Vol::create_xml(&pool.get(), &xml, flags) {
            Ok(vol) => Some(StorageVol { vol }),
            Err(_) => None,
        }
    }

    /// Creates a storage volume, using an existing volume as input.
    ///
    /// # Arguments
    ///
    /// * `pool` - A reference to the StoragePool where the volume will be created.
    /// * `xml` - The XML description of the storage volume to create.
    /// * `vol` - The source StorageVol to clone from.
    /// * `flags` - Bitwise-OR of virStorageVolCreateFlags.
    ///
    /// # Returns
    ///
    /// A Result containing the newly created StorageVol on success, or an Error on failure.
    ///
    /// # Example
    ///
    /// ```javascript
    /// const libvirt = require('libvirt');
    ///
    /// async function cloneVolume() {
    ///   const conn = await libvirt.Connection.open('qemu:///system');
    ///   const pool = await conn.storagePoolLookupByName('default');
    ///
    ///   const sourceVolume = await StorageVol.lookupByName(pool, 'source_volume.qcow2');
    ///
    ///   const cloneXml = `
    ///     <volume>
    ///       <name>cloned_volume.qcow2</name>
    ///       <capacity>0</capacity>
    ///     </volume>
    ///   `;
    ///
    ///   const clonedVolume = await StorageVol.createXmlFrom(pool, cloneXml, sourceVolume, 0);
    ///   console.log(`Cloned volume: ${await clonedVolume.getName()}`);
    ///
    ///   await conn.close();
    /// }
    ///
    /// cloneVolume().catch(console.error);
    /// ```
    #[napi]
    pub fn create_xml_from(
        pool: &StoragePool,
        xml: String,
        vol: &StorageVol,
        flags: u32,
    ) -> Option<StorageVol> {
        match Vol::create_xml_from(&pool.get(), &xml, &vol.vol, flags) {
            Ok(new_vol) => Some(StorageVol { vol: new_vol }),
            Err(_) => None,
        }
    }

    /// Deletes a storage volume.
    ///
    /// # Arguments
    ///
    /// * `flags` - Bitwise-OR of virStorageVolDeleteFlags.
    ///
    /// # Returns
    ///
    /// A Result containing () on success, or an Error on failure.
    ///
    /// # Example
    ///
    /// ```javascript
    /// const libvirt = require('libvirt');
    ///
    /// async function deleteVolume() {
    ///   const conn = await libvirt.Connection.open('qemu:///system');
    ///   const pool = await conn.storagePoolLookupByName('default');
    ///
    ///   const volume = await StorageVol.lookupByName(pool, 'volume_to_delete.qcow2');
    ///
    ///   // Delete the volume
    ///   // 0 is passed as flags to use default behavior
    ///   await volume.delete(0);
    ///
    ///   console.log('Volume deleted successfully');
    ///
    ///   await conn.close();
    /// }
    ///
    /// deleteVolume().catch(console.error);
    /// ```
    #[napi]
    pub fn delete(&self, flags: u32) -> Option<u32> {
        match self.vol.delete(flags) {
            Ok(_) => Some(0),
            Err(_) => None,
        }
    }

    // #[napi]
    // pub fn download(&self, stream: i32, offset: BigInt, length: BigInt, flags: u32) -> napi::Result<()> {
    //     match self.vol.download(stream, offset.get_u64().1, length.get_u64().1, flags) {
    //         Ok(_) => Ok(()),
    //         Err(e) => Err(Error::from_reason(e.to_string())),
    //     }
    // }

    /// Retrieves information about a storage volume.
    ///
    /// # Returns
    ///
    /// A Result containing a JsObject with the following properties:
    /// * `type`: The type of the storage volume (u32).
    /// * `capacity`: The total capacity of the storage volume in bytes (BigInt).
    /// * `allocation`: The current allocation of the storage volume in bytes (BigInt).
    ///
    /// # Example
    ///
    /// ```javascript
    /// const libvirt = require('libvirt');
    ///
    /// async function getVolumeInfo() {
    ///   const conn = await libvirt.Connection.open('qemu:///system');
    ///   const pool = await conn.storagePoolLookupByName('default');
    ///   const volume = await StorageVol.lookupByName(pool, 'my_volume.qcow2');
    ///
    ///   const info = await volume.getInfo();
    ///   console.log('Volume type:', info.type);
    ///   console.log('Volume capacity:', info.capacity.toString());
    ///   console.log('Volume allocation:', info.allocation.toString());
    ///
    ///   await conn.close();
    /// }
    ///
    /// getVolumeInfo().catch(console.error);
    /// ```
    #[napi]
    pub fn get_info(&self) -> Option<serde_json::Value> {
        // TODO: Provably we will need to create a struct to match the info returned by libvirt
        // and then convert it to a JsObject
        match self.vol.get_info() {
            Ok(info) => {
                let value = json!({
                    "type": info.kind as u32,
                    "capacity": info.capacity.to_string(),
                    "allocation": info.allocation.to_string(),
                });
                Some(value)
            },
            Err(_) => None,
        }
    }

    /// Retrieves the name of the storage volume.
    ///
    /// # Returns
    ///
    /// A Result containing a String with the name of the storage volume.
    ///
    /// # Example
    ///
    /// ```javascript
    /// const libvirt = require('libvirt');
    ///
    /// async function getVolumeName() {
    ///   const conn = await libvirt.Connection.open('qemu:///system');
    ///   const pool = await conn.storagePoolLookupByName('default');
    ///   const volume = await StorageVol.lookupByName(pool, 'my_volume.qcow2');
    ///
    ///   const name = await volume.getName();
    ///   console.log('Volume name:', name);
    ///
    ///   await conn.close();
    /// }
    ///
    /// getVolumeName().catch(console.error);
    /// ```
    #[napi]
    pub fn get_name(&self) -> Option<String> {
        match self.vol.get_name() {
            Ok(name) => Some(name),
            Err(_) => None,
        }
    }

    /// Retrieves the path of the storage volume.
    ///
    /// # Returns
    ///
    /// A Result containing a String with the path of the storage volume.
    ///
    /// # Example
    ///
    /// ```javascript
    /// const libvirt = require('libvirt');
    ///
    /// async function getVolumePath() {
    ///   const conn = await libvirt.Connection.open('qemu:///system');
    ///   const pool = await conn.storagePoolLookupByName('default');
    ///   const volume = await StorageVol.lookupByName(pool, 'my_volume.qcow2');
    ///
    ///   const path = await volume.getPath();
    ///   console.log('Volume path:', path);
    ///
    ///   await conn.close();
    /// }
    ///
    /// getVolumePath().catch(console.error);
    /// ```
    #[napi]
    pub fn get_path(&self) -> Option<String> {
        match self.vol.get_path() {
            Ok(path) => Some(path),
            Err(_) => None,
        }
    }

    /// Retrieves the XML description of the storage volume.
    ///
    /// # Arguments
    ///
    /// * `flags` - Bitwise-OR of virStorageXMLFlags
    ///
    /// # Returns
    ///
    /// A Result containing a String with the XML description of the storage volume.
    ///
    /// # Example
    ///
    /// ```javascript
    /// const libvirt = require('libvirt');
    ///
    /// async function getVolumeXMLDesc() {
    ///   const conn = await libvirt.Connection.open('qemu:///system');
    ///   const pool = await conn.storagePoolLookupByName('default');
    ///   const volume = await StorageVol.lookupByName(pool, 'my_volume.qcow2');
    ///
    ///   const xmlDesc = await volume.getXMLDesc(0);
    ///   console.log('Volume XML description:', xmlDesc);
    ///
    ///   await conn.close();
    /// }
    ///
    /// getVolumeXMLDesc().catch(console.error);
    /// ```
    #[napi]
    pub fn get_xml_desc(&self, flags: u32) -> Option<String> {
        match self.vol.get_xml_desc(flags) {
            Ok(xml) => Some(xml),
            Err(_) => None,
        }
    }

    /// Resizes a storage volume.
    ///
    /// # Arguments
    ///
    /// * `capacity` - New capacity for the volume, in bytes.
    /// * `flags` - Bitwise-OR of virStorageVolResizeFlags
    ///
    /// # Returns
    ///
    /// A Result indicating success or failure of the operation.
    ///
    /// # Example
    ///
    /// ```javascript
    /// const libvirt = require('libvirt');
    ///
    /// async function resizeVolume() {
    ///   const conn = await libvirt.Connection.open('qemu:///system');
    ///   const pool = await conn.storagePoolLookupByName('default');
    ///   const volume = await StorageVol.lookupByName(pool, 'my_volume.qcow2');
    ///
    ///   // Resize the volume to 10 GB
    ///   const newCapacity = BigInt(10 * 1024 * 1024 * 1024); // 10 GB in bytes
    ///   await volume.resize(newCapacity, 0);
    ///   console.log('Volume resized successfully');
    ///
    ///   await conn.close();
    /// }
    ///
    /// resizeVolume().catch(console.error);
    /// ```
    #[napi]
    pub fn resize(&self, capacity: napi::bindgen_prelude::BigInt, flags: u32) -> Option<u32> {
        match self.vol.resize(capacity.get_u64().1, flags) {
            Ok(_) => Some(0),
            Err(_) => None,
        }
    }

    // #[napi]
    // pub fn upload(&self, stream: i32, offset: BigInt, length: BigInt, flags: u32) -> napi::Result<()> {
    //     match self.vol.upload(stream, offset.get_u64().1, length.get_u64().1, flags) {
    //         Ok(_) => Ok(()),
    //         Err(e) => Err(Error::from_reason(e.to_string())),
    //     }
    // }


    /// Wipes a storage volume.
    ///
    /// This method erases all data from the storage volume.
    ///
    /// # Arguments
    ///
    /// * `flags` - Bitwise-OR of virStorageVolWipeFlags
    ///
    /// # Returns
    ///
    /// A Result indicating success or failure of the operation.
    ///
    /// # Example
    ///
    /// ```javascript
    /// const libvirt = require('libvirt');
    ///
    /// async function wipeVolume() {
    ///   const conn = await libvirt.Connection.open('qemu:///system');
    ///   const pool = await conn.storagePoolLookupByName('default');
    ///   const volume = await StorageVol.lookupByName(pool, 'my_volume.qcow2');
    ///
    ///   // Wipe the volume
    ///   await volume.wipe(0);
    ///   console.log('Volume wiped successfully');
    ///
    ///   await conn.close();
    /// }
    ///
    /// wipeVolume().catch(console.error);
    /// ```
    #[napi]
    pub fn wipe(&self, flags: u32) -> Option<u32> {
        match self.vol.wipe(flags) {
            Ok(_) => Some(0),
            Err(_) => None,
        }
    }

    /// Looks up a storage volume based on its name within a storage pool.
    ///
    /// # Arguments
    ///
    /// * `pool` - A reference to the StoragePool to search in.
    /// * `name` - The name of the storage volume to look up.
    ///
    /// # Returns
    ///
    /// A Result containing an Option<StorageVol>. If the volume is found, it returns Some(StorageVol).
    /// If the volume is not found, it returns None. Other errors result in an Error.
    ///
    /// # Example
    ///
    /// ```javascript
    /// const libvirt = require('libvirt');
    ///
    /// async function lookupVolume() {
    ///   const conn = await libvirt.Connection.open('qemu:///system');
    ///   const pool = await conn.storagePoolLookupByName('default');
    ///
    ///   const volume = await StorageVol.lookupByName(pool, 'my_volume.qcow2');
    ///   if (volume) {
    ///     console.log('Volume found:', await volume.getName());
    ///   } else {
    ///     console.log('Volume not found');
    ///   }
    ///
    ///   await conn.close();
    /// }
    ///
    /// lookupVolume().catch(console.error);
    /// ```
    #[napi]
    pub fn lookup_by_name(pool: &StoragePool, name: String) -> Option<StorageVol> {
        match Vol::lookup_by_name(&pool.get(), &name) {
            Ok(vol) => Some(StorageVol { vol }),
            Err(_) => None,
        }
    }

    /// Looks up a storage volume based on its unique key.
    ///
    /// # Arguments
    ///
    /// * `conn` - A reference to the Connection to use for the lookup.
    /// * `key` - The unique key of the storage volume to look up.
    ///
    /// # Returns
    ///
    /// A Result containing an Option<StorageVol>. If the volume is found, it returns Some(StorageVol).
    /// If the volume is not found, it returns None. Other errors result in an Error.
    ///
    /// # Example
    ///
    /// ```javascript
    /// const libvirt = require('libvirt');
    ///
    /// async function lookupVolumeByKey() {
    ///   const conn = await libvirt.Connection.open('qemu:///system');
    ///
    ///   const volumeKey = '/path/to/volume/key';
    ///   const volume = await StorageVol.lookupByKey(conn, volumeKey);
    ///   
    ///   if (volume) {
    ///     console.log('Volume found:', await volume.getName());
    ///   } else {
    ///     console.log('Volume not found');
    ///   }
    ///
    ///   await conn.close();
    /// }
    ///
    /// lookupVolumeByKey().catch(console.error);
    /// ```
    #[napi]
    pub fn lookup_by_key(conn: &Connection, key: String) -> Option<StorageVol> {
        match Vol::lookup_by_key(&conn.get_connection(), &key) {
            Ok(vol) => Some(StorageVol { vol }),
            Err(_) => None,
        }
    }

    /// Looks up a storage volume based on its path.
    ///
    /// # Arguments
    ///
    /// * `conn` - A reference to the Connection to use for the lookup.
    /// * `path` - The path of the storage volume to look up.
    ///
    /// # Returns
    ///
    /// A Result containing an Option<StorageVol>. If the volume is found, it returns Some(StorageVol).
    /// If the volume is not found, it returns None. Other errors result in an Error.
    ///
    /// # Example
    ///
    /// ```javascript
    /// const libvirt = require('libvirt');
    ///
    /// async function lookupVolumeByPath() {
    ///   const conn = await libvirt.Connection.open('qemu:///system');
    ///
    ///   const volumePath = '/path/to/storage/volume.qcow2';
    ///   const volume = await StorageVol.lookupByPath(conn, volumePath);
    ///   
    ///   if (volume) {
    ///     console.log('Volume found:', await volume.getName());
    ///   } else {
    ///     console.log('Volume not found');
    ///   }
    ///
    ///   await conn.close();
    /// }
    ///
    /// lookupVolumeByPath().catch(console.error);
    /// ```
    #[napi]
    pub fn lookup_by_path(conn: &Connection, path: String) -> Option<StorageVol> {
        match Vol::lookup_by_path(&conn.get_connection(), &path) {
            Ok(vol) => Some(StorageVol { vol }),
            Err(_) => None,
        }
    }

    /// Frees the storage volume object.
    ///
    /// This method should be called when the storage volume object is no longer needed.
    /// It releases the resources associated with the object.
    ///
    /// # Returns
    ///
    /// A Result containing () on success, or an Error on failure.
    ///
    /// # Example
    ///
    /// ```javascript
    /// const libvirt = require('libvirt');
    ///
    /// async function freeStorageVolume() {
    ///   const conn = await libvirt.Connection.open('qemu:///system');
    ///   const pool = await conn.storagePoolLookupByName('default');
    ///   const volume = await StorageVol.lookupByName(pool, 'my_volume.qcow2');
    ///
    ///   // Use the volume...
    ///
    ///   // Free the volume when done
    ///   await volume.free();
    ///
    ///   console.log('Volume freed successfully');
    ///
    ///   await conn.close();
    /// }
    ///
    /// freeStorageVolume().catch(console.error);
    /// ```
    ///
    /// Note: After calling this method, the StorageVol object should not be used anymore.
		#[napi]
    pub fn free(&mut self) -> Option<u32> {
        match self.vol.free() {
            Ok(_) => Some(0),
            Err(_) =>None,
        }
    }

    /// Wipes a storage volume using a specific algorithm.
    ///
    /// This method erases the data on the storage volume using the specified wiping algorithm.
    ///
    /// # Arguments
    ///
    /// * `algorithm` - The algorithm to use for wiping. See virStorageVolWipeAlgorithm for possible values.
    /// * `flags` - Bitwise-OR of virStorageVolWipeFlags.
    ///
    /// # Returns
    ///
    /// A Result containing () on success, or an Error on failure.
    ///
    /// # Example
    ///
    /// ```javascript
    /// const libvirt = require('libvirt');
    ///
    /// async function wipeVolume() {
    ///   const conn = await libvirt.Connection.open('qemu:///system');
    ///   const pool = await conn.storagePoolLookupByName('default');
    ///   const volume = await StorageVol.lookupByName(pool, 'volume_to_wipe.qcow2');
    ///
    ///   // Wipe the volume using zero-fill algorithm
    ///   // 0 is VIR_STORAGE_VOL_WIPE_ALG_ZERO
    ///   // 0 is passed as flags to use default behavior
    ///   await volume.wipePattern(0, 0);
    ///
    ///   console.log('Volume wiped successfully');
    ///
    ///   await conn.close();
    /// }
    ///
    /// wipeVolume().catch(console.error);
    /// ```
    ///
    /// Note: This operation may take a long time depending on the size of the volume and the chosen algorithm.
		#[napi]
    pub fn wipe_pattern(&self, algorithm: u32, flags: u32) -> Option<u32> {
        match self.vol.wipe_pattern(algorithm, flags) {
            Ok(_) => Some(0),
            Err(_) => None,
        }
    }
}