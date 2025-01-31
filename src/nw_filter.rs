use napi;
use virt;

use crate::connection::Connection;

#[napi]
pub struct NWFilter {
    nw_filter: virt::nwfilter::NWFilter
}

#[napi]
impl NWFilter {
    pub fn get(&self) -> &virt::nwfilter::NWFilter {
        &self.nw_filter
    }

    pub fn from_nw_filter(nw_filter: virt::nwfilter::NWFilter) -> Self {
        Self { nw_filter }
    }

    #[napi]
    pub fn lookup_by_name(conn: &Connection, name: String) -> Option<NWFilter> {
        match virt::nwfilter::NWFilter::lookup_by_name(conn.get_connection(), &name) {
            Ok(nw_filter) => Some(NWFilter { nw_filter }),
            Err(_) => None,
        }
    }

    #[napi]
    pub fn lookup_by_uuid_string(conn: &Connection, uuid: String) -> Option<NWFilter> {
        match virt::nwfilter::NWFilter::lookup_by_uuid_string(conn.get_connection(), &uuid) {
            Ok(nw_filter) => Some(NWFilter { nw_filter }),
            Err(_) => None,
        }
    }

    #[napi]
    pub fn get_name(&self) -> Option<String> {
        match self.nw_filter.get_name() {
            Ok(name) => Some(name),
            Err(_) => None,
        }
    }

    #[napi]
    pub fn get_uuid_string(&self) -> Option<String> {
        match self.nw_filter.get_uuid_string() {
            Ok(uuid) => Some(uuid),
            Err(_) => None,
        }
    }

    #[napi]
    pub fn get_xml_desc(&self, flags: u32) -> Option<String> {
        match self.nw_filter.get_xml_desc(flags) {
            Ok(xml) => Some(xml),
            Err(_) => None,
        }
    }

    #[napi]
    pub fn define_xml(conn: &Connection, xml: String) -> Option<NWFilter> {
        match virt::nwfilter::NWFilter::define_xml(conn.get_connection(), &xml) {
            Ok(nw_filter) => Some(NWFilter { nw_filter }),
            Err(_) => None,
        }
    }

    #[napi]
    pub fn undefine(&self) -> Option<u32> {
        match self.nw_filter.undefine() {
            Ok(_) => Some(0),
            Err(_) => None,
        }
    }
}
