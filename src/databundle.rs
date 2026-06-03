use crate::dialogue::DialogueDB;
use crate::quest::QuestDB;
use serde::{Deserialize, Serialize};
//databundle, this is a generic datatype that ferries necessary information between various parts of the program
#[derive(Serialize, Deserialize)]
pub struct Databundle {
    dialogue_db: DialogueDB,
    name: String,
    quest_db: QuestDB,
}
impl Databundle {
    pub fn new() -> Self {
        Databundle {
            dialogue_db: DialogueDB::new(),
            name: String::new(),
            quest_db: QuestDB::new(),
        }
    }
    pub fn from(name: &str, dialogue_db: DialogueDB, quest_db: QuestDB) -> Self {
        Databundle {
            dialogue_db,
            name: name.to_string(),
            quest_db,
        }
    }
    pub fn export(&self) -> (String, DialogueDB, QuestDB) {
        (
            self.name.clone(),
            self.dialogue_db.clone(),
            self.quest_db.clone(),
        )
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
}
