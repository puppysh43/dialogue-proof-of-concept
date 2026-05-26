/*
SCOPE OF PROJECT
No ECS, no data for NPCs outside of associated dialogue and quests selected from the menu.
create 1 simple quest one intermediate quest and one more complicated quest
find way for quest and dialogue data to be loaded from a file instead of being hardcoded as much as possible
additionally look into making extremely basic proof of concept tooling for creating at LEAST quest db
player will have inventory for quest items, attributes, and skills, inspired by but pared down from MgT2e
obviously logic for interfacing with these things will change when added to an actual game especially b/c an ECS is probably
*/
/*
EGUI ADDITION
create a gui for both the gameplay interface and the editor
will need a minimum window size for both to make sure elements get placed properly
camera needed for properly placing objects
maybe image or shapes to designate nodes? might need to look into other framework to draw the images? idk
game will play in new popup window
make dialogue node path

will need to figure out how to keep track of both the files name to be displayed and the actual hard coded filename when doing save and loading
make this the prototype of
*/
//in order of importance
//TODO create test dialogue that makes sure all skillcheck and dialogue tree functionality is working
//TODO clean up how dialogue is handled and retrieved so that it universally uses a path and maybe get rid of the extraneous current tree and current node variable
//TODO make an actual system for loading dialogue as data files
//TODO allow an alternate mode that acts as an editor for dialogue to make testing
pub mod appstate;
pub mod databundle;
pub mod dialogue;
pub mod editorstate;
pub mod gamestate;
mod init_gamestate;
pub mod player;
pub mod quest;
pub mod skills;
use appstate::AppState;
use dialogue::*;
use eframe::*;
use fastrand;
use gamestate::*;

use std::env;
use std::io;

use crate::player::Player;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default(),
        ..Default::default()
    };
    eframe::run_native(
        "Dialogue Toolkit",
        options,
        Box::new(|_cc| Ok(Box::<AppState>::default())),
    )
    /*
    match *appstate {
        "game" => {
            //run the game
            let mut gamestate = init_gamestate::init_gamestate();
            //change this to a standalone variable in the main game loop
            while gamestate.quitting == false {
                if gamestate.current_dialogue_tree.is_some() {
                    //then parse through the current dialogue tree
                    progress_tree(&mut gamestate);
                } else if gamestate.current_dialogue_tree.is_none() {
                    //if there is no current dialogue tree then you need to let the player select one
                    select_tree(&mut gamestate);
                }
            }
        }
        "editor" => {
            //run the editor
        }
        _ => {
            //print an error message and terminate the program.
        }
    }*/
}

