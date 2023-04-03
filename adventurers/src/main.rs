use adventurers::utils::{block::Block, my_game, player::Player};
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

    //Deseralise
    let game_map: HashMap<(i32, i32), Block> =
        ron::from_str(data.as_str()).expect("Map file is in wrong format.");

    let player = Player {
        x: 3,
        y: 3,
        rel_x: 3,
        rel_y: 3,
        breath: 10,
        char: '♟',
    };
    let mut controller = my_game::MyGame {
        player,
        game_map,
        game_state: 0,
        sign_msg: None,
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
