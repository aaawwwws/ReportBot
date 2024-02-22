use crate::file::{input::Input, uploader::Uploader};
use auth::logs_key;
use std::time::Duration;
use tokio::time::sleep;
mod auth;
mod datetime;
mod dict;
mod discord;
mod file;
mod logs;
mod request;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    const FILE_PATH: &str = "auth.json";
    let mut last_report: u64 = 0;
    //アップローダーを起動
    let _ = Uploader.open_uploader()?;

    let auth = logs_key::LogsKey::get_key(FILE_PATH)
        .await?
        .read_key(FILE_PATH)
        .await?;

    let report_id = Input::url_input()?;

    //エリア情報別ファイルにする予定。
    let areas = dict::area::Area::new();
    //job置き換え
    let job_list = dict::job::Job::new();

    //mainloop
    loop {
        if let Some(send_msg) = auth
            .req_logs(&report_id)
            .await?
            .report_handler(&mut last_report, &report_id, &areas, &job_list)
            .await?
        {
            send_msg.send_msg().await?
        }
        sleep(Duration::from_secs(1)).await;
    }
}
