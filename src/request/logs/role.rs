use serde::{Deserialize, Serialize};

use super::{character::Character, dps::Dps, healer::Healer, tank::Tank};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Role {
    tanks: Tank,
    healers: Healer,
    dps: Dps,
}

impl Role {
    pub fn get_tanks(&self) -> &Tank {
        &self.tanks
    }

    pub fn get_healers(&self) -> &Healer {
        &self.healers
    }

    pub fn get_dps(&self) -> &Dps {
        &self.dps
    }
}

pub trait Class {
    fn get_characters(&self) -> &Vec<Character>;
}
