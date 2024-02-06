use crate::{discord::msg_handler::MsgHandler, file::input::Input};
pub struct WebHook;

impl WebHook {
    pub async fn get_webhook() -> anyhow::Result<String> {
        let input = loop {
            let input = Input::cin(Some("webhookを入力してください"))?;
            println!("接続のテストを行います");
            if !input.contains("discord") {
                println!("URLが間違っています");
                continue;
            }
            if let Ok(_) = MsgHandler::new(&input).send_msg("接続に成功しました").await {
                break input;
            } else {
                println!("接続に失敗しました。もう一度入力してください");
            }
        };
        Ok(input)
    }
}
