use serde::{Deserialize, Serialize};
use strum_macros::{EnumIter, FromRepr};

use crate::game::Suit;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, EnumIter, strum_macros::Display, Default)]
pub enum RankSkin {
    #[default]
    Numbers,
    Traditional,
}

impl RankSkin {
    pub fn rank_text(self, rank: u8) -> String {
        match self {
            RankSkin::Numbers => rank.to_string(),
            RankSkin::Traditional => {
                match rank {
                    1 => String::from("A"),
                    11 => String::from("J"),
                    12 => String::from("Q"),
                    13 => String::from("K"),
                    _ => rank.to_string(),
                }
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, EnumIter, strum_macros::Display, Default, FromRepr)]
#[repr(u8)]
pub enum SuitSkin {
    #[default]
    Animals,
}

impl SuitSkin {
    pub fn suit_symbol(self, suit: Suit) -> &'static str {
        match self {
            SuitSkin::Animals => {
                match suit {
                    Suit::Fox => "🦊",
                    Suit::Rabbit => "🐰",
                    Suit::Carrot => "🥕",
                }
            },
        }
    }

    pub fn font(self) -> &'static str {
        match self {
            SuitSkin::Animals => "'Noto Color Emoji'",
        }
    }
}

const COLOR_GREEN: [&str; 2] = ["#062", "#00ff55"];
const COLOR_RED: [&str; 2] = ["#f00", "#ff8888"];
const COLOR_BLUE: [&str; 2] = ["#00d", "#aaaaff"];

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, EnumIter, strum_macros::Display, Default, FromRepr)]
#[repr(u8)]
pub enum ColorMode {
    #[default] Dark, Light
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, EnumIter, strum_macros::Display, Default, FromRepr)]
#[repr(u8)]
pub enum ColorSkin {
    #[default]
    ThreeColor,
}

impl ColorSkin {
    pub fn color(self, suit: Suit, mode: ColorMode) -> &'static str {
        let res = match self {
            ColorSkin::ThreeColor => {
                match suit {
                    Suit::Fox => COLOR_RED,
                    Suit::Rabbit => COLOR_BLUE,
                    Suit::Carrot => COLOR_GREEN,
                }
            },
        };
        res[mode as usize]
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, Default)]
pub struct Skin {
    pub ranks: RankSkin,
    #[serde(skip)]
    pub suits: SuitSkin,
     #[serde(skip)]
    pub colors: ColorSkin,
}