use std::collections::HashMap;

use termgame::{
    Controller, Game, GameEvent, KeyCode, SimpleEvent, StyledCharacter, GameStyle, GameColor, ViewportLocation, Message
};

use crate::utils::block::{Block, BlockColour, SignText};
use crate::utils::direction::Direction;
use crate::utils::player::{Player, Breath, Movement};
use crate::utils::saved_block::SavedBlock;

pub struct MyGame {
    pub player: Player,
    pub game_map: HashMap<(i32, i32), Block>,
    pub game_state: i32,
    pub saved_block: Option<SavedBlock>,
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
        if self.game_state == 1 {
            game.end_game();
        }
        //Get background colour of current player spot before moving
        let ch = game.get_screen_char(self.player.x, self.player.y).unwrap();
        //Need to change below no unwraps
        let bg_colour = ch.style.unwrap().background_color.unwrap();

        let (width, (height, _)) = game.screen_size();
        let (term_width, term_height) = (width - 2, height - 2);
        
        match event.into() {
            SimpleEvent::Just(KeyCode::Left) => {
                let (x, y) = get_next_position(self.player.x, self.player.y, Direction::Left);
                
                if game.get_screen_char(x, y).is_none() {
                    self.game_map.insert((x, y), Block::Empty);
                    game.set_screen_char(x, y, create_empty_block(GameColor::Black));
                }

                if game.get_screen_char(x, y).unwrap().style.unwrap().background_color.unwrap() != GameColor::White {
                    //Delete old character - replace the sign if we moved onto it
                    if self.saved_block.is_some() {
                        game.set_screen_char(self.player.x, self.player.y, Some(StyledCharacter::new('💬').style(GameStyle::new().background_color(Some(GameColor::Black)))));
                        self.saved_block = None;
                    } else {
                        game.set_screen_char(self.player.x, self.player.y, create_empty_block(bg_colour));
                    }
                    
                    //If player is going to move onto a sign block, save info about sign block
                    if game.get_screen_char(x, y).unwrap().c == '💬' {
                        self.saved_block = Some(SavedBlock {x, y, block: Block::Sign(self.game_map.get(&(x, y)).unwrap().get_sign_text())});
                    }

                    if i32::from(term_width/2) + self.player.rel_x <= 2 {
                        move_viewport(game, Direction::Left);
                    } else {
                        self.player.move_dir_rel(Direction::Left);
                    }
                    self.player.move_dir(Direction::Left);
                    add_player_block(game, self.player.x, self.player.y, self.player.char);

                    

                    if game.get_screen_char(self.player.x, self.player.y).unwrap().style.unwrap().background_color.unwrap() ==  GameColor::Blue {
                        self.player.decrease_breath();
                    } else {
                        self.player.reset_breath();
                    }
                }
                
            },
            SimpleEvent::Just(KeyCode::Right) => {
                let (x, y) = get_next_position(self.player.x, self.player.y, Direction::Right);

                if game.get_screen_char(x, y).is_none() {
                    self.game_map.insert((x, y), Block::Empty);
                    game.set_screen_char(x, y, create_empty_block(GameColor::Black));
                }
            
                if game.get_screen_char(x, y).unwrap().style.unwrap().background_color.unwrap() != GameColor::White {

                    if self.saved_block.is_some() {
                        game.set_screen_char(self.player.x, self.player.y, Some(StyledCharacter::new('💬').style(GameStyle::new().background_color(Some(GameColor::Black)))));
                        self.saved_block = None;
                    } else {
                        game.set_screen_char(self.player.x, self.player.y, create_empty_block(bg_colour));
                    }

                    if game.get_screen_char(x, y).unwrap().c == '💬' {
                        self.saved_block = Some(SavedBlock {x, y, block: Block::Sign(self.game_map.get(&(x, y)).unwrap().get_sign_text())});
                    }

                    if i32::from(term_width/2) - self.player.rel_y <= 2 {
                        move_viewport(game, Direction::Right);
                    } else {
                        self.player.move_dir_rel(Direction::Right);
                    }
                    self.player.move_dir(Direction::Right);
                    add_player_block(game, self.player.x, self.player.y, self.player.char);

                    if game.get_screen_char(self.player.x, self.player.y).unwrap().style.unwrap().background_color.unwrap() ==  GameColor::Blue {
                        self.player.decrease_breath();
                    } else {
                        self.player.reset_breath();
                    }
                }
            },
            SimpleEvent::Just(KeyCode::Up) => {
                let (x, y) = get_next_position(self.player.x, self.player.y, Direction::Up);

                if game.get_screen_char(x, y).is_none() {
                    self.game_map.insert((x, y), Block::Empty);
                    game.set_screen_char(x, y, create_empty_block(GameColor::Black));
                }

                if game.get_screen_char(x, y).expect("Empty1").style.expect("Empty2").background_color.expect("Empty") != GameColor::White {
                    
                    if self.saved_block.is_some() {
                        game.set_screen_char(self.player.x, self.player.y, Some(StyledCharacter::new('💬').style(GameStyle::new().background_color(Some(GameColor::Black)))));
                        self.saved_block = None;
                    } else {
                        eprintln!("here");
                        game.set_screen_char(self.player.x, self.player.y, create_empty_block(bg_colour));
                    }

                    if game.get_screen_char(x, y).unwrap().c == '💬' {
                        self.saved_block = Some(SavedBlock {x, y, block: Block::Sign(self.game_map.get(&(x, y)).unwrap().get_sign_text())});
                    }
                    
                    if i32::from(term_height/2) - self.player.rel_y <= 2 {
                        move_viewport(game, Direction::Up);
                    } else {
                        self.player.move_dir_rel(Direction::Up);
                    }
                    self.player.move_dir(Direction::Up);
                    add_player_block(game, self.player.x, self.player.y, self.player.char);

                    if game.get_screen_char(self.player.x, self.player.y).unwrap().style.unwrap().background_color.unwrap() ==  GameColor::Blue {
                        self.player.decrease_breath();
                    } else {
                        self.player.reset_breath();
                    }
                }
            },
            SimpleEvent::Just(KeyCode::Down) => {
                let (x, y) = get_next_position(self.player.x, self.player.y, Direction::Down);

                if game.get_screen_char(x, y).is_none() {
                    self.game_map.insert((x, y), Block::Empty);
                    game.set_screen_char(x, y, create_empty_block(GameColor::Black));
                }

                if game.get_screen_char(x, y).unwrap().style.unwrap().background_color.unwrap() != GameColor::White {
                    
                    if self.saved_block.is_some() {
                        game.set_screen_char(self.player.x, self.player.y, Some(StyledCharacter::new('💬').style(GameStyle::new().background_color(Some(GameColor::Black)))));
                        self.saved_block = None;
                    } else {
                        game.set_screen_char(self.player.x, self.player.y, create_empty_block(bg_colour));
                    }

                    if game.get_screen_char(x, y).unwrap().c == '💬' {
                        self.saved_block = Some(SavedBlock {x, y, block: Block::Sign(self.game_map.get(&(x, y)).unwrap().get_sign_text())});
                    }

                    if i32::from(term_height/2) + self.player.rel_y <= 2 {
                        move_viewport(game, Direction::Down);
                    } else { 
                        self.player.move_dir_rel(Direction::Down);
                    }
                    self.player.move_dir(Direction::Down);
                    add_player_block(game, self.player.x, self.player.y, self.player.char);

                    if game.get_screen_char(self.player.x, self.player.y).unwrap().style.unwrap().background_color.unwrap() ==  GameColor::Blue {
                        self.player.decrease_breath();
                    } else {
                        self.player.reset_breath();
                    }
                }
            },
            _ => {}
        }

        //Sign_msg contains the string in the sign block, if block is not a sign, sign_msg is empty string
        let sign_msg = self.game_map.get(&(self.player.x, self.player.y)).unwrap().get_sign_text();


        if sign_msg != "" {
            game.set_message(Some(Message::new(String::from(sign_msg)).title("Adventurers".to_owned())));
        } else {
            game.set_message(None);
        }

        if self.player.get_breath() == 0 {
            game.set_message(Some(Message::new(String::from("YOU DIED")).title("Adventurers".to_owned())));
            self.game_state = 1;
        }

    }

    fn on_tick(&mut self, _game: &mut Game) {

    }

}

fn add_player_block(game: &mut Game, x: i32, y: i32, player_char: char) {
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