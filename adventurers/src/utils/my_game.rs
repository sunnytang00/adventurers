use std::collections::HashMap;

use adventurers_quest::quest::{Quest, Questing};
use termgame::{
    Controller, Game, GameColor, GameEvent, GameStyle, KeyCode, Message, SimpleEvent,
    StyledCharacter, ViewportLocation,
};

use lib::block::{Block, BlockColour, SignText};
use lib::direction::Direction;
use lib::player::{Breath, Movement, Player};

pub struct MyGame {
    pub player: Player,
    pub game_map: HashMap<(i32, i32), Block>,
    pub game_state: i32,
    pub sign_msg: Option<String>,
    pub quest: Quest,
}

impl Controller for MyGame {
    fn on_start(&mut self, game: &mut Game) {
        //Initalise map
        for (key, value) in self.game_map.iter() {
            let ch = value.get_colour();
            game.set_screen_char(key.0, key.1, ch);
        }

        //Safe to assume player will always start on a block, so we can call unwrap without any fears
        let bg_colour = get_background_color(game, self.player.x, self.player.y);
        game.set_screen_char(
            self.player.x,
            self.player.y,
            Some(
                StyledCharacter::new(self.player.char)
                    .style(GameStyle::new().background_color(Some(bg_colour))),
            ),
        );

        //Work out relative position of player from middle of terminal, treating middle of terminal as origin
        let (width, (height, _)) = game.screen_size();
        let (term_width, term_height) = (width - 2, height - 2);
        let term_middle = (term_width / 2, term_height / 2);

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
            SimpleEvent::Just(KeyCode::Char('q')) => {
                game.set_message(Some(
                    Message::new(self.quest.to_string()).title("Adventurers".to_string()),
                ));
            }

            SimpleEvent::Just(KeyCode::Char('r')) => {
                self.quest.reset_quest();
                game.set_message(None);
            }

            SimpleEvent::Just(KeyCode::Left) => {
                let (x, y) = get_next_position(self.player.x, self.player.y, Direction::Left);

                //If map is missing fields, before we move, create a block in hashmap and termgame map
                if game.get_screen_char(x, y).is_none() {
                    self.game_map.insert((x, y), Block::Empty);
                    game.set_screen_char(x, y, create_empty_block(GameColor::Black));
                }

                if get_block(&self.game_map, (x, y)) != &Block::Barrier {
                    self.quest.update_quests(get_block(&self.game_map, (x, y)));
                    //Delete old character - replace the sign if we moved onto it
                    if self.sign_msg.is_some() {
                        game.set_screen_char(
                            self.player.x,
                            self.player.y,
                            Some(
                                StyledCharacter::new('💬').style(
                                    GameStyle::new().background_color(Some(GameColor::Black)),
                                ),
                            ),
                        );
                        self.sign_msg = None;
                    } else {
                        game.set_screen_char(
                            self.player.x,
                            self.player.y,
                            create_empty_block(bg_colour),
                        );
                    }

                    //If player is going to move onto a sign block, save info about sign block

                    match game.get_screen_char(x, y) {
                        Some(styled_c) => {
                            if styled_c.c == '💬' {
                                let sign_txt = match self.game_map.get(&(x, y)) {
                                    Some(x) => match x {
                                        Block::Sign(_) => x.get_sign_text(),
                                        _ => panic!(),
                                    },
                                    None => panic!(),
                                };

                                self.sign_msg = Some(sign_txt);
                            }
                        }
                        None => panic!(),
                    };

                    if i32::from(term_width / 2) + self.player.rel_x <= 2 {
                        move_viewport(game, Direction::Left);
                    } else {
                        self.player.move_dir_rel(Direction::Left);
                    }
                    self.player.move_dir(Direction::Left);

                    add_player_block(game, self.player.x, self.player.y, self.player.char);

                    if get_block(&self.game_map, (self.player.x, self.player.y)) == &Block::Water {
                        self.player.decrease_breath();
                    } else {
                        self.player.reset_breath();
                    }
                    
                }
                match &self.sign_msg {
                    Some(message) => game.set_message(Some(
                        Message::new(message.to_string()).title("Adventurers".to_string()),
                    )),
                    None => game.set_message(None),
                };
            }
            SimpleEvent::Just(KeyCode::Right) => {
                let (x, y) = get_next_position(self.player.x, self.player.y, Direction::Right);

                if game.get_screen_char(x, y).is_none() {
                    self.game_map.insert((x, y), Block::Empty);
                    game.set_screen_char(x, y, create_empty_block(GameColor::Black));
                }

                if get_block(&self.game_map, (x, y)) != &Block::Barrier {
                    self.quest.update_quests(get_block(&self.game_map, (x, y)));
                    if self.sign_msg.is_some() {
                        game.set_screen_char(
                            self.player.x,
                            self.player.y,
                            Some(
                                StyledCharacter::new('💬').style(
                                    GameStyle::new().background_color(Some(GameColor::Black)),
                                ),
                            ),
                        );
                        self.sign_msg = None;
                    } else {
                        game.set_screen_char(
                            self.player.x,
                            self.player.y,
                            create_empty_block(bg_colour),
                        );
                    }

                    match game.get_screen_char(x, y) {
                        Some(styled_c) => {
                            if styled_c.c == '💬' {
                                let sign_txt = match self.game_map.get(&(x, y)) {
                                    Some(x) => match x {
                                        Block::Sign(_) => x.get_sign_text(),
                                        _ => panic!(),
                                    },
                                    None => panic!(),
                                };

                                self.sign_msg = Some(sign_txt);
                            }
                        }
                        None => panic!(),
                    };

                    if i32::from(term_width / 2) - self.player.rel_y <= 2 {
                        move_viewport(game, Direction::Right);
                    } else {
                        self.player.move_dir_rel(Direction::Right);
                    }
                    self.player.move_dir(Direction::Right);
                    add_player_block(game, self.player.x, self.player.y, self.player.char);

                    if get_block(&self.game_map, (self.player.x, self.player.y)) == &Block::Water {
                        self.player.decrease_breath();
                    } else {
                        self.player.reset_breath();
                    }
                }
                match &self.sign_msg {
                    Some(message) => game.set_message(Some(
                        Message::new(message.to_string()).title("Adventurers".to_string()),
                    )),
                    None => game.set_message(None),
                };
            }
            SimpleEvent::Just(KeyCode::Up) => {
                let (x, y) = get_next_position(self.player.x, self.player.y, Direction::Up);

                if game.get_screen_char(x, y).is_none() {
                    self.game_map.insert((x, y), Block::Empty);
                    game.set_screen_char(x, y, create_empty_block(GameColor::Black));
                }

                if get_block(&self.game_map, (x, y)) != &Block::Barrier {
                    self.quest.update_quests(get_block(&self.game_map, (x, y)));
                    if self.sign_msg.is_some() {
                        game.set_screen_char(
                            self.player.x,
                            self.player.y,
                            Some(
                                StyledCharacter::new('💬').style(
                                    GameStyle::new().background_color(Some(GameColor::Black)),
                                ),
                            ),
                        );
                        self.sign_msg = None;
                    } else {
                        game.set_screen_char(
                            self.player.x,
                            self.player.y,
                            create_empty_block(bg_colour),
                        );
                    }

                    match game.get_screen_char(x, y) {
                        Some(styled_c) => {
                            if styled_c.c == '💬' {
                                let sign_txt = match self.game_map.get(&(x, y)) {
                                    Some(x) => match x {
                                        Block::Sign(_) => x.get_sign_text(),
                                        _ => panic!(),
                                    },
                                    None => panic!(),
                                };

                                self.sign_msg = Some(sign_txt);
                            }
                        }
                        None => panic!(),
                    };

                    if i32::from(term_height / 2) - self.player.rel_y <= 2 {
                        move_viewport(game, Direction::Up);
                    } else {
                        self.player.move_dir_rel(Direction::Up);
                    }
                    self.player.move_dir(Direction::Up);
                    add_player_block(game, self.player.x, self.player.y, self.player.char);

                    if get_block(&self.game_map, (self.player.x, self.player.y)) == &Block::Water {
                        self.player.decrease_breath();
                    } else {
                        self.player.reset_breath();
                    }
                }
                match &self.sign_msg {
                    Some(message) => game.set_message(Some(
                        Message::new(message.to_string()).title("Adventurers".to_string()),
                    )),
                    None => game.set_message(None),
                };
            }
            SimpleEvent::Just(KeyCode::Down) => {
                let (x, y) = get_next_position(self.player.x, self.player.y, Direction::Down);

                if game.get_screen_char(x, y).is_none() {
                    self.game_map.insert((x, y), Block::Empty);
                    game.set_screen_char(x, y, create_empty_block(GameColor::Black));
                }

                if get_block(&self.game_map, (x, y)) != &Block::Barrier {
                    self.quest.update_quests(get_block(&self.game_map, (x, y)));
                    if self.sign_msg.is_some() {
                        game.set_screen_char(
                            self.player.x,
                            self.player.y,
                            Some(
                                StyledCharacter::new('💬').style(
                                    GameStyle::new().background_color(Some(GameColor::Black)),
                                ),
                            ),
                        );
                        self.sign_msg = None;
                    } else {
                        game.set_screen_char(
                            self.player.x,
                            self.player.y,
                            create_empty_block(bg_colour),
                        );
                    }

                    match game.get_screen_char(x, y) {
                        Some(styled_c) => {
                            if styled_c.c == '💬' {
                                let sign_txt = match self.game_map.get(&(x, y)) {
                                    Some(x) => match x {
                                        Block::Sign(_) => x.get_sign_text(),
                                        _ => panic!(),
                                    },
                                    None => panic!(),
                                };

                                self.sign_msg = Some(sign_txt);
                            }
                        }
                        None => panic!(),
                    };

                    if i32::from(term_height / 2) + self.player.rel_y <= 2 {
                        move_viewport(game, Direction::Down);
                    } else {
                        self.player.move_dir_rel(Direction::Down);
                    }

                    self.player.move_dir(Direction::Down);
                    add_player_block(game, self.player.x, self.player.y, self.player.char);

                    if get_block(&self.game_map, (self.player.x, self.player.y)) == &Block::Water {
                        self.player.decrease_breath();
                    } else {
                        self.player.reset_breath();
                    }
                }

                match &self.sign_msg {
                    Some(message) => game.set_message(Some(
                        Message::new(message.to_string()).title("Adventurers".to_string()),
                    )),
                    None => game.set_message(None),
                };
            }
            _ => {}
        }

