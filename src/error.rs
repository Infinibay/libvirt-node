use napi;

// ...existing code...

/// The level of an error.
///
/// See <https://libvirt.org/html/libvirt-virterror.html#virErrorLevel>
#[napi]
pub enum ErrorLevel {
	/// No error.
	None = 0,
	/// A simple warning.
	Warning,
	/// An error.
	Error,
}

/// An enumeration of all possible origins of an error.
///
/// See <https://libvirt.org/html/libvirt-virterror.html#virErrorDomain>
#[napi]
pub enum ErrorDomain {
	/// No error.
	None = 0,
	/// Error at Xen hypervisor layer
	Xen,
	/// Error at connection with xend daemon
	Xend,
	/// Error at connection with xen store
	XenStore,
	/// Error in the S-Expression code
	SExpr,
	/// Error in the XML code
	Xml,
	/// Error when operating on a domain
	Dom,
	/// Error in the XML-RPC code
	Rpc,
	/// Error in the proxy code; unused since 0.8.6
	Proxy,
	/// Error in the configuration file handling
	Conf,
	/// Error at the QEMU daemon
	Qemu,
	/// Error when operating on a network
	Net,
	/// Error from test driver
	Test,
	/// Error from remote driver
	Remote,
	/// Error from OpenVZ driver
	OpenVz,
	/// Error at Xen XM layer
	XenXm,
	/// Error in the Linux Stats code
	StatsLinux,
	/// Error from Linux Container driver
	Lxc,
	/// Error from storage driver
	Storage,
	/// Error from network config
	Network,
	/// Error from domain config
	Domain,
	/// Error at the UML driver; unused since 5.0.0
	Uml,
	/// Error from node device monitor
	Nodedev,
	/// Error from xen inotify layer
	XenINotify,
	/// Error from security framework
	Security,
	/// Error from VirtualBox driver
	VBox,
	/// Error when operating on an interface
	Interface,
	/// The OpenNebula driver no longer exists. Retained for ABI/API compat only
	ONe,
	/// Error from ESX driver
	Esx,
	/// Error from the phyp driver, unused since 6.0.0
	Phyp,
	/// Error from secret storage
	Secret,
	/// Error from CPU driver
	Cpu,
	/// Error from XenAPI
	XenApi,
	/// Error from network filter driver
	Nwfilter,
	/// Error from Synchronous hooks
	Hook,
	/// Error from domain snapshot
	DomainSnapshot,
	/// Error from auditing subsystem
	Audit,
	/// Error from sysinfo/SMBIOS
	SysInfo,
	/// Error from I/O streams
	Streams,
	/// Error from VMware driver
	Vmware,
	/// Error from event loop impl
	Event,
	/// Error from libxenlight driver
	Libxl,
	/// Error from lock manager
	Locking,
	/// Error from Hyper-V driver
	HyperV,
	/// Error from capabilities
	Capabilities,
	/// Error from URI handling
	Uri,
	/// Error from auth handling
	Auth,
	/// Error from DBus
	Dbus,
	/// Error from Parallels
	Parallels,
	/// Error from Device
	Device,
	/// Error from libssh2 connection transport
	Ssh,
	/// Error from lockspace
	Lockspace,
	/// Error from initctl device communication
	Initctl,
	/// Error from identity code
	Identity,
	/// Error from cgroups
	Cgroup,
	/// Error from access control manager
	Access,
	/// Error from systemd code
	Systemd,
	/// Error from bhyve driver
	Bhyve,
	/// Error from crypto code
	Crypto,
	/// Error from firewall
	Firewall,
	/// Error from polkit code
	Polkit,
	/// Error from thread utils
	Thread,
	/// Error from admin backend
	Admin,
	/// Error from log manager
	Logging,
	/// Error from Xen xl config code
	XenXl,
	/// Error from perf
	Perf,
	/// Error from libssh connection transport
	Libssh,
	/// Error from resource control
	ResCtrl,
	/// Error from firewalld
	Firewalld,
	/// Error from domain checkpoint
	DomainCheckpoint,
	/// Error from TPM
	Tpm,
	/// Error from BPF code
	Bpf,
	/// Error from Cloud Hypervisor driver
	Ch,
	/// Indicates an error domain not yet supported by the Rust bindings
	Last,
}

