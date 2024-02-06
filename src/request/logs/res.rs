use serde::{Deserialize, Serialize};

use super::data::Data;

#[derive(Serialize, Deserialize, Debug)]
pub struct Res {
    data: Data,
}

impl Res {
    pub fn get_data(&self) -> &Data {
        &self.data
    }
}