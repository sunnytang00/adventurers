use adventurers::{player::{Player, Movement}, utils::*};
use std::{error::Error, env::args, fs::{File, self}, collections::HashMap};
use std::time::Duration;
use termgame::{
    run_game, Controller, Game, GameEvent, GameSettings, KeyCode, SimpleEvent, StyledCharacter,
};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Block {
    position: (i32, i32),
    block_type: String,
}
//store coordinates as 2d array index, value is string of the block
//setting block character is a space, background colour is whatever of the block
//when moving character, set player character to the new block, and background colour keep as the same
//vec[0][1] y = 0, x = 1
//use serde or ron crate to read ron files
fn main() -> Result<(), Box<dyn Error>> {
    let args = std::env::args().collect::<Vec<_>>();

    let mut data = fs::read_to_string(&args[1]).expect("Failed to read");
    data = data.replace(" ", "");
    let ll = (3, 4);
    let mut game_map: HashMap<(i32, i32), &str> = HashMap::new();
    let parts = data.trim_end()
                            .trim_matches(|ch| ch == '{' || ch == '}')
                            .trim()
                            .split("\n").collect::<Vec<_>>();
    
    for part in parts {
        let new_vec = part.split(":").collect::<Vec<_>>();
        //eprintln!("{}", new_vec[0]);
        let coord_vec = new_vec[0].split(",").collect::<Vec<_>>();
        let position = (coord_vec[0].replace("(", "").parse::<i32>().unwrap(), coord_vec[1].replace(")", "").parse::<i32>().unwrap());
        //eprintln!("{}", position);
        //let tuple = serde_json::from_str::<(u32, u32)>(new_vec[0]).expect("There was a problem with the coordinates");

        
        game_map.insert(position, new_vec[1]);
        
    }
    eprintln!("{}", game_map.get(&(1, 4)).unwrap());
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