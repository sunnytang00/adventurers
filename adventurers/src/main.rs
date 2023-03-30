use adventurers::{player::{Player, Movement}, utils::*};
use std::{error::Error, env::args, fs::{File, self}};
use std::time::Duration;
use termgame::{
    run_game, Controller, Game, GameEvent, GameSettings, KeyCode, SimpleEvent, StyledCharacter,
};

//store coordinates as 2d array index, value is string of the block
//setting block character is a space, background colour is whatever of the block
//when moving character, set player character to the new block, and background colour keep as the same
//vec[0][1] y = 0, x = 1
//use serde or ron crate to read ron files
fn main() -> Result<(), Box<dyn Error>> {
    
    let args = std::env::args().collect::<Vec<_>>();
    //let garbage = &['{','}'];
    let data = fs::read_to_string(&args[1]).expect("Failed to read");
    let parts = data.trim_end()
                            .trim_matches(|ch| ch == '{' || ch == '}')
                            .trim().split("\n").collect::<Vec<_>>();
    for part in parts {
        eprint!("1");
        eprintln!("{}", part);
        
    }
    
    let mut player = Player {x: 3, y: 3, char: '♟'};
    let mut controller = my_game::MyGame {player};

    run_game(
        &mut controller,GameSettings::new()
            // The below are the defaults, but shown so you can edit them.
            .tick_duration(Duration::from_millis(50))
            .quit_event(Some(SimpleEvent::WithControl(KeyCode::Char('c')).into())),
    )?;

    println!("Game Ended!");

    Ok(())
}