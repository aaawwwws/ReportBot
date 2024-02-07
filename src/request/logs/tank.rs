use super::{character::Character, role::Class};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tank {
    characters: Vec<Character>,
}

impl Class for Tank {
    fn get_characters(&self) -> &Vec<Character> {
        &self.characters
    }
}
