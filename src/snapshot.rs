use napi::bindgen_prelude::*;
use virt::domain_snapshot::DomainSnapshot;

/// Represents a domain snapshot.
#[napi]
pub struct Snapshot {
    pub(crate) snapshot: DomainSnapshot,
}

/// Information about a snapshot
#[napi]
pub struct SnapshotInfo {
    /// Name of the snapshot
    pub name: String,
    /// Description of the snapshot
    pub description: Option<String>,
    /// Creation time (seconds since epoch)
    pub creation_time: i64,
    /// Current snapshot flag
    pub is_current: bool,
    /// Has metadata flag
    pub has_metadata: bool,
}

#[napi]
impl Snapshot {
    pub fn from_domain_snapshot(snapshot: DomainSnapshot) -> Self {
        Self { snapshot }
    }

    /// Get the name of the snapshot.
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
    /// const { Machine } = require('libvirt-node');
    ///
    /// async function getSnapshotName() {
    ///   const machine = await Machine.lookupByName(conn, 'your-domain-name');
    ///   const snapshots = await machine.listAllSnapshots(0);
    ///   if (snapshots && snapshots.length > 0) {
    ///     const name = snapshots[0].getName();
    ///     console.log('Snapshot name:', name);
    ///   }
    /// }
    ///
    /// getSnapshotName();
    /// ```
    #[napi]
    pub fn get_name(&self) -> Option<String> {
        match self.snapshot.get_name() {
            Ok(name) => Some(name),
            Err(_) => None,
        }
    }

    /// Get the XML description of the snapshot.
    ///
    /// # Arguments
    ///
    /// * `flags` - The flags to use for the lookup.
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
    /// const { Machine } = require('libvirt-node');
    ///
    /// async function getSnapshotXml() {
    ///   const machine = await Machine.lookupByName(conn, 'your-domain-name');
    ///   const snapshot = await machine.snapshotLookupByName('snapshot-name', 0);
    ///   if (snapshot) {
    ///     const xml = snapshot.getXmlDesc(0);
    ///     console.log('Snapshot XML:', xml);
    ///   }
    /// }
    ///
    /// getSnapshotXml();
    /// ```
    #[napi]
    pub fn get_xml_desc(&self, flags: u32) -> Option<String> {
        match self.snapshot.get_xml_desc(flags) {
            Ok(xml) => Some(xml),
            Err(_) => None,
        }
    }

    /// Delete the snapshot.
    ///
    /// # Arguments
    ///
    /// * `flags` - The flags to use for the deletion. Use VirDomainSnapshotDeleteFlags enum.
    ///
    /// # Returns
    ///
    /// This function returns:
    /// * `true` - If the snapshot is deleted successfully.
    /// * `false` - If there is an error during the deletion.
    ///
    /// # Example (in JavaScript)
    ///
    /// ```javascript
    /// const { Machine } = require('libvirt-node');
    ///
    /// async function deleteSnapshot() {
    ///   const machine = await Machine.lookupByName(conn, 'your-domain-name');
    ///   const snapshot = await machine.snapshotLookupByName('snapshot-name', 0);
    ///   if (snapshot) {
    ///     const success = await snapshot.delete(0);
    ///     if (success) {
    ///       console.log('Snapshot deleted successfully');
    ///     }
    ///   }
    /// }
    ///
    /// deleteSnapshot();
    /// ```
    #[napi]
    pub fn delete(&self, flags: u32) -> bool {
        match self.snapshot.delete(flags) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    /// Check if this snapshot is the current snapshot.
    ///
    /// # Arguments
    ///
    /// * `flags` - The flags to use for the check.
    ///
    /// # Returns
    ///
    /// This function returns:
    /// * `Boolean` - true if this is the current snapshot, false otherwise.
    /// * `null` - If there is an error during the check.
    ///
    /// # Example (in JavaScript)
    ///
    /// ```javascript
    /// const { Machine } = require('libvirt-node');
    ///
    /// async function checkCurrentSnapshot() {
    ///   const machine = await Machine.lookupByName(conn, 'your-domain-name');
    ///   const snapshot = await machine.snapshotLookupByName('snapshot-name', 0);
    ///   if (snapshot) {
    ///     const isCurrent = snapshot.isCurrent(0);
    ///     console.log('Is current snapshot:', isCurrent);
    ///   }
    /// }
    ///
    /// checkCurrentSnapshot();
    /// ```
    #[napi]
    pub fn is_current(&self, flags: u32) -> Option<bool> {
        match self.snapshot.is_current(flags) {
            Ok(current) => Some(current),
            Err(_) => None,
        }
    }

    /// Check if the snapshot has metadata.
    ///
    /// # Arguments
    ///
    /// * `flags` - The flags to use for the check.
    ///
    /// # Returns
    ///
    /// This function returns:
    /// * `Boolean` - true if the snapshot has metadata, false otherwise.
    /// * `null` - If there is an error during the check.
    #[napi]
    pub fn has_metadata(&self, flags: u32) -> Option<bool> {
        match self.snapshot.has_metadata(flags) {
            Ok(has_meta) => Some(has_meta),
            Err(_) => None,
        }
    }

    /// Get the parent snapshot of this snapshot.
    ///
    /// # Arguments
    ///
    /// * `flags` - The flags to use for the lookup.
    ///
    /// # Returns
    ///
    /// This function returns:
    /// * `Snapshot` - The parent snapshot.
    /// * `null` - If there is no parent or an error occurred.
    #[napi]
    pub fn get_parent(&self, flags: u32) -> Option<Snapshot> {
        match self.snapshot.get_parent(flags) {
            Ok(parent) => Some(Snapshot::from_domain_snapshot(parent)),
            Err(_) => None,
        }
    }

    /// Free the snapshot object.
    ///
    /// # Returns
    ///
    /// This function returns:
    /// * `true` - If the snapshot is freed successfully.
    /// * `false` - If there is an error during the operation.
    #[napi]
    pub fn free(&mut self) -> bool {
        match self.snapshot.free() {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}