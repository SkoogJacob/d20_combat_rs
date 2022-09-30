use crate::CharacterAncestry;

pub struct AncestryStats {
    initial_hit_points: u8,
    speed: u8,
    ancestral_boost_strength: u8,
    ancestral_boost_dexterity: u8,
    ancestral_boost_constitution: u8,
    ancestral_boost_intelligence: u8,
    ancestral_boost_wisdom: u8,
    ancestral_boost_charisma: u8,
    ancestral_ability_boosts: u8,
    ancestral_bonus_feats: u8,
    size: Size,
    subtype: String,
    valid_languages: Vec<String>,
    languages: Vec<String>,
    immunities: Vec<String>,
    ancestral_boost_language: u8,
    vision_type: String
}

impl AncestryStats {
    pub fn new(ancestry: CharacterAncestry) -> AncestryStats {
        let initial_hit_points = match &ancestry {
            CharacterAncestry::Human => 17,
            CharacterAncestry::Elf => 14,
            CharacterAncestry::Dwarf => 20,
        };
        let speed = match &ancestry {
            CharacterAncestry::Human => 4,
            CharacterAncestry::Elf => 5,
            CharacterAncestry::Dwarf => 25,
        };
        let ancestral_boost_strength = match &ancestry {
            CharacterAncestry::Human => 0,
            CharacterAncestry::Elf => 0,
            CharacterAncestry::Dwarf => 2,
        };
        let ancestral_boost_dexterity = match &ancestry {
            CharacterAncestry::Human => 0,
            CharacterAncestry::Elf => 1,
            CharacterAncestry::Dwarf => 0,
        };
        let ancestral_boost_constitution = match &ancestry {
            CharacterAncestry::Human => 0,
            CharacterAncestry::Elf => 0,
            CharacterAncestry::Dwarf => 2,
        };
        let ancestral_boost_intelligence = match &ancestry {
            CharacterAncestry::Human => 1,
            CharacterAncestry::Elf => 1,
            CharacterAncestry::Dwarf => 0,
        };
        let ancestral_boost_wisdom = match &ancestry {
            CharacterAncestry::Human => 0,
            CharacterAncestry::Elf => 1,
            CharacterAncestry::Dwarf => 0,
        };
        let ancestral_boost_charisma = match &ancestry {
            CharacterAncestry::Human => 2,
            CharacterAncestry::Elf => 1,
            CharacterAncestry::Dwarf => 0,
        };
        let ancestral_ability_boosts = match &ancestry {
            CharacterAncestry::Human => 0,
            CharacterAncestry::Elf => 0,
            CharacterAncestry::Dwarf => 0,
        };
        let ancestral_bonus_feats = match &ancestry {
            CharacterAncestry::Human => 0,
            CharacterAncestry::Elf => 0,
            CharacterAncestry::Dwarf => 0,
        };
        let size = match &ancestry {
            CharacterAncestry::Human => Size::Medium,
            CharacterAncestry::Elf => Size::Medium,
            CharacterAncestry::Dwarf => Size::Medium,
        };
        let subtype = match &ancestry {
            CharacterAncestry::Human => String::from("Human Human"),
            CharacterAncestry::Elf => String::from("Legolas"),
            CharacterAncestry::Dwarf => String::from("Mountain Dwarf"),
        };
        let valid_languages = match &ancestry {
            CharacterAncestry::Human => vec![String::from("Common")],
            CharacterAncestry::Elf => vec![String::from("Common"), String::from("Elvish")],
            CharacterAncestry::Dwarf => vec![String::from("Common"), String::from("Dwarvish")],
        };
        let languages = match &ancestry {
            CharacterAncestry::Human => vec![String::from("Common")],
            CharacterAncestry::Elf => vec![String::from("Common"), String::from("Elvish")],
            CharacterAncestry::Dwarf => vec![String::from("Common"), String::from("Dwarvish")],
        };
        let immunities = match &ancestry {
            CharacterAncestry::Human => vec![String::from("Pebbles")],
            CharacterAncestry::Elf => vec![String::from("Night lights")],
            CharacterAncestry::Dwarf => vec![String::from("Poison")],
        };
        let ancestral_boost_language = match &ancestry {
            CharacterAncestry::Human => 2,
            CharacterAncestry::Elf => 0,
            CharacterAncestry::Dwarf => 0,
        };
        let vision_type = match &ancestry {
            CharacterAncestry::Human => String::from("Normal"),
            CharacterAncestry::Elf => String::from("Darkvision"),
            CharacterAncestry::Dwarf => String::from("Darkvision"),
        };

        AncestryStats {
            initial_hit_points,
            speed,
            ancestral_boost_strength,
            ancestral_boost_dexterity,
            ancestral_boost_constitution,
            ancestral_boost_intelligence,
            ancestral_boost_wisdom,
            ancestral_boost_charisma,
            ancestral_ability_boosts,
            ancestral_bonus_feats,
            size,
            subtype,
            valid_languages,
            languages,
            immunities,
            ancestral_boost_language,
            vision_type
        }
    }



    pub fn initial_hit_points(&self) -> u8 {
        self.initial_hit_points
    }

    pub fn speed(&self) -> u8 {
        self.speed
    }

    pub fn ancestral_boost_strength(&self) -> u8 {
        self.ancestral_boost_strength
    }

    pub fn ancestral_boost_dexterity(&self) -> u8 {
        self.ancestral_boost_dexterity
    }

    pub fn ancestral_boost_constitution(&self) -> u8 {
        self.ancestral_boost_constitution
    }

    pub fn ancestral_boost_intelligence(&self) -> u8 {
        self.ancestral_boost_intelligence
    }

    pub fn ancestral_boost_wisdom(&self) -> u8 {
        self.ancestral_boost_wisdom
    }

    pub fn ancestral_boost_charisma(&self) -> u8 {
        self.ancestral_boost_charisma
    }

    pub fn ancestral_ability_boosts(&self) -> u8 {
        self.ancestral_ability_boosts
    }

    pub fn ancestral_bonus_feats(&self) -> u8 {
        self.ancestral_bonus_feats
    }

    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn subtype(&self) -> &str {
        self.subtype.as_ref()
    }

    pub fn valid_languages(&self) -> &[String] {
        self.valid_languages.as_ref()
    }

    pub fn languages(&self) -> &[String] {
        self.languages.as_ref()
    }

    pub fn immunities(&self) -> &[String] {
        self.immunities.as_ref()
    }

    pub fn ancestral_boost_language(&self) -> u8 {
        self.ancestral_boost_language
    }

    pub fn vision_type(&self) -> &str {
        self.vision_type.as_ref()
    }
}

pub enum Size {
    Small,
    Medium,
    Large,
}

