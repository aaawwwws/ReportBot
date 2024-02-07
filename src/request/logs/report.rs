use serde::{Deserialize, Serialize};

use super::{fight::Fight, rankings::Rankings, zone::Zone};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Report {
    fights: Option<Vec<Fight>>,
    zone: Option<Zone>,
    rankings: Option<Rankings>,
}

impl Report {
    pub fn get_fights(&self) -> Option<&Vec<Fight>> {
        self.fights.as_ref()
    }

    pub fn get_zone(&self) -> Option<&Zone> {
        self.zone.as_ref()
    }
    pub fn get_rankings(&self) -> Option<Rankings> {
        self.rankings.clone()
    }
}
