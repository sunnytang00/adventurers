use std::collections::HashMap;

use termgame::{
    run_game, Controller, Game, GameEvent, GameSettings, KeyCode, SimpleEvent, StyledCharacter, GameStyle, GameColor, ViewportLocation,
};
// use adventurers::{player::Player, utils::*};
use crate::{player::{Player, Movement}, block::Block};
pub struct MyGame {
    pub player: Player,
    pub game_map: HashMap<(i32, i32), Block>,
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Controller for MyGame {
    
    fn on_start(&mut self, game: &mut Game) {
        for (key, value) in self.game_map.iter() {
            let colour = match value {
                Block::Barrier => Some(GameColor::White),
                Block::Water => Some(GameColor::Blue),
                Block::Grass => Some(GameColor::Green),
                Block::Sand => Some(GameColor::Yellow),
                Block::Rock => Some(GameColor::Gray),
                Block::Cinderblock => Some(GameColor::LightRed),
                Block::Flowerbush => Some(GameColor::Magenta),
                _ => Some(GameColor::Black),
            };
            game.set_screen_char(key.0, key.1, Some(StyledCharacter::new(' ').style(GameStyle::new().background_color(colour))));
        }

        // //Safe to assume player will always start on a block
        // let ch = game.get_screen_char(self.player.x, self.player.y).unwrap();
        // let bg_colour = ch.style.unwrap().background_color;
        
        // game.set_screen_char(self.player.x, self.player.y, Some(StyledCharacter::new(self.player.char).style(GameStyle::new().background_color(bg_colour))));
    }

    fn on_event(&mut self, game: &mut Game, event: GameEvent) {
        //Get background colour of current player spot
        let ch = game.get_screen_char(self.player.x, self.player.y).unwrap();
        let bg_colour = ch.style.unwrap().background_color;
        let (width, (height, _)) = game.screen_size();
        let (term_width, term_height) = (width - 2, height - 2);

        match event.into() {
            SimpleEvent::Just(KeyCode::Left) => {
                let (x, y) = get_next_position(self.player.x, self.player.y, Direction::Left);
                //Remove previous char of player, and set the background colour to the old colour
                

                if game.get_screen_char(x, y).unwrap().style.unwrap().background_color.unwrap() != GameColor::White  {
                    game.set_screen_char(self.player.x, self.player.y, Some(StyledCharacter::new(' ').style(GameStyle::new().background_color(bg_colour))));
                    self.player.moveLeft();
                    block(game, self.player.x, self.player.y, self.player.char);
                }
            },
            SimpleEvent::Just(KeyCode::Right) => {
                let (x, y) = get_next_position(self.player.x, self.player.y, Direction::Right);
                

                if game.get_screen_char(x, y).unwrap().style.unwrap().background_color.unwrap() != GameColor::White  {
                    game.set_screen_char(self.player.x, self.player.y, Some(StyledCharacter::new(' ').style(GameStyle::new().background_color(bg_colour))));
                    self.player.moveRight();
                    block(game, self.player.x, self.player.y, self.player.char);
                }
            },
            SimpleEvent::Just(KeyCode::Up) => {
                let (x, y) = get_next_position(self.player.x, self.player.y, Direction::Up);
                // eprintln!("{}", self.player.y);
                // eprintln!("{}", term_height);
                // eprintln!("{}", i32::from(term_height) - self.player.y);
                // eprintln!("--");
                // if game.get_screen_char(self.player.x, self.player.y - 2).unwrap().style.unwrap().background_color.unwrap() != GameColor::White {
                    
                //     if i32::from(term_height) - self.player.y >= 17 {
                //         let new_viewport = ViewportLocation {x: game.get_viewport().x, y: game.get_viewport().y - 1};
                //         game.set_viewport(new_viewport)
                //     }
                // }
                

                if game.get_screen_char(x, y).unwrap().style.unwrap().background_color.unwrap() != GameColor::White  {
                    game.set_screen_char(self.player.x, self.player.y, Some(StyledCharacter::new(' ').style(GameStyle::new().background_color(bg_colour))));
                    self.player.moveUp();
                    block(game, self.player.x, self.player.y, self.player.char);
                }
            },
            SimpleEvent::Just(KeyCode::Down) => {
                let (x, y) = get_next_position(self.player.x, self.player.y, Direction::Down);
                //let colour = game.get_screen_char(x, y).unwrap().style.unwrap().background_color.unwrap();

                if game.get_screen_char(self.player.x, self.player.y + 2).unwrap().style.unwrap().background_color.unwrap() != GameColor::White {
                    if i32::from(term_height) - self.player.y <= 3 {
                        let new_viewport = ViewportLocation {x: game.get_viewport().x, y: game.get_viewport().y + 1};
                        game.set_viewport(new_viewport)
                    }
                }
                
                if game.get_screen_char(x, y).unwrap().style.unwrap().background_color.unwrap() != GameColor::White  {
                    game.set_screen_char(self.player.x, self.player.y, Some(StyledCharacter::new(' ').style(GameStyle::new().background_color(bg_colour))));
                    self.player.moveDown();
                    block(game, self.player.x, self.player.y, self.player.char);
                }
                
            },
            _ => {}
        }
        
    }

    fn on_tick(&mut self, _game: &mut Game) {}

}

fn block(game: &mut Game, x: i32, y: i32, player_char: char) {
    let ch_new = game.get_screen_char(x, y).unwrap();
    let bg_colour_new = ch_new.style.unwrap().background_color;
    game.set_screen_char(x, y, Some(StyledCharacter::new(player_char).style(GameStyle::new().background_color(bg_colour_new))));
}

fn move_terminal(game: &mut Game, direction: Direction) {
    
    match direction {
        Direction::Left => todo!(),
        Direction::Right => todo!(),
        Direction::Up => todo!(),
        Direction::Down => todo!(),
    }
}

fn get_next_position(x: i32, y: i32, direction: Direction) -> (i32, i32) {
    let position = match direction {
        Direction::Left => (x - 1, y),
        Direction::Right => (x + 1, y),
        Direction::Up => (x, y - 1),
        Direction::Down => (x, y + 1),
    };
    position
}