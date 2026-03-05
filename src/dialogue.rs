use crate::gamestate::*;
use crate::player::*;
use std::collections::HashMap;

/*
CURRENT NOTES AND THOUGHTS
 - uses a lot of unnecessary cloning for fast prototyping should optimize later
 - maybe make it so various functions take in string literals and turn them into strings so that all values passed in don't need to be appended with .to_string()
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
///type used to specify a quest node with the tree name and node name
#[derive(Clone, Debug)]
pub struct QuestNodePath {
    ///specifies the name of the quest tree needed
    tree: String,
    ///specifies the name of the quest node needed in the specified tree
    node: String,
}
impl QuestNodePath {
    ///creates a new
    pub fn new(tree: String, node: String) -> QuestNodePath {
        QuestNodePath { tree, node }
    }
    pub fn path(&self) -> (String, String) {
        (self.tree.clone(), self.node.clone())
    }
    pub fn tree(&self) -> String {
        self.tree.clone()
    }
    pub fn node(&self) -> String {
        self.node.clone()
    }
}

#[derive(Clone, Debug)]
///flag that determines if a given dialogue node can be shown.
pub enum VisibilityConditions {
    ///marks that a dialogue option can only be visible if the specified quest node is marked as completed
    QuestStage(QuestNodePath),
    ///marks that a dialogue option can only be visible if the player has a minimum level in the specified skill
    SkillMinumum(SkillType, i32),
    ///marks that a dialogue option can only be visible if the player has a minimum level in the specified attribute
    AttributeMinimum(AttributeType, i32),
}
///used for choice and consequence reactivity in the RPG sense. the choice is the test that will be performed
///ex. (checking a quest stage, doing a skill check, checking for an item) and the consequences are event flags for worldstate changes
///to do based on the result of the choice
#[derive(Clone, Debug)]
pub struct CheckAndConsequences {
    ///enum flag that tells the dialogue processing system what kind of checks need to be completed ex. skill check, checking a quest stage, faction reputation, etc
    check: CheckType,
    ///container for a vec of consequences for both success and failure (ex. when making a strength check to move a boulder, success clears the boulder and changes the associated quest
    ///so the play can pass through, on failure the character takes damage)
    consequences: Consequences,
}
impl CheckAndConsequences {
    pub fn new(check: CheckType, consequences: Consequences) -> CheckAndConsequences {
        CheckAndConsequences {
            check,
            consequences,
        }
    }
    pub fn check(&self) -> CheckType {
        self.check.clone()
    }
    pub fn consequences(&self) -> Consequences {
        self.consequences.clone()
    }
}
///the difficulty of a task check, translates into a target number that the character needs to roll at or over to succeed
#[derive(Clone, Debug)]
pub enum CheckDifficulty {
    ///target number of 2 or more
    Simple,
    ///target number of 4 or more
    Easy,
    ///target number of 6 or more
    Rotuine,
    ///target number of 8 or more
    Average,
    ///target number of 10 or more
    Difficult,
    ///target number of 12 or more
    VeryDifficult,
    ///target number of 14 or more
    Formidable,
    ///target number of 16 or more
    Impossible,
}

#[derive(Clone, Debug)]
pub enum CheckResult {
    Success,
    Failure,
}
///specifications used to tell what kind of skillcheck it is - what skill and/or attribute is being tested, and if item bonuses can be used. If no skilltype or attributetype is specified for the check
///it will do an agnostic skillcheck, essentially a random diceroll of average difficulty
///defined at creation and completely immutable after, will copy or clone out various fields as needed for the dialogue parsing and skillcheck system
#[derive(Clone, Debug)]
pub struct TaskCheckSpecifications {
    ///if the test has a checkpoint option or not. most checks will have a bonus level at which no dice roll needs to be made (ex. if athletics lvl = 1 check automatically succeeds)
    ///if the checkpoint is NOT reached then the player can still attempt the check as a diceroll. Both attributes and skills can contribute to reaching the checkpoint
    checkpoint: Option<i32>,
    ///specifies what skilltype is able to lend a bonus to the task check, if any
    skill_bonus: Option<SkillType>,
    ///specified what attribute type is able to lend a bonus to the task check, if any
    attribute_bonus: Option<AttributeType>,
    ///flags if item bonuses are applicable to this check ex. a mechanics toolkit that adds +1 to all repair checks
    item_bonus: bool,
    ///custom check difficulty. if one isn't given/field = none then it will do an average difficulty check
    difficulty: Option<CheckDifficulty>,
}
impl TaskCheckSpecifications {
    ///creates a new TaskCheckSpecifications for defining a task check in dialogues.
    pub fn new(
        checkpoint: Option<i32>,
        skill_bonus: Option<SkillType>,
        attribute_bonus: Option<AttributeType>,
        item_bonus: bool,
        difficulty: Option<CheckDifficulty>,
    ) -> TaskCheckSpecifications {
        TaskCheckSpecifications {
            checkpoint,
            skill_bonus,
            attribute_bonus,
            item_bonus,
            difficulty,
        }
    }
    pub fn checkpoint(&self) -> Option<i32> {
        self.checkpoint
    }
    pub fn skill_bonus(&self) -> Option<SkillType> {
        self.skill_bonus.clone()
    }
    pub fn attribute_bonus(&self) -> Option<AttributeType> {
        self.attribute_bonus.clone()
    }
    pub fn item_bonus(&self) -> bool {
        self.item_bonus
    }
    pub fn difficulty(&self) -> Option<CheckDifficulty> {
        self.difficulty.clone()
    }
}
///Flag for what kind of gamestate check is going to be performed by the dialogue node.
#[derive(Clone, Debug)]
pub enum CheckType {
    ///a task check, more commonly known in other games as a skill check. For more details on what exactly this entails see the documentation for TaskCheckSpecifications.
    TaskCheck(TaskCheckSpecifications),
    ///checks if a player has an item in their inventory.
    ItemCheck(String),
    ///checks if a quest stage has been completed given the path of a quest stage (ex. the tree name and node name)
    QuestStageCheck(QuestNodePath),
}
///container for a vec of consequences for both success and failure (ex. when making a strength check to move a boulder, success clears the boulder and changes the associated quest
///so the play can pass through, on failure the character takes damage) Also contains the ID of the node in the quest tree that a success or failure will send you to.
#[derive(Clone, Debug)]
pub struct Consequences {
    ///contains a vector of variable size of consequences (gamestate changes) to be processed when the player succeeds the check as well as the ID of the dialogue node to route to depending on results
    success: (Vec<Consequence>, String),
    ///contains a vector of variable size of consequences (gamestate changes) to be processed when the player fails the check as well as the ID of the dialogue node to route to depending on results
    failure: (Vec<Consequence>, String),
}
impl Consequences {
    ///creates a new set of consequences for a task check
    pub fn new(
        success: (Vec<Consequence>, String),
        failure: (Vec<Consequence>, String),
    ) -> Consequences {
        Consequences { success, failure }
    }
    ///provides the relevant consequence flags and ID for the proceeding dialogue node if the player succeeds the task check
    pub fn success(&self) -> (Vec<Consequence>, String) {
        self.success.clone()
    }
    ///provides the relevant consequence flags and ID for the proceeding dialogue node if the player fails the task check
    pub fn failure(&self) -> (Vec<Consequence>, String) {
        self.failure.clone()
    }
}
///event flag to tell the game what gamestate changes to make based on the result of the choice/check.
#[derive(Clone, Debug)]
pub enum Consequence {
    ///damage the player by the amount specified
    DamagePlayer(i32),
    //put an item in the player's inventory given the name/ID of the item as a string (currently not functional in the prototype due to how items are handled)
    // GivePlayerItem(String),
    ///give the player an item. includes the ID to use as the key in the hashmap as well as the Item struct
    GivePlayerItem(String, Item),
    ///remove an item from the player's inventory given the ItemID (string) specified in the flag
    RemoveItem(String),
    ///flag with a path to a quest node that the dialogue processing system will mark as completed
    CompleteQuestStage(QuestNodePath),
    //used later/example of future possibilites, will raise the faction reputation by a specified amount according to the FactionID (string for now)
    // RaiseReputation(String, i32),
    ///flag for any complicated or unique consequences of dialogue. the string will need to be hard coded to a function in the custom flag parsing system.
    Custom(String),
}

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
    visibility_req: Option<VisibilityConditions>,
    ///the "choice and consequences field" aka the list of various flags needed to parse out a choice/check
    cnc: Option<CheckAndConsequences>,
}
impl DialogueNode {
    ///Creates a new dialogue node
    pub fn new(
        player_text: String,
        npc_text: String,
        parent_node: Vec<String>,
        child_nodes: Vec<String>,
        visibility_req: Option<VisibilityConditions>,
        cnc: Option<CheckAndConsequences>,
    ) -> Self {
        DialogueNode {
            player_text,
            npc_text,
            parent_node,
            child_nodes,
            visibility_req,
            cnc,
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
    pub fn visibility_req(&self) -> Option<VisibilityConditions> {
        self.visibility_req.clone()
    }
    ///clones out an option containing the gamestate changes if present so that they can be run on the gamestate
    pub fn checks_and_consequences(&self) -> Option<CheckAndConsequences> {
        self.cnc.clone()
    }
}
