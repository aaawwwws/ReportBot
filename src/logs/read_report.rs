use std::{cell::RefCell, rc::Rc};

use crate::{
    datetime::DateTime,
    dict::{
        area::{Area, AreaName},
        area_check::AreaCheck,
        job::Job,
    },
    graph,
    logs::report_count,
    request::{
        logs::{
            character::{self, Character},
            datum::Datum,
            dps::Dps,
            healer::Healer,
            res::Res,
            role::{Class, Role},
            tank::Tank,
            Logs,
        },
        query_graphql::{self, QeuryGraphql},
    },
};

use super::{
    super::discord::{msg_handler::MsgHandler, send_report::SendReport},
    ranking_data::{self, RankingData},
};

pub struct ReadReport {
    msg_handler: MsgHandler,
    report: Res,
    logs_key: String,
}

impl ReadReport {
    pub fn new(msg_handler: MsgHandler, report: Res, logs_key: &str) -> Self {
        Self {
            msg_handler,
            report,
            logs_key: String::from(logs_key),
        }
    }

    pub async fn report_handler(
        self,
        last_report: &mut report_count::ReportCount,
        report_id: &str,
        areas: &AreaCheck,
        job_list: &Job,
        wipe_vec: &Vec<usize>,
    ) -> anyhow::Result<(Option<SendReport>, Option<graph::wipe_graph::WipeGraph>)> {
        let report_data = self.report.get_data().get_report().report_data();
        let report_len = report_data.get_fights().unwrap().len() as u64;
        let latest_report = report_data.get_fights().unwrap().last();
        let is_kill = report_data.get_fights().unwrap().last().unwrap().get_kill();
        //レポートデータがない or trush or 現行零式 or 絶じゃないときは早期リターン
        let (unwrap_latest, unwrap_kill) = match (latest_report, is_kill) {
            (Some(l), Some(k)) => (l, k),
            _ => {
                return Ok((None, None));
            }
        };

        let Some(last_area) = unwrap_latest.get_name() else {
            return Ok((None, None));
        };

        if !areas.is_area(&last_area) {
            return Ok((None, None));
        }

        //更新されていない場合早期リターン
        if report_len <= *last_report.get_count() {
            println!("return {:?}", *last_report);
            return Ok((None, None));
        }

        *last_report = last_report.set_count(report_len);

        let now_dt = DateTime::get_dt();
        let area_name = unwrap_latest.get_name().unwrap();
        let url = format!(
            "https://ja.fflogs.com/reports/{}#fight={}",
            report_id, report_len
        );

        //killかwipeかでメッセージを変更
        let (msg, phase) = match (unwrap_kill, unwrap_latest.get_phase_trasitions()) {
            //kill時
            (true, _) => {
                (format!(
                "{}\\n**kill!**\\nエリア(フェーズ):{}\\n{}\\n{}",
                now_dt, area_name, self.req_ranking(report_id,*last_report.get_count(), job_list).await?,url),None)}
            ,
            //ワイプしてフェーズがない場合
            (false, None) => {
                (format!(
                    "{}\\n**wipe!**\\nエリア(フェーズ):{}\\n{}",
                    now_dt, area_name, url
                ),None)
            }
            //ワイプしてフェーズがある場合
            (false, Some(phase)) => {
                let p = phase.len();
                //グラフを作る
                (format!(
                    "{}\\n**wipe!**\\nエリア(フェーズ):{}(P{})\\n{}",
                    now_dt, area_name, p, url
                ),Some(graph::wipe_graph::WipeGraph::new(p,&area_name)))
            }
        };

        Ok((Some(SendReport::new(self.msg_handler, msg)), phase))
    }

    async fn req_ranking(
        &self,
        report_id: &str,
        last_fight: u64,
        job_list: &Job,
    ) -> anyhow::Result<String> {
        let logs = Logs::new(&self.logs_key, report_id);
        let query = format!(
            "{{
            reportData {{
              report(code: \"{}\") {{
                rankings(fightIDs: [{}])
              }}
            }}
          }}",
            report_id, last_fight
        );
        let res = logs.get_report(QeuryGraphql { query }).await?;
        let Some(unwrap_rankings) = res.get_data().get_report().report_data().get_rankings() else {
            return Err(anyhow::anyhow!("ランキングがありません"));
        };

        let role = unwrap_rankings.get_rankings().last().unwrap().get_roles();
        let tank = role.get_tanks();
        let healer = role.get_healers();
        let dps = role.get_dps();

        let mut ranking_data = String::new();

        self.ranking_str::<Tank>(&tank, &mut ranking_data, job_list);
        self.ranking_str::<Healer>(&healer, &mut ranking_data, job_list);
        self.ranking_str::<Dps>(&dps, &mut ranking_data, job_list);
        Ok(ranking_data)
    }

    fn ranking_str<T: Class>(&self, role: &T, str: &mut String, job_list: &Job) {
        for i in role.get_characters() {
            if str.contains(i.get_name()) {
                break;
            };
            let mut job_replace = String::new();
            for (k, v) in job_list.get_dict() {
                if i.get_class().eq(k) {
                    job_replace.push_str(v);
                    break;
                }
            }
            let player = format!(
                "- {} {} perf:**{}** \\n",
                job_replace,
                i.get_name(),
                i.get_percent()
            );
            str.push_str(&player);
        }
    }
}
