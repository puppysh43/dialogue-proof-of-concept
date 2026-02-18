///all player information needed for proof of concept and basic testing of functionality. maybe will make configurable via config file.
pub struct Player {
    attributes: Attributes,
    skills: Vec<Skill>,
    inventory: HashMap<Item>,
}

pub struct Attributes {}