/*
fn progress_tree(gamestate: &mut GameState) {
    //step through the dialogue tree depending on player choices.
    //check if there are worldstate changes that need to be run. this is done first for cases where ex. the result of a skillcheck
    //determines which dialogue options will be visible.
    //print out what the NPC says formatted with their name
    println!(
        "{}: {}",
        gamestate.current_dialogue_tree.as_ref().unwrap().name(),
        gamestate.current_dialogue_node.as_ref().unwrap().npc_text()
    );
    //then collect the childnode keys into a vec
    let child_node_keys = gamestate
        .current_dialogue_node
        .as_ref()
        .unwrap()
        .child_nodes();
    //if there is no child nodes then that means it is a terminating node and dialogue should be exited by clearing the current dialogue tree and node
    if child_node_keys.is_empty() {
        (
            gamestate.current_dialogue_tree,
            gamestate.current_dialogue_node,
        ) = (None, None);
        return;
    }
    //make a vector for the list of possible options to be shown to the player
    let mut visible_keys: Vec<String> = Vec::new();
    //go through the childnodes and check if they can be displayed
    for key in child_node_keys.iter() {
        let temp_node = gamestate
            .current_dialogue_tree
            .as_ref()
            .unwrap()
            .get(key.clone());
        //check if the dialogue node has a visibility requirement
        match temp_node.visibility_req() {
            //if so then do various gamestate checks depending on the visibility requirement flag
            Some(vis_check) => match vis_check {
                VisibilityConditions::QuestStage(quest_node_path) => {
                    if gamestate
                        .quest_db
                        .get_from_path(quest_node_path.path())
                        .status()
                    {
                        visible_keys.push(key.clone());
                    }
                }
                VisibilityConditions::SkillMinumum(skilltype, level) => {
                    if gamestate.player.skills().get_lvl(skilltype).is_some() {
                        if gamestate.player.skills().get_lvl(skilltype).unwrap() >= level {
                            visible_keys.push(key.clone());
                        }
                    }
                }
                VisibilityConditions::AttributeMinimum(attribute_type, level) => {
                    if gamestate.player.attributes().get_current(attribute_type) >= level {
                        visible_keys.push(key.clone());
                    }
                }
            },
            None => {
                visible_keys.push(key.clone());
            }
        }
    }
    //now print out the pc text of each visible dialogue option
    let mut count: i32 = 1;
    for key in visible_keys.iter() {
        println!(
            "{}. {}",
            count,
            gamestate
                .current_dialogue_tree
                .as_ref()
                .unwrap()
                .get(key.clone())
                .player_text()
        );
        count += 1;
    }
    println!("Please enter which response you wish to choose.");
    //and prompt the player to pick one
    //
    let mut selection = String::new();
    io::stdin()
        .read_line(&mut selection)
        .expect("Failed to read line.");
    //parse the entered text into an i32
    let index = selection.trim().parse::<i32>();
    match index {
        //if the result was properly parsed use it to select a dialogue node and load it into the system.
        Ok(i) => {
            //only try and access the vec if the index is in bounds
            if i <= visible_keys.len() as i32 && i > -1 {
                let selected_response = visible_keys[(i - 1) as usize].clone();
                gamestate.current_dialogue_node = Some(
                    gamestate
                        .current_dialogue_tree
                        .as_ref()
                        .unwrap()
                        .get(selected_response),
                );
                println!(
                    "player_name: {}",
                    gamestate
                        .current_dialogue_node
                        .as_ref()
                        .unwrap()
                        .player_text()
                );
            } else {
                println!("Entered number is out of bounds! Try again.");
            }
        }
        Err(_) => {
            println!(
                "I'm sorry, you didn't enter a number that the system can read. Please try again."
            );
        }
    }
}

fn select_tree(gamestate: &mut GameState) {
    //print out available dialogue trees and let the player select one
    let tree_list = gamestate.dialogue_db.db_list();
    //display all available dialogue trees
    let mut count: i32 = 1;
    for tree in tree_list.iter() {
        println!("{}. {}", count, tree);
        count += 1;
    }
    //add the option to quit
    println!("{}. Quit the game", count);
    //will later be replaced by a character instead of a number to allow for more options
    println!("Please enter the number of the dialogue tree you want to access");
    //ask for a number
    let mut selection = String::new();
    io::stdin()
        .read_line(&mut selection)
        .expect("Failed to read line.");
    //parse the entered text into an i32
    let index = selection.trim().parse::<i32>();
    match index {
        //if the result was properly parsed use it to select a dialogue tree and load it into the system.
        Ok(i) => {
            if i <= tree_list.len() as i32 && i > -1 {
                //if index is inbounds then process it
                // retrieve the key of the selected tree
                let selected_tree = tree_list[(i - 1) as usize].clone();
                //set the dialogue tree to the one selected and the current node to the first of the tree
                gamestate.current_dialogue_tree =
                    Some(gamestate.dialogue_db.get(selected_tree.clone()));
                gamestate.current_dialogue_node = gamestate
                    .dialogue_db
                    .get(selected_tree.clone())
                    .first_node();
            } else if i + 1 == tree_list.len() as i32 {
                //if the player enters a number one above the last option quit the game
                gamestate.quitting = true;
            } else {
                println!("Not a valid option, please try again.");
            }
        }
        Err(_) => {
            println!(
                "I'm sorry, you didn't enter a number that the system can read. Please try again."
            );
        }
    }
}

fn process_cnc(cnc: CheckAndConsequences, state: &mut GameState) {
    let check = cnc.check();
    let mut consequences: (Vec<Consequence>, String) = (Vec::new(), String::new());
    let mut check_result: Option<CheckResult> = None;
    //use a match statement to process the various checks and see which set of consequences need to be carried out
    match check {
        CheckType::TaskCheck(task_check) => {
            let result = process_taskcheck(task_check, state.player.clone());
            if result >= 0 {
                check_result = Some(CheckResult::Success);
            } else {
                check_result = Some(CheckResult::Failure);
            }
        }
        CheckType::ItemCheck(item) => {
            if state.player.inventory().contains_key(&item) {
                check_result = Some(CheckResult::Success);
            } else {
                check_result = Some(CheckResult::Failure);
            }
        }
        CheckType::QuestStageCheck(path) => {
            if state.quest_db.get_from_path(path.path()).status() == true {
                check_result = Some(CheckResult::Success);
            } else {
                check_result = Some(CheckResult::Failure);
            }
        }
    }
    //depending on the result of the check prep the consequences buffer to iterate through
    match check_result.unwrap() {
        CheckResult::Success => {
            consequences = cnc.consequences().success();
        }
        CheckResult::Failure => {
            consequences = cnc.consequences().failure();
        }
    }
    for consequence in consequences.0 {
        match consequence {
            Consequence::DamagePlayer(dmg) => {
                //do this later once the damage attributes function is working.
            }
            Consequence::GivePlayerItem(id, item) => {
                state.player.add_item(id, item);
            }
            Consequence::RemoveItem(id) => {
                state.player.remove_item(id);
            }
            Consequence::CompleteQuestStage(path) => {
                state.quest_db.get_from_path(path.path()).complete();
            }
            Consequence::Custom(_custom_id) => {
                //currently unused, in the future there will be another match statement that parses the
                //custom id into a bespoke function for altering the gamestate. Used as a catchall for when
                //a more complicated or unique consequence is needed thats not worth making an enum flag for
            }
        }
    }
    //set the active dialogue node with the string specified
    // maybe look into changing this into a path?
    state.current_dialogue_node = Some(
        state
            .current_dialogue_tree
            .as_ref()
            .unwrap()
            .get(consequences.1),
    );
}

///takes in taskcheck specifications and runs them, returning the result of the roll as an integer expressing the difference
///between the roll and the task check target. a positive is a success, a negative is a failure. in the future the amount of
///success or failure can be used for degrees of success in special checks
fn process_taskcheck(specs: TaskCheckSpecifications, player: Player) -> i32 {
    //TODO it seems like passing the gamestate into the function here is wasteful look at later
    //first check if its a checkpoint taskcheck, if so just query the relevant player skill level and automatically return a success
    if specs.checkpoint().is_some() {
        //query what bonuses are requested by the checkpoint and then get them from the player
        //first a holder variable for the bonuses/levels that will fail if there's no relevant bonuses
        let mut player_level: Option<i32> = None;
        //holds the amount of bonus if one is present
        let mut bonus: i32 = 0;
        //if there is a skill bonus find it and add it to the bonus
        if specs.skill_bonus().is_some() {
            if player
                .skills()
                .get_lvl(specs.skill_bonus().unwrap())
                .is_some()
            {
                bonus += player
                    .skills()
                    .get_lvl(specs.skill_bonus().unwrap())
                    .unwrap();
                player_level = Some(bonus);
            }
        }
        //if it also takes an attribute bonus then grab the attribute bonus, add it to the bonus, and set the bonus
        //to the player_level
        if specs.attribute_bonus().is_some() {
            bonus += player
                .attributes()
                .get_bonus(specs.attribute_bonus().unwrap());
            player_level = Some(bonus);
        }
        //if player level is none that means the player has NO relevant attribute bonuses or skills for the checkpoint
        //and the checkpoint automatically fails
        //however if there is an amount of bonuses then we check if the player has met the checkpoint.
        if player_level.is_some() {
            if player_level.unwrap() >= specs.checkpoint().unwrap() {
                return 0;
            }
        }
    }
    //END CHECKPOINT LOGIC BLOCK
    //BEGIN REGULAR SKILLCHECK LOGIC BLOCK
    //if the player cannot meet the checkpoint OR the taskcheck is NOT a checkpoint taskcheck do a regular taskcheck
    //roll 2d6, add the relevant bonuses, compare it to the check difficulty. return the difference.
    //initially roll 2d6
    let mut roll = roll_2d6();
    //if the taskcheck uses a skillbonus add the skillbonus to the roll
    if specs.skill_bonus().is_some() {
        roll += player.skills().get_bonus(specs.skill_bonus().unwrap());
    }
    //if the taskcheck uses an attribute bonus add the attribute bonus to the roll
    if specs.attribute_bonus().is_some() {
        roll += player
            .attributes()
            .get_bonus(specs.attribute_bonus().unwrap());
    }
    //this block will be added later and it will query the player inventory for an item with the relevant bonuses

    //set a default check difficulty of 8 in case there isn't one specified
    let mut check_difficulty = 8;
    if specs.difficulty().is_some() {
        //if there is a specified difficulty set the value
        check_difficulty = specs.difficulty().unwrap().value();
    }
    //return the difference between the roll and the check difficulty
    return roll - check_difficulty;
}
*/
///helper function that uses a random number generator to create a simulated d6 roll
fn roll_d6() -> i32 {
    fastrand::i32(1..=6)
}
fn roll_2d6() -> i32 {
    fastrand::i32(1..=6) + fastrand::i32(1..=6)
}
