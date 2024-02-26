use reqwest::Client;
use serde::Deserialize;

use super::area_check::AreaCheck;

pub struct AreaList;

impl AreaList {
    pub async fn area_list() -> anyhow::Result<AreaCheck> {
        let client = Client::new();
        let res = client
            .get("https://raw.githubusercontent.com/aaawwwws/ReportBot/master/area.json")
            .send()
            .await?;
        let result = res.json::<Area>().await?.area;
        return Ok(AreaCheck::new(result));
    }
}

#[derive(Debug, Deserialize)]
pub struct Area {
    area: Vec<AreaName>,
}
#[derive(Debug, Deserialize)]
pub struct AreaName {
    name: String,
}

impl AreaName {
    pub fn get_name(&self) -> &str {
        &self.name
    }
}
