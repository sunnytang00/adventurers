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

        //Initalise player char
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
        //Relative position is a field in the player struct to help with panning the screen
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
        let bg_colour = get_background_color(game, self.player.x, self.player.y);

        let (width, (height, _)) = game.screen_size();
        let (term_width, term_height) = (width - 2, height - 2);

        match event.into() {
            SimpleEvent::Just(KeyCode::Char('q')) => {
                game.set_message(Some(
                    Message::new(self.quest.to_string()).title("Quest Status".to_string()),
                ));
            }

            SimpleEvent::Just(KeyCode::Char('r')) => {
                self.quest.reset_quest();
                game.set_message(None);
            }

            SimpleEvent::Just(KeyCode::Left) => {
                do_everything_related_to_move(
                    game,
                    self,
                    Direction::Left,
                    bg_colour,
                    term_width,
                    term_height,
                );
            }
            SimpleEvent::Just(KeyCode::Right) => {
                do_everything_related_to_move(
                    game,
                    self,
                    Direction::Right,
                    bg_colour,
                    term_width,
                    term_height,
                );
            }
            SimpleEvent::Just(KeyCode::Up) => {
                do_everything_related_to_move(
                    game,
                    self,
                    Direction::Up,
                    bg_colour,
                    term_width,
                    term_height,
                );
            }
            SimpleEvent::Just(KeyCode::Down) => {
                do_everything_related_to_move(
                    game,
                    self,
                    Direction::Down,
                    bg_colour,
                    term_width,
                    term_height,
                );
            }
            _ => {}
        }

        //Sign_msg contains the string in the sign block, if block is not a sign, sign_msg is empty string
        if self.player.get_breath() == 0 {
            game.set_message(Some(
                Message::new(String::from("how did u die in a 2d pixel game lmao"))
                    .title("Adventurers".to_string()),
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

fn do_everything_related_to_move(
    game: &mut Game,
    mygame: &mut MyGame,
    direction: Direction,
    bg_colour: GameColor,
    term_width: u16,
    term_height: u16,
) {
    //Get x, y of block we are trying to move to
    let (x, y) = get_next_position(mygame.player.x, mygame.player.y, direction);

    //If map is missing fields, before we move, create an entry in hashmap and termgame map
    if game.get_screen_char(x, y).is_none() {
        mygame.game_map.insert((x, y), Block::Empty);
        game.set_screen_char(x, y, create_empty_block(GameColor::Black));
    }

    //Only allow moving if block we are moving to is not barrier
    if get_block(&mygame.game_map, (x, y)) != &Block::Barrier {
        mygame
            .quest
            .update_quests(get_block(&mygame.game_map, (x, y)));
        //Delete old character - replace the sign if we moved onto it
        if mygame.sign_msg.is_some() {
            game.set_screen_char(
                mygame.player.x,
                mygame.player.y,
                Some(
                    StyledCharacter::new('💬')
                        .style(GameStyle::new().background_color(Some(GameColor::Black))),
                ),
            );
            mygame.sign_msg = None;
        } else {
            game.set_screen_char(
                mygame.player.x,
                mygame.player.y,
                create_empty_block(bg_colour),
            );
        }

        //If player is going to move onto a sign block, save info about sign block

        match game.get_screen_char(x, y) {
            Some(styled_c) => {
                if styled_c.c == '💬' {
                    let sign_txt = match mygame.game_map.get(&(x, y)) {
                        Some(x) => match x {
                            Block::Sign(_) => x.get_sign_text(),
                            _ => panic!(),
                        },
                        None => panic!(),
                    };

                    mygame.sign_msg = Some(sign_txt);
                }
            }
            None => panic!(),
        };

        //Moving viewport and updating relative player coords if needed
        match direction {
            Direction::Left => {
                if i32::from(term_width / 2) + mygame.player.rel_x <= 2 {
                    move_viewport(game, direction);
                } else {
                    mygame.player.move_dir_rel(direction);
                }
            }
            Direction::Right => {
                if i32::from(term_width / 2) - mygame.player.rel_y <= 2 {
                    move_viewport(game, Direction::Right);
                } else {
                    mygame.player.move_dir_rel(Direction::Right);
                }
            }
            Direction::Up => {
                if i32::from(term_height / 2) - mygame.player.rel_y <= 2 {
                    move_viewport(game, Direction::Up);
                } else {
                    mygame.player.move_dir_rel(Direction::Up);
                }
            }
            Direction::Down => {
                if i32::from(term_height / 2) + mygame.player.rel_y <= 2 {
                    move_viewport(game, Direction::Down);
                } else {
                    mygame.player.move_dir_rel(Direction::Down);
                }
            }
        };
        mygame.player.move_dir(direction);

        //Create new player at new position
        add_player_block(game, mygame.player.x, mygame.player.y, mygame.player.char);

        if get_block(&mygame.game_map, (mygame.player.x, mygame.player.y)) == &Block::Water {
            mygame.player.decrease_breath();
        } else {
            mygame.player.reset_breath();
        }
    }
    match &mygame.sign_msg {
        Some(message) => game.set_message(Some(
            Message::new(message.to_string()).title("Adventurers".to_string()),
        )),
        None => game.set_message(None),
    };
}
