use termgame::{
    run_game, Controller, Game, GameEvent, GameSettings, KeyCode, SimpleEvent, StyledCharacter, GameStyle, GameColor,
};
// use adventurers::{player::Player, utils::*};
use crate::player::{Player, Movement};
pub struct MyGame {
    pub player: Player,
}

impl Controller for MyGame {
    fn on_start(&mut self, game: &mut Game) {
        game.set_screen_char(self.player.x, self.player.y, Some(StyledCharacter::new(self.player.char)));
        game.set_screen_char(3, 4, Some(StyledCharacter::new(' ').style(GameStyle::new().background_color(Some(GameColor::Cyan)))));
    }

    fn on_event(&mut self, game: &mut Game, event: GameEvent) {
        match event.into() {
            SimpleEvent::Just(KeyCode::Left) => {
                game.set_screen_char(self.player.x, self.player.y, None);
                self.player.moveLeft();
                game.set_screen_char(self.player.x, self.player.y, Some(StyledCharacter::new(self.player.char)))
            },
            SimpleEvent::Just(KeyCode::Right) => {
                game.set_screen_char(self.player.x, self.player.y, None);
                self.player.moveRight();
                game.set_screen_char(self.player.x, self.player.y, Some(StyledCharacter::new(self.player.char)))
            },
            SimpleEvent::Just(KeyCode::Up) => {
                game.set_screen_char(self.player.x, self.player.y, None);
                self.player.moveUp();
                game.set_screen_char(self.player.x, self.player.y, Some(StyledCharacter::new(self.player.char)))
            },
            SimpleEvent::Just(KeyCode::Down) => {
                game.set_screen_char(self.player.x, self.player.y, None);
                self.player.moveDown();
                game.set_screen_char(self.player.x, self.player.y, Some(StyledCharacter::new(self.player.char)))
            },
            SimpleEvent::Just(KeyCode::Char(ch)) => {
                game.set_screen_char(3, 3, Some(StyledCharacter::new(ch)))
            }
            _ => {}
        }
    }

    fn on_tick(&mut self, _game: &mut Game) {}
}