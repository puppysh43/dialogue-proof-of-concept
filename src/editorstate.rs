use crate::{
    databundle::Databundle,
    dialogue::{DialogueDB, DialogueNode, DialoguePath, QuestNodePath},
    quest::{QuestDB, QuestNode},
};

//will hold the state information needed by the editor
pub struct EditorState {
    dialogue_buffer: DialogueBuffer,
    dialogue_db: DialogueDB,
    filename: String,
    quest_buffer: QuestBuffer,
    quest_db: QuestDB,
}
impl EditorState {
    ///Generates a new, empty editor state. Used for resets or when starting the program.
    pub fn new() -> Self {
        EditorState {
            dialogue_buffer: DialogueBuffer::new(),
            dialogue_db: DialogueDB::new(),
            quest_buffer: QuestBuffer::new(),
            filename: String::new(),
            quest_db: QuestDB::new(),
        }
    }
    pub fn export_databundle(&self) -> Databundle {
        Databundle::from(
            &self.filename,
            self.dialogue_db.clone(),
            self.quest_db.clone(),
        )
    }
}
pub struct DialogueBuffer {
    node: DialogueNode,
    path: DialoguePath,
}
impl DialogueBuffer {
    pub fn new() -> Self {
        DialogueBuffer {
            node: DialogueNode::new(),
            path: DialoguePath::new(),
        }
    }
}
pub struct QuestBuffer {
    node: QuestNode,
    path: QuestNodePath,
}
impl QuestBuffer {
    pub fn new() -> Self {
        QuestBuffer {
            node: QuestNode::new(),
            path: QuestNodePath::new(),
        }
    }
}
