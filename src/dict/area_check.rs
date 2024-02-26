use super::area::AreaName;

pub struct AreaCheck {
    area_list: Vec<AreaName>,
}

impl AreaCheck {
    pub fn new(area_list: Vec<AreaName>) -> Self {
        Self { area_list }
    }

    pub fn is_area(&self, area_name: &str) -> bool {
        for area in &self.area_list {
            if area.get_name().eq(area_name) {
                return true
            }
        }
        false
    }
}
