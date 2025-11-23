use std::collections::HashMap;
///Component that holds all of the known skills a character has
pub struct Skills {
    known_skills: HashMap<SkillType, i32>,
}
impl Skills {
    ///Given a list of skills the character knows it'll turn it into the full component
    ///plus making sure all the needed base skills get added without needing to be made explicit
    pub fn new(skills: Vec<(SkillType, i32)>) -> Self {
        let mut known_skills: HashMap<SkillType, i32> = HashMap::new();
        for skill_tuple in skills.iter() {
            //if there's a base skill/general skill associated with the specialized skill they'll get that added too
            if Skills::get_base_skill(skill_tuple.0).is_some() {
                let base_skill = Skills::get_base_skill(skill_tuple.0).unwrap();
                known_skills.insert(base_skill, 0);
            }
            known_skills.insert(skill_tuple.0, skill_tuple.1);
        }
        Self { known_skills }
    }
    fn get_base_skill(skill: SkillType) -> Option<SkillType> {
        match skill {
            SkillType::MeleeUnarmed => Some(SkillType::Melee),
            SkillType::MeleeBlades => Some(SkillType::Melee),
            SkillType::MeleeBludgeoning => Some(SkillType::Melee),
            SkillType::RangedOneHanded => Some(SkillType::Ranged),
            SkillType::RangedTwoHanded => Some(SkillType::Ranged),
            SkillType::HeavyWeaponsArtillery => Some(SkillType::HeavyWeapons),
            SkillType::HeavyWeaponsPortable => Some(SkillType::HeavyWeapons),
            SkillType::HeavyWeaponsVehicle => Some(SkillType::HeavyWeapons),
            SkillType::AthleticsDexterity => Some(SkillType::Athletics),
            SkillType::AthleticsEndurance => Some(SkillType::Athletics),
            SkillType::AthleticsStrength => Some(SkillType::Athletics),
            SkillType::DriveWheels => Some(SkillType::Drive),
            SkillType::DriveWalker => Some(SkillType::Drive),
            SkillType::DriveTracked => Some(SkillType::Drive),
            _ => None,
        }
    }
    ///Used to get the dice modifier of the skill level a character has
    pub fn get_dm(&self, skill: SkillType) -> i32 {
        //First check if the player actually has the specific skill and if there are any associated base skills
        match (self.known_skills.get(&skill), Skills::get_base_skill(skill)) {
            //if the player has the specific skill being asked for then just return their skill level!
            (Some(skill_val), _) => *skill_val,
            //if they don't have the specific skill being asked for but the skill is one that's part of a general skill w/ specialties
            (None, Some(base_skill)) => {
                //then check if they have training in the base skill associated w/ the one yr checking for
                if self.known_skills.get(&base_skill).is_some() {
                    //and return 0 to represent a base level of competence
                    0
                } else {
                    //if it's a specialty in a general skill they have no training in then they have a DM of -3
                    -3
                }
            }
            //if they don't have the skill and it's not a skill w/ specialties just return the -3 DM
            (None, None) => -3,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
pub enum SkillType {
    /*Combat Skills*/
    //primary combat skills
    Melee,
    MeleeUnarmed,
    MeleeBlades,
    MeleeBludgeoning,
    Ranged,
    RangedOneHanded,
    RangedTwoHanded,
    //secondary combat skills
    Explosives,
    HeavyWeapons,
    HeavyWeaponsArtillery,
    HeavyWeaponsPortable,
    HeavyWeaponsVehicle,
    /*Social Skills*/
    //primary social skills
    Broker,
    Persuade,
    Streetwise,
    //secondary social skills
    Deception,
    Leadership,
    Diplomat,
    /*Knowledge Skills*/
    //primary knowledge skills
    Electronics,
    Investigate,
    Mechanic,
    Medic,
    //secondary knowledge skills
    Admin,
    Advocate,
    Science,
    //possibly science subtypes
    LanguageBasic,
    LanguageBinaricCant,
    LanguageOuterAsh,
    /*Misc Skills*/
    //primary misc skills
    Athletics,
    AthleticsDexterity,
    AthleticsEndurance,
    AthleticsStrength,
    Stealth,
    Survival,
    Recon,
    //secondary misc skills
    AnimalHandling,
    Carouse,
    Drive,
    DriveWheels,
    DriveWalker,
    DriveTracked,
    Gambler,
    Navigation,
    VaccSuit,
}

//
//skill check logic from previous projects
//

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CheckDifficulty {
    Simple,
    Easy,
    Routine,
    Average,
    Difficult,
    VeryDifficult,
    Formidable,
    Impossible,
}
pub enum BoonOrBane {
    Boon,
    Bane,
}

pub enum RollResult {
    Success(i32),
    Failure(i32),
}
pub fn task_check(
    dm: i32,
    check_difficulty: CheckDifficulty,
    boon_or_bane: Option<BoonOrBane>,
) -> RollResult {
    let difficulty = match check_difficulty {
        CheckDifficulty::Simple => 2,
        CheckDifficulty::Easy => 4,
        CheckDifficulty::Routine => 6,
        CheckDifficulty::Average => 8,
        CheckDifficulty::Difficult => 10,
        CheckDifficulty::VeryDifficult => 12,
        CheckDifficulty::Formidable => 14,
        CheckDifficulty::Impossible => 16,
    };
    if boon_or_bane.is_some() {
        let effect = (roll_3d6(boon_or_bane.unwrap()) + dm) - difficulty;
        if effect >= 0 {
            RollResult::Success(effect)
        } else {
            RollResult::Failure(effect)
        }
    } else {
        let effect = (roll_2d6() + dm) - difficulty;
        if effect >= 0 {
            RollResult::Success(effect)
        } else {
            RollResult::Failure(effect)
        }
    }
}

///randomly generates the result of rolling 2d6s
pub fn roll_2d6() -> i32 {
    rand::srand(rand::rand() as u64);
    rand::gen_range(1, 6) + rand::gen_range(1, 6)
}
pub fn roll_3d6(boon_or_bane: BoonOrBane) -> i32 {
    let raw_roll: Vec<i32> = vec![
        rand::gen_range(1, 6),
        rand::gen_range(1, 6),
        rand::gen_range(1, 6),
    ];
    match boon_or_bane {
        BoonOrBane::Bane => raw_roll.iter().sum::<i32>() - raw_roll.iter().max().unwrap(),
        BoonOrBane::Boon => raw_roll.iter().sum::<i32>() - raw_roll.iter().min().unwrap(),
    }
}
