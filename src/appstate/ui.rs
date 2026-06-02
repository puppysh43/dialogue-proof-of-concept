use crate::appstate::*;
impl eframe::App for AppState {
    //first render the ui for the editor layer using the editor state data. this will always be rendered
    //and is the "base window" of the application, the same way the editor of an IDE is always open
    //even when you've launched/compiled the code you're working on.
    //then depending on the appmode there will be either a popup for the character creator
    //or parsing through the "game" itself
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::top("menu bar panel").show_inside(ui, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New Bundle").clicked() {
                        self.menu_modal = MenuModal::New;
                    }
                    if ui.button("Load Bundle").clicked() {
                        self.menu_modal = MenuModal::Load;
                    }
                    if ui.button("Rename Bundle").clicked() {
                        self.menu_modal = MenuModal::Rename;
                    }
                    if ui.button("Save Bundle").clicked() {
                        self.menu_modal = MenuModal::Save;
                    }
                    if ui.button("Save Bundle As").clicked() {
                        self.menu_modal = MenuModal::SaveAs;
                    }
                    if ui.button("Play Bundle").clicked() {
                        self.menu_modal = MenuModal::CharacterCreation;
                    }
                    if ui.button("Quit").clicked() {
                        self.menu_modal = MenuModal::Quitting;
                        // ui.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
            });
        });
        //this leftmost panel is going to have the actual dialogue or quest trees represented as a nested series of collapsable headers similar
        //to the "tree" example in the web demo. I will need to find a way to like recursively go through the quest and dialogue db to determine what
        //needs to be displayed, always appending an "add node" button at the bottom of every single layer
        egui::Panel::left("side panel").show_inside(ui, |ui| {
            //run the tree ui
            self.tree.ui(ui, self.editorstate.filename());
        });
        //this is where the actual editing of the various data files will be
        egui::CentralPanel::default().show_inside(ui, |ui| {
            //
        });
        //render the modal windows if necessary
        //first the ones opened by the menu bar
        match self.menu_modal {
            MenuModal::None => {
                //do nothing
            }
            MenuModal::Rename => {
                Modal::new(egui::Id::new("RenameBundle")).show(ui.ctx(), |ui| {
                    //put a textbox that updates the editorstate filename
                    ui.set_width(250.0);
                    ui.heading("Rename Bundle");
                    ui.label("Name Databundle:");
                    ui.text_edit_singleline(self.editorstate.mut_filename());
                    //maybe switch over to like a buffer that gets sent to the editorstate once the
                    // ui.button("Rename")
                    if ui.button("Close").clicked() {
                        self.menu_modal = MenuModal::None;
                        ui.close();
                    }
                });
            }
            MenuModal::New => {
                //
            }
            MenuModal::Load => {
                //
            }
            MenuModal::Save => {
                //
            }
            MenuModal::SaveAs => {
                //
            }
            MenuModal::CharacterCreation => {
                //
            }
            MenuModal::Play => {
                //
            }
            MenuModal::Quitting => {
                //
            }
        }
    }
}
