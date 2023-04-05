use adventurers::utils::my_game;
use adventurers_quest::quest::{create_quest, Condition};
use adventurers_quest::quest_action::QuestAction;
use adventurers_quest::quest_task::create_task;
use lib::{block::Block, player::Player};
use std::time::{Duration, SystemTime};
use std::{
    collections::HashMap,
    error::Error,
    fs::{self},
};
use termgame::{run_game, GameSettings, KeyCode, SimpleEvent};

fn main() -> Result<(), Box<dyn Error>> {
    //Reading in args
    let args = std::env::args().collect::<Vec<_>>();

    //Map file
    let data = fs::read_to_string(&args[1]).expect("Failed to read map file.");

    let quest_id = &args[2];

    let def_time = SystemTime::now();

    let quest = match quest_id.as_str() {
        "q1" => {
            let mut quest = create_quest(None);
            create_task(&mut quest, QuestAction::Walk, Block::Sand, 5, def_time);
            quest
        }
        "q2" => {
            let mut quest = create_quest(Some(Condition::InOrder));
            create_task(
                &mut quest,
                QuestAction::Collect,
                Block::Object('x'),
                5,
                def_time,
            );
            create_task(
                &mut quest,
                QuestAction::Collect,
                Block::Object('y'),
                3,
                def_time,
            );
            quest
        }
        "q3" => create_quest(None),
        _ => todo!(), //Can implement future quests
    };

    //Deseralise
    let game_map: HashMap<(i32, i32), Block> =
        ron::from_str(data.as_str()).expect("Map file is in wrong format.");

    let player = Player {
        x: 2,
        y: 2,
        rel_x: 2,
        rel_y: 2,
        breath: 10,
        char: '♟',
    };
    
    //Game state 0 represents OK, game state 1 represents that game will end on next move
    let mut controller = my_game::MyGame {
        player,
        game_map,
        game_state: 0,
        sign_msg: None,
        quest: quest,
    };

    run_game(
        &mut controller,
        GameSettings::new()
            // The below are the defaults, but shown so you can edit them.
            .tick_duration(Duration::from_millis(50))
            .quit_event(Some(SimpleEvent::WithControl(KeyCode::Char('c')).into())),
    )?;

    println!("Game Ended!");

    Ok(())
}
