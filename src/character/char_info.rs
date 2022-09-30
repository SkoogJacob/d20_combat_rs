use crate::Error;
use crate::character::{class_stats::ClassStats, ancestry_stats::AncestryStats};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct CharacterBase {
    ancestry: CharacterAncestry,
    class: CharacterClass,
}
impl CharacterBase {
    pub fn new(ancestry: CharacterAncestry, class: CharacterClass) -> CharacterBase {
        CharacterBase {
            ancestry,
            class
        }
    }
    pub fn get_ancestry_stats(&self) -> AncestryStats {
        AncestryStats::new(self.ancestry.clone())
    }
    pub fn get_class_stats(&self) -> ClassStats {
        ClassStats::new(self.class.clone())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub enum CharacterClass {
    Fighter,
    Rogue,
    Ranger,
}
impl From<CharacterClass> for &'static str {
    fn from(c: CharacterClass) -> Self {
        match c {
            CharacterClass::Fighter => "Fighter",
            CharacterClass::Rogue => "Rogue",
            CharacterClass::Ranger => "Ranger",
        }
    }
}
impl TryFrom<&str> for CharacterClass {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "fighter" => Ok(CharacterClass::Fighter),
            "rogue" => Ok(CharacterClass::Rogue),
            "ranger" => Ok(CharacterClass::Ranger),
            _ => Err(Error::BadClassInput(String::from(value)))
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub enum CharacterAncestry {
    Human,
    Elf,
    Dwarf,
}
impl TryFrom<&str> for CharacterAncestry {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "human" => Ok(CharacterAncestry::Human),
            "elf" => Ok(CharacterAncestry::Elf),
            "dwarf" => Ok(CharacterAncestry::Dwarf),
            _ => Err(Error::BadAncestryInput(String::from(value)))
        }
    }
}
impl From<CharacterAncestry> for &'static str {
    fn from(val: CharacterAncestry) -> Self {
        match val {
            CharacterAncestry::Human => "Human",
            CharacterAncestry::Elf => "Elf",
            CharacterAncestry::Dwarf => "Dwarf",
        }
    }
}
