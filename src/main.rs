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
    /*
    let wild_card_side_quest = Quest::new(vec![
        ("ContactGreatKhans".to_string(), Quest::new(vec![("AssassinatedPapaKhan")])),
        ("ContactBrotherhoodOfSteel", ),
        ("ContactBoomers"),
        ("ContactWhiteGloveSociety"),
        ("Completed", Vec::new())

    ])*/
    quest_database.insert("Boulder".to_string(), boulder_quest);

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
