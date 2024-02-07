use serde::{Deserialize, Serialize};

use super::{character::Character, role::Class};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Healer {
    characters:Vec<Character>
}

impl Class for Healer {
    fn get_characters(&self) -> &Vec<Character> {
        &self.characters
    }
}
