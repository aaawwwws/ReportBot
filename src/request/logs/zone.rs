use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Zone {
    name: String,
}

impl Zone {
    pub fn get_name(&self) -> &str {
        &self.name
    }
}
