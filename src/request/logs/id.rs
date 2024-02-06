use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Id {
    id:u64,
}

impl Id {
    pub fn get_id(&self) -> &u64 {
        &self.id
    }
}