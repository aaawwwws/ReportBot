use crate::file::{input::Input, uploader::Uploader};
use auth::logs_key;
use std::{cell::RefCell, f32::consts::E, rc::Rc, time::Duration};
use tokio::time::sleep;
mod auth;
mod datetime;
mod dict;
mod discord;
mod file;
mod graph;
mod logs;
mod request;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    const TICK: u64 = 1;
    const FILE_PATH: &str = "auth.json";

    let mut last_report = logs::report_count::ReportCount::new();
    //アップローダーを起動
    let _ = Uploader.open_uploader()?;

    let mut wipe_vec: Vec<usize> = Vec::new();

    let auth = logs_key::LogsKey::get_key(FILE_PATH)
        .await?
        .read_key(FILE_PATH)
        .await?;

    let report_id = Input::url_input()?;

    //エリア情報別ファイルにする予定。
    let areas = dict::area::AreaList::area_list().await?;
    //job置き換え
    let job_list = dict::job::Job::new();

    //mainloop
    loop {
        let request = auth.req_logs(&report_id).await?;
        let (send_msg, phase) = request
            .report_handler(&mut last_report, &report_id, &areas, &job_list, &wipe_vec)
            .await?;
        match (send_msg, phase) {
            //更新とグラフ共にある時
            (Some(sm), Some(phase)) => {
                sm.send_msg().await?;
                wipe_vec.push(*phase.get_phase());
                phase.create_graph(&wipe_vec)?
            }
            //更新のみある時
            (Some(sm), None) => sm.send_msg().await?,
            //無
            (None, _) => {}
        };
        sleep(Duration::from_secs(TICK)).await;
    }
}
