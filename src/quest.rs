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
    pub fn get_from_path(&self, path: (String, String)) -> QuestNode {
        self.db.get(&path.0).unwrap().get(path.1).clone()
    }
}
//

#[derive(Clone, Debug)]
pub struct QuestTree {
    ///quest name
    name: String,
    tree: HashMap<String, QuestNode>,
}
impl QuestTree {
    ///make new quest tree to be filled
    pub fn new(name: String) -> Self {
        let tree: HashMap<String, QuestNode> = HashMap::new();
        QuestTree { name, tree }
    }
    ///add a new quest node to the tree
    pub fn add(&mut self, node_name: String, new_node: QuestNode) {
        self.tree.insert(node_name, new_node);
    }
    ///clones out a copy of the requested dialogue node
    pub fn get(&self, node_name: String) -> QuestNode {
        self.tree.get(&node_name).unwrap().clone()
    }
    ///gets the first node of the quest tree by checking for one without a parent node. should always return something
    pub fn first_node(&self) -> Option<QuestNode> {
        let mut first_node: Option<QuestNode> = None;
        for (_, node) in self.tree.iter() {
            if node.parent_nodes.is_empty() {
                first_node = Some(node.clone());
            }
        }
        first_node
    }
    ///get quest name
    pub fn name(&self) -> String {
        self.name.clone()
    }
}

#[derive(Clone, Debug)]
pub struct QuestNode {
    completed: bool,
    parent_nodes: Vec<String>,
    child_nodes: Vec<String>,
}
impl QuestNode {
    ///instantiates a new questnode. quest nodes always start out as false b/c they all need to be started and advanced by player actions.
    pub fn new(parent_nodes: Vec<String>, child_nodes: Vec<String>) -> Self {
        QuestNode {
            completed: false,
            parent_nodes,
            child_nodes,
        }
    }
    ///clone out list of parent nodes. if the vec of parent nodes is empty it means it's the first node in the quest - almost always the one used to mark the quest as accepted/initiated
    pub fn get_parent_nodes(&self) -> Vec<String> {
        self.parent_nodes.clone()
    }
    ///clone out the list of child nodes, if the vec of child nodes is empty it means it's a terminating node in the quest and marks its completion.
    pub fn get_child_nodes(&self) -> Vec<String> {
        self.child_nodes.clone()
    }
    ///marks a quest node as completed
    pub fn complete(&mut self) {
        self.completed = true;
    }
}
