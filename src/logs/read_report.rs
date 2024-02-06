use crate::{datetime::DateTime, dict::area::Area, request::logs::res::Res};

use super::super::discord::{msg_handler::MsgHandler, send_report::SendReport};

pub struct ReadReport {
    msg_handler: MsgHandler,
    report: Res,
}

impl ReadReport {
    pub fn new(msg_handler: MsgHandler, report: Res) -> Self {
        Self {
            msg_handler,
            report,
        }
    }

    pub async fn report_handler(
        self,
        last_report: &mut u64,
        report_id: &str,
        areas: &Area,
    ) -> anyhow::Result<Option<SendReport>> {
        let report_data = self.report.get_data().get_report().report_data();
        let report_len = report_data.get_fights().len() as u64;

        let latest_report = report_data.get_fights().last();
        let is_kill = report_data.get_fights().last().unwrap().get_kill();
        let area_name = report_data.get_zone().get_name().to_string();
        let area_list = areas.get_areas();
        //レポートデータがない or trush or 現行零式 or 絶じゃないときは早期リターン
        let (unwrap_latest, unwrap_kill) = match (latest_report, is_kill) {
            (Some(l), Some(k)) if area_list.contains(&area_name) => (l, k),
            _ => {
                return Ok(None);
            }
        };

        let now_dt = DateTime::get_dt();
        let area_name = unwrap_latest.get_name().unwrap();
        let url = format!(
            "https://www.fflogs.com/reports/{}#fight={}",
            report_id, report_len
        );

        //killかwipeかでメッセージを変更
        let msg = if unwrap_kill {
            //kill時
            format!(
                "{}\\n**kill!**\\nエリア(フェーズ):{}\\n{}",
                now_dt, area_name, url
            )
        } else {
            //wipe時
            if let Some(phase) = unwrap_latest.get_phase_trasitions() {
                //フェーズがある場合
                let p = phase.last().unwrap().get_id();
                format!(
                    "{}\\n**wipe!**\\nエリア(フェーズ):{}(P{})\\n{}",
                    now_dt, area_name, *p, url
                )
            } else {
                //フェーズがない場合
                format!(
                    "{}\\n**wipe!**\\nエリア(フェーズ):{}\\n{}",
                    now_dt, area_name, url
                )
            }
        };

        //レポートが更新されていたらメッセージを送信
        if report_len > *last_report {
            *last_report = report_len;
            return Ok(Some(SendReport::new(self.msg_handler, msg)));
        }
        return Ok(None);
    }
}
