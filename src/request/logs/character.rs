use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct Character {
    name: String,
    class: String,
    rankPercent: u8,
}

impl Character {
    pub fn get_name (&self) -> &str {
        &self.name
    }

    pub fn get_class (&self) -> &str {
        &self.class
    }

    pub fn get_percent (&self) -> &u8 {
        &self.rankPercent
    }
}