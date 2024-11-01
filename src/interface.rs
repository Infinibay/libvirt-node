use napi;
use virt;

#[napi]
pub struct Interface {
	interface: virt::interface::Interface
}

impl Interface {
	pub fn get(&self) -> &virt::interface::Interface {
		&self.interface
	}

	pub fn from_interface(int: virt::interface::Interface) -> Self {
		Self { interface: int }
	}
}