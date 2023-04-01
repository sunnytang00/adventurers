use std::collections::HashMap;

use termgame::{
    Controller, Game, GameEvent, KeyCode, SimpleEvent, StyledCharacter, GameStyle, GameColor, ViewportLocation,
};
// use adventurers::{player::Player, utils::*};
use crate::{player::{Player, Movement}, block::{Block, BlockColour}};
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
        //Initalise map
        for (key, value) in self.game_map.iter() {
            let ch = value.get_colour();
            game.set_screen_char(key.0, key.1, ch);
        }

        //Safe to assume player will always start on a block, so we can call unwrap without any fears
        let ch = game.get_screen_char(self.player.x, self.player.y).unwrap();
        let bg_colour = ch.style.unwrap().background_color;
        game.set_screen_char(self.player.x, self.player.y, Some(StyledCharacter::new(self.player.char).style(GameStyle::new().background_color(bg_colour))));
        
        //Work out relative position of player from middle of terminal, treating middle of terminal as origin
        let (width, (height, _)) = game.screen_size();
        let (term_width, term_height) = (width - 2, height - 2);
        let term_middle = (term_width/2, term_height/2);
        
        self.player.rel_x = self.player.rel_x - i32::from(term_middle.0);
        self.player.rel_y = i32::from(term_middle.1) - self.player.rel_y;
    }

    fn on_event(&mut self, game: &mut Game, event: GameEvent) {
        //Get background colour of current player spot before moving
        let ch = game.get_screen_char(self.player.x, self.player.y).unwrap();
        //Need to change below no unwraps
        let bg_colour = ch.style.unwrap().background_color.unwrap();

        let (width, (height, _)) = game.screen_size();
        let (term_width, term_height) = (width - 2, height - 2);

        match event.into() {
            SimpleEvent::Just(KeyCode::Left) => {
                let (x, y) = get_next_position(self.player.x, self.player.y, Direction::Left);
                
                if game.get_screen_char(x, y).unwrap().style.unwrap().background_color.unwrap() != GameColor::White  {
                    //Remove previous char of player, and set the background colour to the old colour
                    game.set_screen_char(self.player.x, self.player.y, create_empty_block(bg_colour));
                    self.player.move_left();
                    block(game, self.player.x, self.player.y, self.player.char);
                }
            },
            SimpleEvent::Just(KeyCode::Right) => {
                let (x, y) = get_next_position(self.player.x, self.player.y, Direction::Right);
                
                if game.get_screen_char(x, y).unwrap().style.unwrap().background_color.unwrap() != GameColor::White  {
                    game.set_screen_char(self.player.x, self.player.y, create_empty_block(bg_colour));
                    self.player.move_right();
                    block(game, self.player.x, self.player.y, self.player.char);
                }
            },
            SimpleEvent::Just(KeyCode::Up) => {
                let (x, y) = get_next_position(self.player.x, self.player.y, Direction::Up);

                if game.get_screen_char(x, y).unwrap().style.unwrap().background_color.unwrap() != GameColor::White {
                    game.set_screen_char(self.player.x, self.player.y, create_empty_block(bg_colour));
                    if i32::from(term_height/2) - self.player.rel_y <= 2 {
                        move_viewport(game, Direction::Up);
                        self.player.move_up();
                    } else {
                        self.player.move_up();
                        self.player.move_rel_up();
                    }
                    block(game, self.player.x, self.player.y, self.player.char);
                }
            },
            SimpleEvent::Just(KeyCode::Down) => {
                let (x, y) = get_next_position(self.player.x, self.player.y, Direction::Down);

                if game.get_screen_char(x, y).unwrap().style.unwrap().background_color.unwrap() != GameColor::White {
                    if i32::from(term_height/2) + self.player.rel_y <= 2 {
                        move_viewport(game, Direction::Down);
                        if game.get_screen_char(x, y).unwrap().style.unwrap().background_color.unwrap() != GameColor::White  {
                            game.set_screen_char(self.player.x, self.player.y, create_empty_block(bg_colour));
                            self.player.move_down();
                            block(game, self.player.x, self.player.y, self.player.char);
                        }
                    } else {
                        if game.get_screen_char(x, y).unwrap().style.unwrap().background_color.unwrap() != GameColor::White  {
                            game.set_screen_char(self.player.x, self.player.y, create_empty_block(bg_colour));
                            self.player.move_down();
                            self.player.move_rel_down();
                            block(game, self.player.x, self.player.y, self.player.char);
                        }
                    }
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

fn move_viewport(game: &mut Game, direction: Direction) {
    let new_viewport = match direction {
        Direction::Left => ViewportLocation {x: game.get_viewport().x - 1, y: game.get_viewport().y},
        Direction::Right => ViewportLocation {x: game.get_viewport().x +1 , y: game.get_viewport().y},
        Direction::Up => ViewportLocation {x: game.get_viewport().x, y: game.get_viewport().y - 1},
        Direction::Down => ViewportLocation {x: game.get_viewport().x, y: game.get_viewport().y + 1},
    };
    game.set_viewport(new_viewport);
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

fn create_empty_block(colour: GameColor) -> Option<StyledCharacter> {
    Some(StyledCharacter::new(' ').style(GameStyle::new().background_color(Some(colour))))
}