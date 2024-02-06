use super::{auth_file::AuthFile, webhook::WebHook};
use std::{fs, io::Write};
pub struct ReadKey {
    logs_key: Option<String>,
}

impl ReadKey {
    pub fn new(logs_key: Option<String>) -> Self {
        Self { logs_key }
    }

    pub async fn read_key(self, path: &str) -> anyhow::Result<AuthFile> {
        if let Some(key) = self.logs_key {
            //初回起動
            let hook_key = WebHook::get_webhook().await?;
            let json_obj = AuthFile::new(key, hook_key);
            let mut file = fs::OpenOptions::new().create(true).write(true).open(path)?;
            let json_file = serde_json::to_string(&json_obj)?;
            file.write_all(json_file.as_bytes())?;
            return Ok(json_obj);
        } else {
            //二回目以降の起動
            let file = fs::read_to_string(&path)?;
            let json_obj: AuthFile = serde_json::from_str(&file)?;
            return Ok(json_obj);
        }
    }
}
