use std::io::Chain;

use serde::Deserialize;
use termgame::{GameColor, Game, GameStyle, StyledCharacter};
use tui::style::Color;
#[derive(Deserialize)]
pub enum Block {
    Barrier,
    Water,
    Grass,
    Sand,
    Rock,
    Cinderblock,
    Flowerbush,
    Sign(String),
    Object(char),
}

pub trait BlockColour{
    fn get_colour(&self) -> Option<StyledCharacter>;
}

impl BlockColour for Block {
    fn get_colour(&self) -> Option<StyledCharacter> {
        let ret = match self {
            Block::Barrier => Some(StyledCharacter::new(' ').style(GameStyle::new().background_color(Some(GameColor::White)))),
            Block::Water => Some(StyledCharacter::new(' ').style(GameStyle::new().background_color(Some(GameColor::Blue)))),
            Block::Grass => Some(StyledCharacter::new(' ').style(GameStyle::new().background_color(Some(GameColor::Green)))),
            Block::Sand => Some(StyledCharacter::new(' ').style(GameStyle::new().background_color(Some(GameColor::Yellow)))),
            Block::Rock => Some(StyledCharacter::new(' ').style(GameStyle::new().background_color(Some(GameColor::Yellow)))),
            Block::Cinderblock => Some(StyledCharacter::new(' ').style(GameStyle::new().background_color(Some(GameColor::LightRed)))),
            Block::Flowerbush => Some(StyledCharacter::new(' ').style(GameStyle::new().background_color(Some(GameColor::Magenta)))),
            Block::Sign(String) => Some(StyledCharacter::new(' ').style(GameStyle::new().background_color(Some(GameColor::Black)))),
            Block::Object(char) => Some(StyledCharacter::new(char.to_owned()).style(GameStyle::new().background_color(Some(GameColor::Black)))),
        };
        ret
    }
}