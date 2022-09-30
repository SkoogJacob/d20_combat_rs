use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct CharacterStats {
    hit_points_per_level: u8,
    initial_class_ability_boosts: Vec<String>,
    fortitude_prof_vec: Vec<u8>,
    reflex_prof_vec: Vec<u8>,
    will_prof_vec: Vec<u8>,
    heavy_armour_training: bool,
    medium_armour_training: bool,
    light_armour_training: bool,
    unarmoured_training: bool,
}

impl CharacterStats {
    pub fn new()
}
