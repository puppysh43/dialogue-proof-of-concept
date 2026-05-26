use crate::databundle::Databundle;
use crate::dialogue::*;
use crate::player::*;
use crate::quest::*;
pub struct GameState {
    pub dialogue_db: DialogueDB,
    pub current_dialogue_node: Option<(DialoguePath, DialogueNode)>,
    pub quest_db: QuestDB,
    ///contains all the player stats and information
    pub player: Player,
    ///used to flag when the player has decided to quit the game
    pub quitting: bool,
}
impl GameState {
    ///generates a new completely empty GameState used for resetting the game or initializing it on program startup
    pub fn new() -> Self {
        GameState {
            dialogue_db: DialogueDB::new(),
            current_dialogue_node: None,
            quest_db: QuestDB::new(),
            player: Player::default(),
            quitting: false,
        }
    }
    /* don't think I'll need this function tbh
    pub fn export_databundle(&self) -> Databundle {
        Databundle::from(
            String::new(),
            self.dialogue_db.clone(),
            self.quest_db.clone(),
        )
    }*/
    ///used to initialize the game from the databundle passed directly from the editor or loaded from file,
    ///as well as with the data entered in the character creation stage
    pub fn init(&mut self, databundle: Databundle, player: Player) {
        (_, self.dialogue_db, self.quest_db) = databundle.export();
        self.player = player;
    }
}
