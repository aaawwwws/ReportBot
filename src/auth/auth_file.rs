use serde::{Deserialize, Serialize};

use crate::{
    discord::msg_handler::{self, MsgHandler},
    file::input::Input,
    logs::read_report::ReadReport,
    request::{logs, query_graphql::QeuryGraphql},
};

#[derive(Serialize, Deserialize, Debug)]

pub struct AuthFile {
    logs_key: String,
    hook_key: String,
}

impl AuthFile {
    pub fn new(logs_key: String, hook_key: String) -> Self {
        Self { logs_key, hook_key }
    }

    pub async fn req_logs(&self, report_id: &str) -> anyhow::Result<ReadReport> {
        let q = format!("{{ reportData {{ report(code:\"{}\") {{ zone {{ name }} fights {{ id, kill, name, phaseTransitions {{ id }} }} }} }} }}", report_id);
        let query = QeuryGraphql { query: q };
        let logs = logs::Logs::new(&self.logs_key, &report_id);
        let report = logs.get_report(query).await?;
        return Ok(ReadReport::new(
            MsgHandler::new(&self.hook_key),
            report,
            &self.logs_key,
        ));
    }
}
