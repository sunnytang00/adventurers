use adventurers::{player::{Player, Movement}, utils::*};
use std::{error::Error, env::args, fs::{File, self}, collections::HashMap};
use std::time::Duration;
use termgame::{
    run_game, Controller, Game, GameEvent, GameSettings, KeyCode, SimpleEvent, StyledCharacter,
};
use serde::Deserialize;
use adventurers::block::Block;
// #[derive(Deserialize, Debug)]
// struct Block {
//     position: (i32, i32),
//     block_type: String,
// }
//store coordinates as 2d array index, value is string of the block
//setting block character is a space, background colour is whatever of the block
//when moving character, set player character to the new block, and background colour keep as the same
//vec[0][1] y = 0, x = 1
//use serde or ron crate to read ron files
fn main() -> Result<(), Box<dyn Error>> {
    //Reading in args
    let args = std::env::args().collect::<Vec<_>>();

    //Map file
    let data = fs::read_to_string(&args[1]).expect("Failed to read map file.");

    //Deseralise
    let game_map: HashMap<(i32, i32), Block> = ron::from_str(data.as_str()).expect("Map file is in wrong format.");

    let player = Player {x: 3, y: 3, char: '♟'};
    let mut controller = my_game::MyGame {player, game_map};
    
    run_game(
        &mut controller,GameSettings::new()
            // The below are the defaults, but shown so you can edit them.
            .tick_duration(Duration::from_millis(50))
            .quit_event(Some(SimpleEvent::WithControl(KeyCode::Char('c')).into())),
    )?;
    
    println!("Game Ended!");

    Ok(())
}