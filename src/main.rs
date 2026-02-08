pub mod dialogue;
pub mod gamestate;
mod init_gamestate;
pub mod skills;
use dialogue::*;
use gamestate::*;
use hecs::*;
use std::io;

fn main() {
    let mut gamestate = init_gamestate::init_gamestate();
    //change this to a standalone variable in the main game loop
    while gamestate.quitting == false {
        let tree_list = gamestate.dialogue_db.db_list();

        match gamestate.current_dialogue_tree {
            Some(_) => {
                //parse through the current dialogue tree
                let current_node = gamestate.current_dialogue_node.unwrap().clone();
            }
            None => {
                //display all available dialogue trees
                let mut count: i32 = 1;
                for tree in tree_list.iter() {
                    println!("{}. {}", count, tree);
                    count += 1;
                }
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
                        let selected_tree = tree_list[(i - 1) as usize].clone();
                        gamestate.current_dialogue_tree =
                            Some(gamestate.dialogue_db.get(selected_tree.clone()));
                        gamestate.current_dialogue_node = gamestate
                            .dialogue_db
                            .get(selected_tree.clone())
                            .first_node();
                        // gamestate.current_dialogue_tree.unwrap().first_node();
                    }
                    Err(_) => {
                        println!(
                            "I'm sorry, you didn't enter a number that the system can read. Please try again."
                        );
                    }
                }
            }
        }
    }
}
