
use std::collections::HashMap;
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum AttributeType {
    Strength,
    Dexterity,
    Endurance,
    Intelligence,
    Education,
    Charm,
}

///component that stores the attributes of an npc inspired by mongoose traveller 2e characteristics
pub struct Attributes {
    ///a character's natural physical strength
    strength: AttributeValue,
    ///a character's agility, reflexes, coordination, and fine motor control
    dexterity: AttributeValue,
    ///a character's physical stamina, determination, and ability to sustain damage
    endurance: AttributeValue,
    ///a character's raw intelligence and quickness of mind - used for new information
    ///and puzzle solving
    intelligence: AttributeValue,
    ///a character's level of lifetime learning and experience especially in academics/intellectual pursuits
    education: AttributeValue,
    ///a character's untrained charisma, social aptitude, and ability to relate to others
    charm: AttributeValue,
}
impl Attributes {
    pub fn new(
        strength: i32,
        dexterity: i32,
        endurance: i32,
        intelligence: i32,
        education: i32,
        charm: i32,
    ) -> Self {
        Self {
            strength: AttributeValue::new(strength),
            dexterity: AttributeValue::new(dexterity),
            endurance: AttributeValue::new(endurance),
            intelligence: AttributeValue::new(intelligence),
            education: AttributeValue::new(education),
            charm: AttributeValue::new(charm),
        }
    }
    pub fn default() -> Self {
        Self {
            strength: AttributeValue::new(7),
            dexterity: AttributeValue::new(7),
            endurance: AttributeValue::new(7),
            intelligence: AttributeValue::new(7),
            education: AttributeValue::new(7),
            charm: AttributeValue::new(7),
        }
    }
    ///provides an attribute given the attribute type enum to then have further data specified out of it
    pub fn attribute(&self, attribute: AttributeType) -> &AttributeValue {
        match attribute {
            AttributeType::Strength => &self.strength,
            AttributeType::Dexterity => &self.dexterity,
            AttributeType::Endurance => &self.endurance,
            AttributeType::Intelligence => &self.intelligence,
            AttributeType::Education => &self.education,
            AttributeType::Charm => &self.charm,
        }
    }
    ///provides mutable access to a specific attribute
    pub fn mut_attribute(&mut self, attribute: AttributeType) -> &AttributeValue {
        match attribute {
            AttributeType::Strength => &mut self.strength,
            AttributeType::Dexterity => &mut self.dexterity,
            AttributeType::Endurance => &mut self.endurance,
            AttributeType::Intelligence => &mut self.intelligence,
            AttributeType::Education => &mut self.education,
            AttributeType::Charm => &mut self.charm,
        }
    }
}
#[derive(Copy, Clone, Debug, PartialEq)]
///Holds all necessary data about the character's attributes
pub struct AttributeValue {
    current: i32,
    max: i32,
}
impl AttributeValue {
    pub fn new(value: i32) -> Self {
        Self {
            current: value,
            max: value,
        }
    }
    ///gets the current value of the given attribute. Used for almost all circumstances
    ///the game checks the attribute's value
    pub fn current(&self) -> i32 {
        self.current
    }
    ///gets the maximum value of the given attribute
    pub fn max(&self) -> i32 {
        self.max
    }
    ///Get the current bonus of the attribute used for almost all skill checks
    pub fn bonus(&self) -> i32 {
        match self.current {
            ..=0 => -3,
            1..=2 => -2,
            3..=5 => -1,
            6..=8 => 0,
            9..=11 => 1,
            12..=14 => 2,
            15.. => 3,
        }
    }
    ///Heal the attribute by a given delta, up to the maximum value of the attribute
    pub fn heal(&mut self, delta: i32) {
        if delta.is_positive() {
            if (self.current + delta) <= self.max {
                self.current += delta;
            } else {
                self.current = self.max;
            }
        }
    }
    ///Damage the attribute by a given delta
    pub fn damage(&mut self, delta: i32) {
        if delta.is_positive() {
            self.current -= delta;
        }
    }
}