/// An enumeration of all possible errors.
///
/// See <https://libvirt.org/html/libvirt-virterror.html#virErrorNumber>
#[napi]
pub enum ErrorNumber {
	/// No error.
	Ok = 0,
	/// Internal error
	InternalError,
	/// Memory allocation failure
	NoMemory,
	/// No support for this function
	NoSupport,
	/// Could not resolve hostname
	UnknownHost,
	/// Can't connect to hypervisor
	NoConnect,
	/// Invalid connection object
	InvalidConn,
	/// Invalid domain object
	InvalidDomain,
	/// Invalid function argument
	InvalidArg,
	/// A command to hypervisor failed
	OperationFailed,
	/// A HTTP GET command to failed
	GetFailed,
	/// A HTTP POST command to failed
	PostFailed,
	/// Unexpected HTTP error code
	HttpError,
	/// Failure to serialize an S-Expr
	SExprSerial,
	/// Could not open Xen hypervisor control
	NoXen,
	/// Failure doing an hypervisor call
	XenCall,
	/// Unknown OS type
	OsType,
	/// Missing kernel information
	NoKernel,
	/// Missing root device information
	NoRoot,
	/// Missing source device information
	NoSource,
	/// Missing target device information
	NoTarget,
	/// Missing domain name information
	NoName,
	/// Missing domain OS information
	NoOs,
	/// Missing domain devices information
	NoDevice,
	/// Could not open Xen Store control
	NoXenStore,
	/// Too many drivers registered
	DriverFull,
	/// Not supported by the drivers (DEPRECATED)
	CallFailed,
	/// An XML description is not well formed or broken
	XmlError,
	/// The domain already exist
	DomExist,
	/// Operation forbidden on read-only connections
	OperationDenied,
	/// Failed to open a conf file
	OpenFailed,
	/// Failed to read a conf file
	ReadFailed,
	/// Failed to parse a conf file
	ParseFailed,
	/// Failed to parse the syntax of a conf file
	ConfSyntax,
	/// Failed to write a conf file
	WriteFailed,
	/// Detail of an XML error
	XmlDetail,
	/// Invalid network object
	InvalidNetwork,
	/// The network already exist
	NetworkExist,
	/// General system call failure
	SystemError,
	/// Some sort of RPC error
	Rpc,
	/// Error from a GNUTLS call
	GnutlsError,
	/// Failed to start network
	NoNetworkStart,
	/// Domain not found or unexpectedly disappeared
	NoDomain,
	/// Network not found
	NoNetwork,
	/// Invalid MAC address
	InvalidMac,
	/// Authentication failed
	AuthFailed,
	/// Invalid storage pool object
	InvalidStoragePool,
	/// Invalid storage vol object
	InvalidStorageVol,
	/// Failed to start storage
	NoStorage,
	/// Storage pool not found
	NoStoragePool,
	/// Storage volume not found
	NoStorageVolume,
	/// Failed to start node driver
	NoNode,
	/// Invalid node device object
	InvalidNodeDevice,
	/// Node device not found
	NoNodeDevice,
	/// Security model not found
	NoSecurityModel,
	/// Operation is not applicable at this time
	OperationInvalid,
	/// Failed to start interface driver
	NoInterfaceStart,
	/// Interface driver not running
	NoInterface,
	/// Invalid interface object
	InvalidInterface,
	/// More than one matching interface found
	MultipleInterfaces,
	/// Failed to start nwfilter driver
	NoNwfilterStart,
	/// Invalid nwfilter object
	InvalidNwfilter,
	/// Nw filter pool not found
	NoNwfilter,
	/// Failed to build firewall
	BuildFirewall,
	/// Failed to start secret storage
	NoSecretStart,
	/// Invalid secret
	InvalidSecret,
	/// Secret not found
	NoSecret,
	/// Unsupported configuration construct
	ConfigUnsupported,
	/// Timeout occurred during operation
	OperationTimeout,
	/// A migration worked, but making the VM persist on the dest host failed
	MigratePersistFailed,
	/// A synchronous hook script failed
	HookScriptFailed,
	/// Invalid domain snapshot
	InvalidDomainSnapshot,
	/// Domain snapshot not found
	NoDomainSnapshot,
	/// Stream pointer not valid
	InvalidStream,
	/// Valid API use but unsupported by the given driver
	ArgumentUnsupported,
	/// Storage pool probe failed
	StorageProbeFailed,
	/// Storage pool already built
	StoragePoolBuilt,
	/// Force was not requested for a risky domain snapshot revert
	SnapshotRevertRisky,
	/// Operation on a domain was canceled/aborted by user
	OperationAborted,
	/// Authentication cancelled
	AuthCancelled,
	/// The metadata is not present
	NoDomainMetadata,
	/// Migration is not safe
	MigrateUnsafe,
	/// Integer overflow
	Overflow,
	/// Action prevented by block copy job
	BlockCopyActive,
	/// The requested operation is not supported
	OperationUnsupported,
	/// Error in ssh transport driver
	Ssh,
	/// Guest agent is unresponsive, not running or not usable
	AgentUnresponsive,
	/// Resource is already in use
	ResourceBusy,
	/// Operation on the object/resource was denied
	AccessDenied,
	/// Error from a dbus service
	DbusService,
	/// The storage vol already exists
	StorageVolExist,
	/// Given CPU is incompatible with host CPU
	CpuIncompatible,
	/// XML document doesn't validate against schema
	XmlInvalidSchema,
	/// Finish API succeeded but it is expected to return NULL
	MigrateFinishOk,
	/// Authentication unavailable
	AuthUnavailable,
	/// Server was not found
	NoServer,
	/// Client was not found
	NoClient,
	/// Guest agent replies with wrong id to guest-sync command (DEPRECATED)
	AgentUnsynced,
	/// Error in libssh transport driver
	Libssh,
	/// Fail to find the desired device
	DeviceMissing,
	/// Invalid nwfilter binding
	InvalidNwfilterBinding,
	/// No nwfilter binding
	NoNwfilterBinding,
	/// Invalid domain checkpoint
	InvalidDomainCheckpoint,
	/// Domain checkpoint not found
	NoDomainCheckpoint,
	/// Domain backup job id not found
	NoDomainBackup,
	/// Invalid network port object
	InvalidNetworkPort,
	/// The network port already exist
	NetworkPortExists,
	/// Network port not found
	NoNetworkPort,
	/// No domain's hostname found
	NoHostname,
	/// Checkpoint can't be used
	CheckpointInconsistent,
	/// More than one matching domain found
	MultipleDomains,
	/// Network metadata is not present
	NoNetworkMetadata,
	/// Indicates an error number not yet supported by the Rust bindings
	Last,
}

#[napi]
pub struct Error {
	pub code: u32,
	pub domain: u32,
	pub message: String,
	pub level: u32,
}

#[napi]
impl Error {
  #[napi]
	pub fn last_error() -> Self {
		let err = virt::error::Error::last_error();
		Error {
			code: err.code() as u32,
			domain: err.domain() as u32,
			message: err.to_string(),
			level: err.level() as u32,
		}
	}
}
