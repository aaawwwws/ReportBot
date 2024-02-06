use serde::{Deserialize, Serialize};

use super::report_data::ReportData;
#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct Data {
    reportData: ReportData,
}

impl Data {
    pub fn get_report (&self) -> &ReportData {
        &self.reportData
    }
}