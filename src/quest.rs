/*
quest tree database structs
*/
use std::collections::HashMap;
///highest level "container" of all quest trees in the game
#[derive(Clone, Debug)]
pub struct QuestDB {
    db: HashMap<String, QuestTree>,
}
impl QuestDB {
    pub fn new() -> Self {
        let db: HashMap<String, QuestTree> = HashMap::new();
        QuestDB { db }
    }

    pub fn add(&mut self, tree_name: String, new_tree: QuestTree) {
        self.db.insert(tree_name, new_tree);
    }

    pub fn get(&self, tree_name: String) -> QuestTree {
        self.db.get(&tree_name).unwrap().clone()
    }
}
//

#[derive(Clone, Debug)]
pub struct QuestTree {
    nodes: HashMap<String, QuestNode>,
}

#[derive(Clone, Debug)]
pub struct QuestNode {
    completed: bool,
    parent_nodes: Vec<String>,
    child_nodes: Vec<String>,
}
