use crate::editorstate::EditorState;
use crate::gamestate::GameState;
use crate::player::Player;
///Tracks whether the player is in the editor (default), running through a dialogue
///with the "run dialogue" option in the gui, or making a character before running through a dialogue
///tree
enum AppMode {
    ///flags that the program is in the initial popup that appears before running through a selected dialogueDB
    ///where the user defines the stats of the player character for that run.
    CharacterCreation,
    ///flags that the program is in the default state of creating/editing a dialoguedb+questdb databundle
    Editor,
    ///flags that the program is currently running through a "game" dialoguedb+questdb databundle
    Game,
}
pub struct AppState {
    ///tracks what the program is doing
    pub appmode: AppMode,
    ///holds the data needed to edit the databundle
    editorstate: EditorState,
    ///data needed to play through the databundle as a "game"
    gamestate: GameState,
    ///buffer that holds the player data as it is entered during "character creation" before a "game" is launched
    player_buffer: Option<Player>,
}

impl AppState {
    //
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            appmode: AppMode::Editor,
            editorstate: EditorState::new(),
            gamestate: GameState::new(),
            player_buffer: None,
        }
    }
}

impl eframe::App for AppState {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        //render the menu bar with all the big functions in its own top panel
        egui::Panel::top("menu bar panel").show_inside(ui, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New Bundle").clicked() {
                        //this is where it would pop up a prompt confirming if the player wants to reset
                        //the data in the editor currently, then it would ask for a name for the new bundle
                        println!("New bundle button clicked!");
                    }
                    if ui.button("Open Bundle").clicked() {
                        //this would create a focus locked popup ("modal") prompting the user
                        //to select a saved bundle file from the saves folder
                        println!("Open bundle button clicked!");
                    }
                    if ui.button("Save Bundle").clicked() {
                        //if there is an existing save file with the name of the editor state filename
                        //you can just save and overwrite without checking, if there isn't an existing save file
                        //with a name that matches the current filename then you can create a new file and maybe prompt the user
                        //for a filename or something idk
                        println!("Save bundle button clicked!");
                    }
                    if ui.button("Save Bundle As").clicked() {
                        //prompt the user for a different filename to save an alternate copy
                        println!("Save bundle as button clicked!");
                    }
                    if ui.button("Quit").clicked() {
                        ui.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.menu_button("Play", |ui| {
                    if ui.button("Play Current Bundle").clicked() {
                        //this will send a databundle from the editorstate directly to the gamestate and launch a popup window
                        //for doing character creation before then popping up the window for displaying the game
                    }
                    if ui.button("Play Bundle From File").clicked() {
                        //make a popup to select from the save folder a bundle the user wants to play, then start the
                        //character creation modal window -> gameplay modal window sequence.
                    }
                });
            });
        });
        //this leftmost panel is going to have the actual dialogue or quest trees represented as a nested series of collapsable headers similar
        //to the "tree" example in the web demo. I will need to find a way to like recursively go through the quest and dialogue db to determine what
        //needs to be displayed, always appending an "add node" button at the bottom of every single layer
        egui::Panel::left("side panel").show_inside(ui, |ui| {
            //
        });
        egui::CentralPanel::default().show_inside(ui, |ui| {
            //
        });
        //first render the ui for the editor layer using the editor state data. this will always be rendered
        //and is the "base window" of the application, the same way the editor of an IDE is always open
        //even when you've launched/compiled the code you're working on.
        //then depending on the appmode there will be either a popup for the character creator
        //or parsing through the "game" itself
    }
}
