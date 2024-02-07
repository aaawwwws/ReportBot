pub struct RankingData {
    name: String,
    perf: u8,
}

impl RankingData {
    pub fn new(name: impl Into<String>, perf: u8) -> Self {
        Self {
            name: name.into(),
            perf,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_perf(&self) -> &u8 {
        &self.perf
    }
}
