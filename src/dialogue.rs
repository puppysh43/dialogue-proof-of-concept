use crate::gamestate::*;
use hecs::*;
use std::collections::HashMap;

/*
CURRENT NOTES AND THOUGHTS
 - uses a lot of unnecessary cloning for fast prototyping should optimize later
*/

///parent struct that contains all dialogue trees within the game. dialogue trees will be cloned out of the hashmap as needed
#[derive(Clone, Debug)]
pub struct DialogueDB {
    db: HashMap<String, DialogueTree>,
}
//methods for the dialogue database
impl DialogueDB {
    ///create appropriately typed empty dialogue database for use in the gamestate struct
    pub fn new() -> Self {
        let db: HashMap<String, DialogueTree> = HashMap::new();
        DialogueDB { db }
    }
    ///add dialogue tree to dialogue database
    pub fn add(&mut self, tree_name: String, new_tree: DialogueTree) {
        self.db.insert(tree_name, new_tree);
    }
    ///clones out copy of the requested dialogue tree
    pub fn get(&self, tree_name: String) -> DialogueTree {
        self.db.get(&tree_name).unwrap().clone()
    }
    ///access a copy of the dialogue database
    pub fn db(&self) -> HashMap<String, DialogueTree> {
        self.db.clone()
    }

    pub fn db_list(&self) -> Vec<String> {
        let mut list = Vec::new();
        for key in self.db.keys() {
            list.push(key.clone());
        }
        list.clone()
    }
}
///intermediate struct that contains the full dialogue tree for an NPC, interaction, or prompt
#[derive(Clone, Debug)]
pub struct DialogueTree {
    ///the name of the NPC the dialogue tree "belongs to", will be displayed in the UI
    name: String,
    ///the container for the dialogue tree
    tree: HashMap<String, DialogueNode>,
}
impl DialogueTree {
    ///creates a new empty dialogue tree
    pub fn new(name: String) -> Self {
        let tree: HashMap<String, DialogueNode> = HashMap::new();
        DialogueTree { name, tree }
    }
    ///adds a new dialogue node to the tree
    pub fn add(&mut self, node_name: String, new_node: DialogueNode) {
        self.tree.insert(node_name, new_node);
    }
    ///clones out a copy of the requested dialogue node
    pub fn get(&self, node_name: String) -> DialogueNode {
        self.tree.get(&node_name).unwrap().clone()
    }
    ///gets the first node of the tree by checking for one without a parent node. should always return some.
    pub fn first_node(&self) -> Option<DialogueNode> {
        let mut first_node: Option<DialogueNode> = None;
        for (_, node) in self.tree.iter() {
            if node.parent_node.is_empty() {
                first_node = Some(node.clone());
            }
        }
        first_node
    }
    ///gets the name of the NPC in the dialogue tree
    pub fn name(&self) -> String {
        self.name.clone()
    }
}
///function pointer for any function that references the gamestate for any kind of condition and returns a boolean operator
pub type VisibilityCheck = fn(&GameState) -> bool;

///individual node that makes up dialogue trees. contains the previous dialogue option, the possible dialogue options
///the text printed to the screen, as well as gameplay features; a requirement for visibility, a worldstate check
///(such as a skillcheck, checking quest flags, or checking for an item in the player's inventory), and a worldstate change
///(editing the inventory of a character, changing a questflag, dealing damage, etc)
#[derive(Debug, Clone)]
pub struct DialogueNode {
    ///what the player is saying, this is used when displaying the childnode options.
    player_text: String,
    ///what the npc is saying, this will display when the node has been chosen
    npc_text: String,
    ///the node that preceded this one. if empty it means it's the initial node
    parent_node: Vec<String>,
    ///the child nodes aka what dialogue options are available when this one is selected. if empty it's a node that terminates the dialogue
    child_nodes: Vec<String>,
    ///an optional requirement that must be met for the option to be visible. this is to prevent for example the PC from
    ///seeing an option to say they're going to return a quest item when it is not in their inventory, or from seeing
    ///dialogue related to a quest they haven't progressed to yet.
    visibility_req: Option<VisibilityCheck>,
    ///this is a generic container for dialogues that rely on changing the worldstate. for example, a simple speech check would
    ///use this field to run a skill check against the player's speech skill. depending on success or failure a different quest
    ///flag would be marked, which would then is parsed by visibility_req to ensure the PC sees the appropriate response
    worldstate_changes: Option<fn(&mut GameState)>,
}
impl DialogueNode {
    ///Creates a new dialogue node
    pub fn new(
        player_text: String,
        npc_text: String,
        parent_node: Vec<String>,
        child_nodes: Vec<String>,
        visibility_req: Option<VisibilityCheck>,
        worldstate_changes: Option<fn(&mut GameState)>,
    ) -> Self {
        DialogueNode {
            player_text,
            npc_text,
            parent_node,
            child_nodes,
            visibility_req,
            worldstate_changes,
        }
    }
    ///clones out the player text
    pub fn player_text(&self) -> String {
        self.player_text.clone()
    }
    ///clones out the npc text
    pub fn npc_text(&self) -> String {
        self.npc_text.clone()
    }
    ///clones out an option that will contain the parent node if it has one, aka the preceeding dialogue option
    pub fn parent_node(&self) -> Vec<String> {
        self.parent_node.clone()
    }
    ///clones out the list of the current node's child nodes aka proceeding dialogue options
    pub fn child_nodes(&self) -> Vec<String> {
        self.child_nodes.clone()
    }
    ///clones out an option that will contain the visibility check if it has one so that it can be run on the gamestate
    ///and a boolean produced when the game needs to decide what dialogue options to show
    pub fn visibility_req(&self) -> Option<VisibilityCheck> {
        self.visibility_req.clone()
    }
    ///clones out an option containing the gamestate changes if present so that they can be run on the gamestate
    pub fn gamestate_changes(&self) -> Option<fn(&mut GameState)> {
        self.worldstate_changes.clone()
    }
}
