use serde::{Deserialize, Serialize};

use super::read_key::ReadKey;
use crate::file::{file_check, input::Input};

pub struct LogsKey;

impl LogsKey {
    pub async fn get_key(path: &str) -> anyhow::Result<ReadKey> {
        if !file_check::FileCeck::is_file(path) {
            //ファイルがない場合の初回起動
            let client_id = Input::cin(Some("client_idを入力してください"))?;
            let client_secret = Input::cin(Some("client_secretを入力してください"))?;
            println!("{}::{}", &client_id, &client_secret);
            let client = reqwest::Client::new();
            let res = client
                .post("https://ja.fflogs.com/oauth/token")
                .basic_auth(client_id, Some(client_secret))
                .form(&[("grant_type", "client_credentials")])
                .send()
                .await?;
            println!("{:?}", res);
            let res_auth: AuthRes = res.json().await?;
            return Ok(ReadKey::new(Some(res_auth.access_token)));
        }
        return Ok(ReadKey::new(None));
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthRes {
    pub access_token: String,
    expires_in: i64,
    token_type: String,
}
