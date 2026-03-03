use crate::dialogue::*;
use crate::gamestate::*;
use crate::player::*;
use crate::quest::*;
use hecs::*;
use std::collections::HashMap;
pub fn init_gamestate() -> GameState {
    GameState {
        world: World::new(),
        dialogue_db: init_dialogue_db(),
        current_dialogue_tree: None,
        current_dialogue_node: None,
        quest_db: init_quest_db(),
        player: init_player(),
        quitting: false,
    }
}

///in the future this will just load things from a folder of files hopefully.
fn init_dialogue_db() -> DialogueDB {
    let mut db = DialogueDB::new();

    let mut humble_farmer = DialogueTree::new("Humble Farmer".to_string());
    humble_farmer.add(
        "Greeting".to_string(),
        DialogueNode::new(
            "null".to_string(),
            "Hello traveller! What brings you to our humble settlement?".to_string(),
            Vec::new(),
            vec!["InquiryHub".to_string(), "ExitDialogue".to_string()],
            None,
            None,
        ),
    );
    {
        humble_farmer.add(
            "InquiryHub".to_string(),
            DialogueNode::new(
                "I have some questions actually.".to_string(),
                "Sure, I'd be happy to help".to_string(),
                vec!["Greeting".to_string()],
                vec![
                    "InquiryShops".to_string(),
                    "InquiryQuests".to_string(),
                    "InquiryPersonal".to_string(),
                    "InquiryArea".to_string(),
                    "ExitInquiryHub".to_string(),
                    "ExitDialogue".to_string(),
                ],
                None,
                None,
            ),
        );
        {
            humble_farmer.add("InquiryShops".to_string(),
            DialogueNode::new("Is there anywhere in this town where I can pick up some supplies?".to_string(),
                "Well, Jeb is the closest thing we have here to a general trader. We don't use hard currency between ourselves, but I'm sure he'd be happy to have a little extra cash the next time the caravan comes by. Don't think he'll have much in terms of armor or weapons though, if that's what you're after. Anything else you want to know?".to_string(),
                vec!["Greeting".to_string()],
                vec!["InquiryHub".to_string()],
                None,
                None));
            //prompt for a quest. this will be changed to include one later, as well as options for turning in the quest etc. once I work out the questflag system
            humble_farmer.add("InquiryQuests".to_string(),
                DialogueNode::new("I'm something of a freelancer, is there anything you'd need doing in the settlement?".to_string(),
                "Actually things have been going well here, I can't really think of anything we'd need done by someone like yourself.".to_string(),
                vec!["InquiryHub".to_string()],
                vec!["InquiryHub".to_string()],
                None,
                None));
            humble_farmer.add("InquiryPersonal".to_string(),
                DialogueNode::new("What can you tell me about yourself?".to_string(),
                "I appreciate the interest but I don't really talk about topics like that with strangers. I work the land here, have for decades. All you need to know.".to_string(),
                vec!["InquiryHub".to_string()],
                vec!["InquiryHub".to_string()],
                None,
                None));
            //flesh this out with more detailed dialogue about the specific locations listed.
            humble_farmer.add(
                "InquiryArea".to_string(),
                DialogueNode::new(
                    "What can you tell me about the area?".to_string(),
                    "temp filler explain area".to_string(),
                    vec!["InquiryHub".to_string()],
                    vec!["InquiryHub".to_string()],
                    None,
                    None,
                ),
            );
            humble_farmer.add(
                "ExitInquiryHub".to_string(),
                DialogueNode::new(
                    "I'm done asking questions for now.".to_string(),
                    "Fine by me, need anything else?".to_string(),
                    vec!["InquiryHub".to_string()],
                    vec!["InquiryHub".to_string()],
                    None,
                    None,
                ),
            );
        }
        humble_farmer.add(
            "ExitDialogue".to_string(),
            DialogueNode::new(
                "Goodbye!".to_string(),
                "Goodbye!".to_string(),
                vec!["Greeting".to_string(), "InquiryHub".to_string()],
                Vec::new(),
                None,
                None,
            ),
        );
    }

    db.add("Humble Farmer".to_string(), humble_farmer);
    db
}

//currently provides an example questdb for testing and a proof of concept but will eventually
//just load the questdb from a file possibly produced with my own primitive tooling
fn init_quest_db() -> QuestDB {
    let mut questdb = QuestDB::new();
    let mut farmer_persuasion = QuestTree::new("Persuade The Farmer".to_string());
    farmer_persuasion.add(
        "SomethingToHide".to_string(),
        QuestNode::new(
            Vec::new(),
            vec![
                "PersuadeTheFarmer".to_string(),
                "TalkToShopkeeper".to_string(),
            ],
        ),
    );
    // farmer_persuasion.add
    questdb
}

fn init_player() -> Player {
    Player::new(
        Attributes::new(8, 8, 8, 8, 8, 8),
        Skills::from_skills(vec![
            (SkillType::Speech, 2),
            (SkillType::Stealth, 0),
            (SkillType::Tech, 1),
            (SkillType::Athletics, 0),
        ]),
        HashMap::new(),
    )
}
