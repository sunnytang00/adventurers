use adventurers::utils::my_game;
use adventurers_quest::quest::Quest;
use adventurers_quest::quest_action::QuestAction;
use adventurers_quest::quest_task::QuestTask;
use adventurers_quest::utils::QuestStatus;
use lib::{block::Block, player::Player};
use std::time::Duration;
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

    let quest = match quest_id.as_str() {
        "q1" => {
            let mut tasks: Vec<QuestTask> = Vec::new();

            tasks.push(QuestTask {
                status: QuestStatus::Ongoing,
                task: QuestAction::Walk,
                object: Block::Sand,
                no_of_times: 5,
                x_more_times: 5,
            });

            Quest {
                status: QuestStatus::Ongoing,
                tasks: tasks,
            }
        }
        "q2" => {
            let mut tasks: Vec<QuestTask> = Vec::new();
            
            tasks.push(QuestTask {
                status: QuestStatus::Ongoing,
                task: QuestAction::Collect,
                object: Block::Object('x'),
                no_of_times: 5,
                x_more_times: 5,
            });

            tasks.push(QuestTask {
                status: QuestStatus::Ongoing,
                task: QuestAction::Collect,
                object: Block::Object('y'),
                no_of_times: 3,
                x_more_times: 3,
            });

            Quest {
                status: QuestStatus::Ongoing,
                tasks: tasks,
            }
        },
        "q3" => todo!(),
        _ => Quest {
            status: QuestStatus::Complete,
            tasks: Vec::new(),
        },
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
