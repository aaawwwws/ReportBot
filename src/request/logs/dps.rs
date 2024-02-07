use super::{character::Character, role::Class};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Dps {
    characters: Vec<Character>,
}

impl Class for Dps {
    fn get_characters(&self) -> &Vec<Character> {
        &self.characters
    }
}
