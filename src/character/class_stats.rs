use serde::{Serialize, Deserialize};
use super::CharacterClass;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct ClassStats {
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

impl ClassStats {
    pub fn new(class: CharacterClass) -> ClassStats {
        let hit_points_per_level = match &class {
            CharacterClass::Fighter => 10,
            CharacterClass::Rogue => 8,
            CharacterClass::Ranger => 10,
        };
        let initial_class_ability_boosts = match class {
            CharacterClass::Fighter => vec![String::from("Punch")],
            CharacterClass::Rogue => vec![String::from("Stab")],
            CharacterClass::Ranger => vec![String::from("Shoot")],
        };
        let fortitude_prof_vec: Vec<u8> = match &class {
            CharacterClass::Fighter |
            CharacterClass::Ranger => vec![4],
            CharacterClass::Rogue => vec![2],
        };
        let reflex_prof_vec = vec![4];
        let will_prof_vec = match &class {
            CharacterClass::Fighter |
            CharacterClass::Ranger => vec![2],
            CharacterClass::Rogue => vec![4],
        };
        let heavy_armour_training = match &class {
            CharacterClass::Fighter => true,
            CharacterClass::Rogue |
            CharacterClass::Ranger => false,
        };
        let medium_armour_training = match &class {
            CharacterClass::Fighter |
            CharacterClass::Ranger => true,
            CharacterClass::Rogue => false,
        };

        ClassStats {
            hit_points_per_level,
            initial_class_ability_boosts,
            fortitude_prof_vec,
            reflex_prof_vec,
            will_prof_vec,
            heavy_armour_training,
            medium_armour_training,
            light_armour_training: true,
            unarmoured_training: true,
        }
    }

    pub fn hit_points_per_level(&self) -> u8 {
        self.hit_points_per_level
    }

    pub fn initial_class_ability_boosts(&self) -> &[String] {
        self.initial_class_ability_boosts.as_ref()
    }

    pub fn fortitude_prof_vec(&self) -> &[u8] {
        self.fortitude_prof_vec.as_ref()
    }

    pub fn reflex_prof_vec(&self) -> &[u8] {
        self.reflex_prof_vec.as_ref()
    }

    pub fn will_prof_vec(&self) -> &[u8] {
        self.will_prof_vec.as_ref()
    }

    pub fn heavy_armour_training(&self) -> bool {
        self.heavy_armour_training
    }

    pub fn medium_armour_training(&self) -> bool {
        self.medium_armour_training
    }

    pub fn light_armour_training(&self) -> bool {
        self.light_armour_training
    }

    pub fn unarmoured_training(&self) -> bool {
        self.unarmoured_training
    }
}

