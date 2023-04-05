use std::fmt::{self, Formatter};

use serde::Deserialize;
use termgame::{GameColor, GameStyle, StyledCharacter};
#[derive(PartialEq, Deserialize)]
pub enum Block {
    Barrier,
    Water,
    Grass,
    Sand,
    Rock,
    Cinderblock,
    Flowerbush,
    Empty,
    Sign(String),
    Object(char),
}

pub trait SignText {
    fn get_sign_text(&self) -> String;
}

impl SignText for Block {
    fn get_sign_text(&self) -> String {
        let str = match self {
            Block::Sign(str) => str,
            _ => "",
        };
        //let Block::Sign(str) = Block::Sign(String);
        str.to_string()
    }
}
pub trait BlockColour {
    fn get_colour(&self) -> Option<StyledCharacter>;
}

impl BlockColour for Block {
    fn get_colour(&self) -> Option<StyledCharacter> {
        let ret = match self {
            Block::Barrier => Some(
                StyledCharacter::new(' ')
                    .style(GameStyle::new().background_color(Some(GameColor::White))),
            ),
            Block::Water => Some(
                StyledCharacter::new(' ')
                    .style(GameStyle::new().background_color(Some(GameColor::Blue))),
            ),
            Block::Grass => Some(
                StyledCharacter::new(' ')
                    .style(GameStyle::new().background_color(Some(GameColor::Green))),
            ),
            Block::Sand => Some(
                StyledCharacter::new(' ')
                    .style(GameStyle::new().background_color(Some(GameColor::Yellow))),
            ),
            Block::Rock => Some(
                StyledCharacter::new(' ')
                    .style(GameStyle::new().background_color(Some(GameColor::Yellow))),
            ),
            Block::Cinderblock => Some(
                StyledCharacter::new(' ')
                    .style(GameStyle::new().background_color(Some(GameColor::LightRed))),
            ),
            Block::Flowerbush => Some(
                StyledCharacter::new(' ')
                    .style(GameStyle::new().background_color(Some(GameColor::Magenta))),
            ),
            Block::Sign(_) => Some(
                StyledCharacter::new('💬')
                    .style(GameStyle::new().background_color(Some(GameColor::Black))),
            ),
            Block::Object(char) => Some(
                StyledCharacter::new(char.to_owned())
                    .style(GameStyle::new().background_color(Some(GameColor::Black))),
            ),
            Block::Empty => Some(
                StyledCharacter::new(' ')
                    .style(GameStyle::new().background_color(Some(GameColor::Black))),
            ),
        };
        ret
    }
}
pub trait Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result;
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ret = match self {
            Block::Barrier => write!(f, "Barrier"),
            Block::Water => write!(f, "block of water"),
            Block::Grass => write!(f, "block of grass"),
            Block::Sand => write!(f, "block of sand"),
            Block::Rock => write!(f, "block of rock"),
            Block::Cinderblock => write!(f, "block of cinderblock"),
            Block::Flowerbush => write!(f, "block of flowerbush"),
            Block::Empty => write!(f, "??"),
            Block::Sign(_) => write!(f, "sign"),
            Block::Object(char) => write!(f, "'{}' object", char),
        };
        ret
    }
}
