use std::collections::HashMap;
struct State {
    quest_db: QuestDatabase,
}

impl State {
    fn new() -> Self {
        Self {
            quest_db: init_quest_db(),
        }
    }
}

fn init_quest_db() -> QuestDatabase {
    let mut quest_database: HashMap<String, Quest> = HashMap::new();
    let boulder_quest = Quest::new(vec![
        (
            "PushedBoulder".to_string(),
            Quest::new(vec![
                ("Failure".to_string(), Quest::new(Vec::new())),
                ("PartialSuccess".to_string(), Quest::new(Vec::new())),
                ("FullSuccess".to_string(), Quest::new(Vec::new())),
            ]),
        ),
        ("ExplodedBoulder".to_string(), Quest::new(Vec::new())),
    ]);
    quest_database.insert("Boulder".to_string(), boulder_quest);
    let complex_quest = Quest::new(vec![
        //the player can simply get the item by being able to steal it from somewhere
        ("GetKey".to_string(), Quest::new(Vec::new())),
        //they may have the necessary tech skill to hack the door themselves
        ("HackedDoorAlone".to_string(), Quest::new(Vec::new())),
        //the player can also recruit someone to hack the door for them!
        (
            String::from("UsedOutsideHacker"),
            Quest::new(vec![
                //they can bribe the hacker which is the simplest but requires 50 credits
                ("BribedTheHacker".to_string(), Quest::new(Vec::new())),
                //they can try and convince the hacker its in their best interest to help
                (
                    "PersuadedTheHacker".to_string(),
                    Quest::new(vec![
                        //if they fail they'll need to pay the hacker even more b/c they've revealed themselves to be desparate
                        (
                            "Failure".to_string(),
                            Quest::new(vec![(
                                String::from("PaidMoreExpensiveBribe"),
                                Quest::new(Vec::new()),
                            )]),
                        ),
                        //if they partially succeed they can get the hacker to work for them in exchange for a favor
                        (
                            "PartialSuccess".to_string(),
                            Quest::new(vec![(
                                "CompletedFavor".to_string(),
                                Quest::new(Vec::new()),
                            )]),
                        ),
                        //if they fully succeed they can get the hacker to work for them without any comepensation
                        ("FullSuccess".to_string(), Quest::new(Vec::new())),
                    ]),
                ),
                //they can also try to intimidate the hacker into working for them for free
                (
                    "ThreatenedTheHacker".to_string(),
                    Quest::new(vec![
                        //if they failed to intimidate the hacker they need to kill one of the hacker's friends
                        //to threaten him fully
                        (
                            "Failure".to_string(),
                            Quest::new(vec![(
                                String::from("DecidedToKillHackerFriends"),
                                Quest::new(Vec::new()),
                            )]),
                        ),
                        //if they partially succeed then they can either take a reduced rate or they can press onward and mutilate
                        //the hacker lessening their reputation w/ a faction
                        (
                            "PartialSuccess".to_string(),
                            Quest::new(vec![
                                (String::from("AcceptLowerRate"), Quest::new(Vec::new())),
                                (String::from("MutilateHacker"), Quest::new(Vec::new())),
                            ]),
                        ),
                        //if the player is able to fully successfully intimidate the hacker then he'll agree to help
                        //without any payment
                        ("FullSuccess".to_string(), Quest::new(Vec::new())),
                    ]),
                ),
            ]),
        ),
    ]);
    quest_database.insert(String::from("HackDoor"), complex_quest);
    QuestDatabase::new(quest_database)
}

fn main() {
    let mut state = State::new();
    state
        .quest_db
        .quest("Boulder")
        .step("PushedBoulder")
        .unwrap()
        .step("PartialSuccess")
        .unwrap()
        .complete();

    //

    println!("{:?}", state.quest_db);
}

#[derive(Clone, Debug)]
pub struct QuestDatabase {
    contents: HashMap<String, Quest>,
}
impl QuestDatabase {
    pub fn new(contents: HashMap<String, Quest>) -> Self {
        Self { contents }
    }
    pub fn quest(&mut self, key: &str) -> &mut Quest {
        self.contents
            .get_mut(key)
            .expect("Invalid key caused failure to retrieve quest.")
    }
    //make update quest function
    pub fn update(&mut self, key: &str, value: Quest) {
        self.contents.insert(key.to_string(), value);
    }
}
#[derive(Clone, Debug)]
pub struct Quest {
    is_complete: bool,
    // mutually_exclusive: bool,
    choices: Vec<(String, Quest)>,
}

impl Quest {
    pub fn new(choices: Vec<(String, Quest)>) -> Self {
        Self {
            is_complete: false,
            choices,
        }
    }

    pub fn step(&mut self, step_name: &str) -> Option<&mut Quest> {
        for (choicename, ref mut questnode) in self.choices.iter_mut() {
            if step_name == choicename {
                return Some(questnode);
            }
        }
        return None;
    }
    pub fn complete(&mut self) {
        self.is_complete = true;
    }
    pub fn status(&self) -> bool {
        self.is_complete
    }
}
