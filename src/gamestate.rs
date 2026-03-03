use crate::dialogue::*;
use crate::player::*;
use crate::quest::*;
use hecs::*;
pub struct GameState {
    pub world: World,
    pub dialogue_db: DialogueDB,
    pub current_dialogue_tree: Option<DialogueTree>,
    pub current_dialogue_node: Option<DialogueNode>,
    pub quest_db: QuestDB,
    pub player: Player,
    ///used to flag when the player has decided to quit the game
    pub quitting: bool,
}
