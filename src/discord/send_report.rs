use super::msg_handler::MsgHandler;

pub struct SendReport {
    msg_handler: MsgHandler,
    msg: String,
}

impl SendReport {
    pub fn new(msg_handler: MsgHandler, msg: String) -> Self {
        Self { msg_handler, msg }
    }

    pub async fn send_msg(&self) -> anyhow::Result<()> {
        let _ = &self.msg_handler.send_msg(&self.msg).await?;
        return Ok(());
    }
}
