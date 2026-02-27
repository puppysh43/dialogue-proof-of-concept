///all player information needed for proof of concept and basic testing of functionality. maybe will make configurable via config file.
use std::collections::HashMap;
#[derive(Clone, Debug)]
pub struct Player {
    attributes: Attributes,
    skills: Skills,
    inventory: HashMap<String, Item>,
}

#[derive(Clone, Debug)]
pub enum AttributeType {
    Strength,
    Dexterity,
    Endurance,
    Intellect,
    Education,
    Charisma,
}

///contains the attributes of a character. each attribute is a tuple, with the first entry being the max/default set at character creation, and the second being the current
///allowing for attribute damage as well as temporary boosts from items or consumables.
#[derive(Clone, Debug)]
pub struct Attributes {
    strength: (i32, i32),
    dexterity: (i32, i32),
    endurance: (i32, i32),
    intellect: (i32, i32),
    education: (i32, i32),
    charisma: (i32, i32),
}
impl Attributes {
    pub fn get_current(&self, attribute_type: AttributeType) -> i32 {
        match attribute_type {
            AttributeType::Strength => self.strength.1,
            AttributeType::Dexterity => self.dexterity.1,
            AttributeType::Endurance => self.endurance.1,
            AttributeType::Intellect => self.intellect.1,
            AttributeType::Education => self.education.1,
            AttributeType::Charisma => self.charisma.1,
        }
    }
    pub fn get_max(&self, attribute_type: AttributeType) -> i32 {
        match attribute_type {
            AttributeType::Strength => self.strength.0,
            AttributeType::Dexterity => self.dexterity.0,
            AttributeType::Endurance => self.endurance.0,
            AttributeType::Intellect => self.intellect.0,
            AttributeType::Education => self.education.0,
            AttributeType::Charisma => self.charisma.0,
        }
    }
    ///gets the bonus a given attribute can contribute to a task check
    pub fn get_bonus(&self, attribute_type: AttributeType) -> i32 {
        let val = self.get_current(attribute_type);
        match val {
            0 => -3,
            1..=2 => -2,
            3..=5 => -1,
            6..=8 => 0,
            9..=11 => 1,
            12..=14 => 2,
            15.. => 3,
            _ => 0,
        }
    }
    pub fn full_heal(&mut self) {
        (
            self.strength.1,
            self.dexterity.1,
            self.endurance.1,
            self.intellect.1,
            self.education.1,
            self.charisma.1,
        ) = (
            self.strength.0,
            self.dexterity.0,
            self.endurance.0,
            self.intellect.0,
            self.education.0,
            self.charisma.0,
        );
    }
}
/*
impl Bonus for Attributes {
    fn get_bonus<T>(&self, specifier: T) -> i32 {}
}
pub trait Bonus {
    fn get_bonus(&self, specifier: AttributeType || SkillType) -> i32;
}*/

#[derive(Clone, Debug)]
pub struct Skills {
    skills: HashMap<SkillType, i32>,
}
impl Skills {
    pub fn new() -> Self {
        let skills: HashMap<SkillType, i32> = HashMap::new();
        Skills { skills }
    }
    pub fn add_skill(&mut self, skilltype: SkillType, level: i32) {
        self.skills.insert(skilltype, level);
    }
    pub fn get_bonus(&self, skilltype: SkillType) -> i32 {
        let query = self.skills.get(&skilltype);
        if query.is_some() {
            return query.unwrap().clone();
        }
        -3
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum SkillType {
    Speech,
    Stealth,
    Medicine,
    Tech,
}

#[derive(Clone, Debug)]
pub struct Item {
    name: String,
}
impl Item {
    pub fn new(name: String) -> Self {
        Item { name }
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
}
