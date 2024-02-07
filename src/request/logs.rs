use crate::request::query_graphql::QeuryGraphql;
use reqwest::Client;
pub mod data;
pub mod fight;
pub mod id;
pub mod report;
pub mod report_data;
pub mod res;
pub mod zone;
pub mod rankings;
pub mod role;
pub mod tank;
pub mod healer;
pub mod dps;
pub mod character;
pub mod rankigs_data;
pub mod datum;
pub struct Logs {
    secret_key: String,
    report_id: String,
}

impl Logs {
    pub fn new(secret_key: &str, log_url: &str) -> Self {
        Self {
            secret_key: String::from(secret_key),
            report_id: String::from(log_url),
        }
    }

    pub async fn get_report(&self, query: QeuryGraphql) -> anyhow::Result<res::Res> {
        const END_POINT: &str = "https://www.fflogs.com/api/v2/client";
        let c = serde_json::to_string(&query).unwrap();
        let res = Client::new()
            .post(END_POINT)
            .bearer_auth(&self.secret_key)
            .header("Content-Type", "application/json")
            .body(c.clone())
            .send()
            .await?;
        let res_str = res.json::<res::Res>().await?;
        return Ok(res_str);
    }
}
