use serde::{Deserialize, Serialize};

use super::datum::Datum;

#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct Rankings {
    data: Vec<Datum>,
}

impl Rankings {
    pub fn get_rankings(&self) -> &Vec<Datum> {
        &self.data
    }
}
