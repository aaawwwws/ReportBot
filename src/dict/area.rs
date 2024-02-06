pub struct Area {
    array: [String; 6],
}

impl Area {
    pub fn new() -> Self {
        let array = [
            String::from("Anabaseios"),
            String::from("The Omega Protocol"),
            String::from("Dragonsong's Reprise"),
            String::from("The Unending Coil of Bahamut"),
            String::from("The Weapon's Refrain"),
            String::from("The Epic of Alexander"),
        ];
        Self { array }
    }
    pub fn get_areas(&self) -> &[String; 6] {
        &self.array
    }
}
