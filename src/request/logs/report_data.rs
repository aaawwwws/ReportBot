use serde::{Deserialize, Serialize};

use super::report::Report;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReportData {
    report:Report,
}

impl ReportData {
    pub fn report_data(&self) -> &Report {
        &self.report
    }
}