use virt;

#[napi]
pub struct DomainStatsRecord {
    stat: virt::domain::DomainStatsRecord,
}

impl DomainStatsRecord {
    pub fn from_stat(stat: virt::domain::DomainStatsRecord) -> Self {
        Self { stat }
    }

    pub fn get(&self) -> &virt::domain::DomainStatsRecord {
        &self.stat
    }
}
