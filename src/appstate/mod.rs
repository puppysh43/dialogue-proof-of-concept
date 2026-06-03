use crate::editorstate::EditorState;
use crate::gamestate::GameState;
use crate::player::Player;
use egui::*;
mod tree;
use crate::appstate::tree::Tree;
mod ui;
pub struct AppState {
    ///holds the data needed to edit the databundle
    editorstate: EditorState,
    ///data needed to play through the databundle as a "game"
    gamestate: GameState,
    ///buffer that holds the player data as it is entered during "character creation" before a "game" is launched
    player_buffer: Option<Player>,
    //
    tree: Tree,
    menu_modal: MenuModal,
    ///multi use string buffer to hold text entered by the user before sending it to the appropriate part of the program.
    string_buffer: String,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            editorstate: EditorState::new(),
            gamestate: GameState::new(),
            player_buffer: None,
            tree: Tree::new(),
            menu_modal: MenuModal::None,
            string_buffer: String::new(),
        }
    }
}
//tracks which modal windows have been opened by the menu bar
//these can all be tracked in one enum because they will never be open at the same time!
enum MenuModal {
    ///flags that there is no modal window open from the menu bar
    None,
    ///flags that the Rename Bundle modal menu has been activated
    Rename,
    New,
    Load,
    Save,
    SaveAs,
    Play,
    CharacterCreation,
    Quitting,
}
