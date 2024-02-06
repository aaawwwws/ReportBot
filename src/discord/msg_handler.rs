use reqwest::Client;

pub struct MsgHandler {
    hook: String,
}

impl MsgHandler {
    pub fn new(end_point: &str) -> Self {
        Self {
            hook: String::from(end_point),
        }
    }
    pub async fn send_msg(&self, msg: &str) -> anyhow::Result<&Self> {
        let query = format!(
            r#"{{
            "content": "{}"
            }}"#,
            msg
        );
        let client = Client::new();
        let Ok(_) = client
            .post(&self.hook)
            .header("Content-Type", "application/json")
            .body(query)
            .send()
            .await
        else {
            return Err(anyhow::anyhow!("接続できませんでした。"));
        };
        return Ok(self);
    }
}
