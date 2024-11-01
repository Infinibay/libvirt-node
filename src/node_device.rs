use napi;
use virt;

#[napi]
pub struct NodeDevice {
	node: virt::nodedev::NodeDevice
}

impl NodeDevice {
	pub fn get(&self) -> &virt::nodedev::NodeDevice {
		&self.node
	}

	pub fn from_node(node: virt::nodedev::NodeDevice) -> Self {
		Self { node: node }
	}
}