        //Sign_msg contains the string in the sign block, if block is not a sign, sign_msg is empty string
        if self.player.get_breath() == 0 {
            game.set_message(Some(
                Message::new(String::from("how did u die in a 2d pixel game lmao")).title("Adventurers".to_string()),
            ));
            self.game_state = 1;
        }
    }

    fn on_tick(&mut self, _game: &mut Game) {}
}

fn add_player_block(game: &mut Game, x: i32, y: i32, player_char: char) {
    let ch_new = game.get_screen_char(x, y).unwrap();
    let bg_colour_new = ch_new.style.unwrap().background_color;
    game.set_screen_char(
        x,
        y,
        Some(
            StyledCharacter::new(player_char)
                .style(GameStyle::new().background_color(bg_colour_new)),
        ),
    );
}

fn move_viewport(game: &mut Game, direction: Direction) {
    let new_viewport = match direction {
        Direction::Left => ViewportLocation {
            x: game.get_viewport().x - 1,
            y: game.get_viewport().y,
        },
        Direction::Right => ViewportLocation {
            x: game.get_viewport().x + 1,
            y: game.get_viewport().y,
        },
        Direction::Up => ViewportLocation {
            x: game.get_viewport().x,
            y: game.get_viewport().y - 1,
        },
        Direction::Down => ViewportLocation {
            x: game.get_viewport().x,
            y: game.get_viewport().y + 1,
        },
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

fn get_block(game_map: &HashMap<(i32, i32), Block>, (x, y): (i32, i32)) -> &Block {
    let block = match game_map.get(&(x, y)) {
        Some(block) => block,
        None => &Block::Empty,
    };
    block
}

fn get_background_color(game: &Game, x: i32, y: i32) -> GameColor {
    let colour = match game.get_screen_char(x, y) {
        Some(styled_char) => match styled_char.style {
            Some(style) => match style.background_color {
                Some(colour) => colour,
                None => GameColor::Black,
            },
            None => GameColor::Black,
        },
        None => GameColor::Black,
    };
    colour
}
