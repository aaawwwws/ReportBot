use std::collections::HashMap;
pub struct Job {
    job_dict: HashMap<String, String>,
}

impl Job {
    pub fn new() -> Self {
        //無駄だな
        let mut job_dict: HashMap<String, String> = HashMap::new();
        job_dict.insert("Astrologian".to_string(), "占".to_string());
        job_dict.insert("Bard".to_string(), "詩人".to_string());
        job_dict.insert("BlackMage".to_string(), "黒".to_string());
        job_dict.insert("Dancer".to_string(), "踊".to_string());
        job_dict.insert("DarkKnight".to_string(), "暗".to_string());
        job_dict.insert("Dragoon".to_string(), "竜".to_string());
        job_dict.insert("Gunbreaker".to_string(), "ガ".to_string());
        job_dict.insert("Machinist".to_string(), "機".to_string());
        job_dict.insert("Monk".to_string(), "モ".to_string());
        job_dict.insert("Ninja".to_string(), "忍".to_string());
        job_dict.insert("Paladin".to_string(), "ナ".to_string());
        job_dict.insert("Reaper".to_string(), "リ".to_string());
        job_dict.insert("RedMage".to_string(), "赤".to_string());
        job_dict.insert("Sage".to_string(), "賢".to_string());
        job_dict.insert("Samurai".to_string(), "侍".to_string());
        job_dict.insert("Scholar".to_string(), "学".to_string());
        job_dict.insert("Summoner".to_string(), "召".to_string());
        job_dict.insert("Warrior".to_string(), "戦".to_string());
        job_dict.insert("WhiteMage".to_string(), "白".to_string());
        Self { job_dict }
    }

    pub fn get_dict(&self) -> &HashMap<String, String> {
        &self.job_dict
    }
}
