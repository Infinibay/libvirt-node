use virt;

#[napi]
pub struct NodeInfo {
    node_info: virt::connect::NodeInfo
}

impl NodeInfo {
    pub fn get(&self) -> &virt::connect::NodeInfo {
        &self.node_info
    }

    pub fn from_node_info(node_info: virt::connect::NodeInfo) -> Self {
        Self { node_info: node_info }
    }
}
