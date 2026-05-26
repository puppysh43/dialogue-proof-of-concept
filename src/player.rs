///all player information needed for proof of concept and basic testing of functionality. maybe will make configurable via config file.
use std::collections::HashMap;
#[derive(Clone, Debug)]
pub struct Player {
    attributes: Attributes,
    skills: Skills,
    inventory: HashMap<String, Item>,
}
impl Default for Player {
    fn default() -> Self {
        let inventory: HashMap<String, Item> = HashMap::new();
        Player {
            attributes: Attributes::new(8, 8, 8, 8, 8, 8),
            skills: Skills::new(),
            inventory,
        }
    }
}
impl Player {
    ///generates a new player, used on game initialization.
    pub fn new(attributes: Attributes, skills: Skills, inventory: HashMap<String, Item>) -> Player {
        Player {
            attributes,
            skills,
            inventory,
        }
    }
    ///clone out the player's attributes for use in game systems
    pub fn attributes(&self) -> Attributes {
        self.attributes.clone()
    }
    ///clone out the player's skills for use in game systems
    pub fn skills(&self) -> Skills {
        self.skills.clone()
    }
    ///clone out the player's inventory for use in game systems
    pub fn inventory(&self) -> HashMap<String, Item> {
        self.inventory.clone()
    }
    ///add an item to the player's inventory.
    pub fn add_item(&mut self, id: String, item: Item) {
        self.inventory.insert(id, item);
    }
    pub fn remove_item(&mut self, id: String) {
        self.inventory.remove(&id);
    }
}

///enum used for specifying the attribute in various functions.
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
    ///create a new set of attributes for a character given the maximum values for the attributes, which then get copied into the current
    ///values as well
    pub fn new(
        strength: i32,
        dexterity: i32,
        endurance: i32,
        intellect: i32,
        education: i32,
        charisma: i32,
    ) -> Attributes {
        Attributes {
            strength: (strength, strength),
            dexterity: (dexterity, dexterity),
            endurance: (endurance, endurance),
            intellect: (intellect, intellect),
            education: (education, education),
            charisma: (charisma, charisma),
        }
    }
    ///given an attribute return the current value
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
    ///given an attribute return the maximum value defined at character creation
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
    ///fully heal all attributes to their maximum value defined at character creation
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

///container for character skills. Unlike attributes the character skill list is NOT exhaustive - a lack of a level in the skill
///represents a complete lack of familiarity with the skill and imposes a -3 penalty on relevant checks. as such, skill code
///will need to accomodate the possibility of a None result when querying for an arbitary skill level
#[derive(Clone, Debug)]
pub struct Skills {
    skills: HashMap<SkillType, i32>,
}
impl Skills {
    ///creates an empty set of skills.
    pub fn new() -> Skills {
        let skills: HashMap<SkillType, i32> = HashMap::new();
        Skills { skills }
    }
    ///generates a set of skills when passed a vector of tuples
    pub fn from_skills(starting_skills: Vec<(SkillType, i32)>) -> Skills {
        let mut skills: HashMap<SkillType, i32> = HashMap::new();
        for (skill, level) in starting_skills.iter() {
            skills.insert(*skill, *level);
        }
        Skills { skills }
    }
    ///adds a skill to the character or overwrites the skill to adjust the level. use varies depending on how much character
    ///progression is desired in your implementation of the RPG system
    pub fn add_skill(&mut self, skilltype: SkillType, level: i32) {
        self.skills.insert(skilltype, level);
    }
    ///gets the skill bonus of a specified SkillType. if the character does not have the requested skill then they suffer a penalty
    ///of -3 on all relevant task checks
    pub fn get_bonus(&self, skilltype: SkillType) -> i32 {
        let query = self.skills.get(&skilltype);
        if query.is_some() {
            return query.unwrap().clone();
        }
        -3
    }
    ///gets the flat level of the skill for various purposes such as visibility requirements or checkpoint style task checks
    ///if the character doesn't have the specified skill then it'll return none.
    pub fn get_lvl(&self, skilltype: SkillType) -> Option<i32> {
        if self.skills.get(&skilltype).is_some() {
            Some(self.skills.get(&skilltype).unwrap().clone())
        } else {
            None
        }
    }
}

///enum for specifying which character skill is needed in various functions
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum SkillType {
    Speech,
    Stealth,
    Medicine,
    Tech,
    Athletics,
}

///very simple item implementation currently only used to test quest mcguffins i.e. keycards, fetch quests, etc.
#[derive(Clone, Debug)]
pub struct Item {
    name: String,
}
impl Item {
    ///generate a new item given a name. note that the name is only used for displaying the inventory
    ///when accessing an item in game systems the ID, aka the string key in the inventory hashmap, is used
    pub fn new(name: String) -> Self {
        Item { name }
    }
    ///clones out the name of the item so it can be displayed in a primitive inventory screen
    pub fn name(&self) -> String {
        self.name.clone()
    }
}
