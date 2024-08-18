#[napi]
#[repr(u32)]
pub enum VirDomainGetHostnameFlags {
  /// Parse DHCP lease file
  VirDomainGetHostnameLease = 1,
  /// Query qemu guest agent
  VirDomainGetHostnameAgent = 2
}

#[napi]
#[repr(u32)]
pub enum VirDomainXMLFlags {
	/// dump security sensitive information too
	VirDomainXMLSecure	= 1, 
	/// dump inactive domain information
	VirDomainXMLInactive	= 2,
	/// update guest CPU requirements according to host CPU
	VirDomainXMLUpdateCPU	= 4,
	/// dump XML suitable for migration
	VirDomainXMLMigratable	= 8,
}

#[napi]
#[repr(u32)]
pub enum VirDomainCreateFlags {
    /// Default behavior
    VirDomainNone = 0,
    /// Launch guest in paused state
    VirDomainStartPaused = 1,
    /// Automatically kill guest when virConnectPtr is closed
    VirDomainStartAutodestroy = 2,
    /// Avoid file system cache pollution
    VirDomainStartBypassCache = 4,
    /// Boot, discarding any managed save
    VirDomainStartForceBoot = 8,
    /// Validate the XML document against schema
    VirDomainStartValidate = 16,
    /// Re-initialize NVRAM from template
    VirDomainStartResetNvram = 32,
}

#[napi]
#[repr(u32)]
pub enum VirDomainDefineFlags {
    /// Validate the XML document against schema
    VirDomainDefineValidate = 1,
}

#[napi]
#[repr(u32)]
pub enum VirDomainDestroyFlags {
    /// Default behavior - could lead to data loss!!
    VirDomainDestroyDefault = 0,
    /// Only SIGTERM, no SIGKILL
    VirDomainDestroyGraceful = 1,
    /// Remove VM logs on destroy
    VirDomainDestroyRemoveLogs = 2,
}

#[napi]
#[repr(u32)]
pub enum VirDomainRebootFlag {
    /// Hypervisor choice
    VirDomainRebootDefault = 0,
    /// Send ACPI event
    VirDomainRebootAcpiPowerBtn = 1,
    /// Use guest agent
    VirDomainRebootGuestAgent = 2,
    /// Use initctl
    VirDomainRebootInitctl = 4,
    /// Send a signal
    VirDomainRebootSignal = 8,
    /// Use paravirt guest control
    VirDomainRebootParavirt = 16,
}

#[napi]
#[repr(u32)]
pub enum VirDomainUndefineFlags {
    /// Also remove any managed save
    VirDomainUndefineManagedSave = 1,
    /// If last use of domain, then also remove any snapshot metadata
    VirDomainUndefineSnapshotsMetadata = 2,
    /// Also remove any nvram file
    VirDomainUndefineNvram = 4,
    /// Keep nvram file
    VirDomainUndefineKeepNvram = 8,
    /// If last use of domain, then also remove any checkpoint metadata
    VirDomainUndefineCheckpointsMetadata = 16,
    /// Also remove any TPM state
    VirDomainUndefineTpm = 32,
    /// Keep TPM state
    VirDomainUndefineKeepTpm = 64,
}

#[napi]
#[repr(u32)]
pub enum VirDomainModificationImpact {
    /// Affect current domain state
    VirDomainAffectCurrent = 0,
    /// Affect running domain state
    VirDomainAffectLive = 1,
    /// Affect persistent domain state
    VirDomainAffectConfig = 2,
}

#[napi]
#[repr(u32)]
pub enum VirDomainMemoryModFlags {
    /// See virDomainModificationImpact
    VirDomainMemConfig = 2, // VIR_DOMAIN_AFFECT_CONFIG
    /// See virDomainModificationImpact
    VirDomainMemCurrent = 0, // VIR_DOMAIN_AFFECT_CURRENT
    /// See virDomainModificationImpact
    VirDomainMemLive = 1, // VIR_DOMAIN_AFFECT_LIVE
    /// Affect Max rather than current
    VirDomainMemMaximum = 4,
}

#[napi]
#[repr(u32)]
pub enum VirStoragePoolCreateFlags {
    /// Default behavior
    VirStoragePoolCreateNormal = 0,
    /// Create pool from XML, build it
    VirStoragePoolCreateWithBuild = 1,
    /// Create pool from XML, build it and overwrite if exists
    VirStoragePoolCreateWithBuildOverwrite = 2,
    /// Create pool from XML, build it and do not overwrite if exists
    VirStoragePoolCreateWithBuildNoOverwrite = 4,
}

#[napi]
#[repr(u32)]
pub enum VirStorageXMLFlags {
    /// Inactive
    VirStorageXMLInactive = 1,
}