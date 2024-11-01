use napi;
use virt;

#[napi]
pub struct NWFilter {
    nw_filter: virt::nwfilter::NWFilter
}

impl NWFilter {
    pub fn get(&self) -> &virt::nwfilter::NWFilter {
        &self.nw_filter
    }

    pub fn from_nw_filter(nw_filter: virt::nwfilter::NWFilter) -> Self {
        Self { nw_filter }
    }
}
