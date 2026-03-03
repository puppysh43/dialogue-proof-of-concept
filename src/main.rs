/*
SCOPE OF PROJECT
No ECS, no data for NPCs outside of associated dialogue and quests selected from the menu.
create 1 simple quest one intermediate quest and one more complicated quest
find way for quest and dialogue data to be loaded from a file instead of being hardcoded as much as possible
additionally look into making extremely basic proof of concept tooling for creating at LEAST quest db
player will have inventory for quest items, attributes, and skills, inspired by but pared down from MgT2e
obviously logic for interfacing with these things will change when added to an actual game especially b/c an ECS is probably
*/
pub mod dialogue;
pub mod gamestate;
mod init_gamestate;
pub mod player;
pub mod quest;
pub mod skills;
use dialogue::*;
use gamestate::*;
use hecs::*;
use std::io;

fn main() {
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
