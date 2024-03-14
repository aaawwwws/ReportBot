use base64::engine::general_purpose;

#[derive(Debug)]
pub struct ReportCount {
    count: u64,
}

impl ReportCount {
    pub fn new() -> Self {
        Self { count: 0 }
    }

    pub fn set_count(&self, count: u64) -> Self {
        Self {
            count,
        }
    }

    pub fn get_count(&self) -> &u64 {
        &self.count
    }
}
