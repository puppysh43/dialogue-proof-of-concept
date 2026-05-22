use crate::gamestate::GameState;
struct AppState {
    appmode: AppMode,
    gamestate: GameState,
    editorstate: EditorState,
    quitting: bool,
}
enum AppMode {
    Root,
    Editor,
    Game,
}
