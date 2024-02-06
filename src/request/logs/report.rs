use serde::{Deserialize, Serialize};

use super::{fight::Fight, zone::Zone};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Report {
    fights: Vec<Fight>,
    zone: Zone,
}

impl Report {
    pub fn get_fights(&self) -> &Vec<Fight> {
        &self.fights
    }

    pub fn get_zone(&self) -> &Zone {
        &self.zone
    }
}
