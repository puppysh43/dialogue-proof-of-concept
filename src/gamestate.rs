use crate::dialogue::*;
use hecs::*;
pub struct GameState {
    pub world: World,
    pub dialogue_db: DialogueDB,
    pub current_dialogue_tree: Option<DialogueTree>,
    pub current_dialogue_node: Option<DialogueNode>,
    ///used to flag when the player has decided to quit the game
    pub quitting: bool,
}
