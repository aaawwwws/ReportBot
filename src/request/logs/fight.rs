use serde::{Deserialize, Serialize};

use super::id::Id;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct Fight {
    id: Option<u64>,
    kill: Option<bool>,
    name: Option<String>,
    difficulty: Option<u64>,
    phaseTransitions: Option<Vec<Id>>,
}

impl Fight {
    pub fn get_id(&self) -> Option<u64> {
        self.id.clone()
    }
    pub fn get_kill(&self) -> Option<bool> {
        //trushの場合null(none)になる。
        self.kill.clone()
    }
    pub fn get_name(&self) -> Option<String> {
        self.name.clone()
    }
    pub fn get_difficulty(&self) -> Option<u64> {
        return self.difficulty.clone();
    }
    pub fn get_phase_trasitions(&self) -> Option<Vec<Id>> {
        self.phaseTransitions.clone()
    }
}
