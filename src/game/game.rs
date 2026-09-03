use std::time::Duration;

use enum_map::EnumMap;
use enumset::EnumSet;
use extend::ext;
use rand::{Rng, seq::SliceRandom};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

use crate::game::{Board, BoardPos, Card, DECK_SIZE, DepotRole, RANKS, Skin, Suit};

use super::AnimationAct;

pub type SuitCount = EnumMap<Suit, u8>;
pub type SuitCounts = EnumMap<DepotRole, SuitCount>;
pub type Dangers = EnumSet<Suit>;

#[ext]
pub impl SuitCount {
    fn find_dangers(self) -> Dangers {
        let pairs = [
            [Suit::Fox, Suit::Rabbit],
            [Suit::Rabbit, Suit::Carrot],
        ];

        pairs.iter().filter(|&&[pred, prey]| {
            self[prey] > 0 && self[pred] >= self[prey] + 2
        }).map(|x| x[1]).collect()
    }

    // fn has_danger(self) -> bool {
    //     !self.find_dangers().is_empty()
    // }
}

#[ext]
pub impl SuitCounts {
    fn find_first_danger(self) -> Option<Suit> {
        for count in self.values() {
            for danger in count.find_dangers() {
                return Some(danger);
            }
        }
        None
    }
}

impl Board {
    fn actual_suit_counts(&self) -> SuitCounts {
        SuitCounts::from_fn(|role| {
            let mut count = SuitCount::default();

            for &card in role.range().flat_map(|i| &self.depots[i]) {
                count[card.suit] += 1;
            }

            count
        })
    }

    fn predicted_suit_counts(&self) -> SuitCounts {
        let mut counts = self.actual_suit_counts();

        if let Some(pos) = self.selected {
            let depot_index = pos.depot_index;
            let role = DepotRole::role(depot_index).unwrap();
            let card_index = pos.card_index;

            for &card in self.depots[depot_index][card_index..].iter() {
                counts[role][card.suit] -= 1;
                counts[!role][card.suit] += 1;
            }
        }

        for act in &self.animation_acts {
            match act {
                AnimationAct::Move(cards, _pos1, pos2) => {
                    let role = DepotRole::role(pos2.depot_index).unwrap();
                    for &card in cards {
                        counts[role][card.suit] += 1;
                    }
                },
            }
        }

        counts
    }
}

pub const ANIMATION_DURATION: Duration = Duration::from_millis(200);
pub type AnimationKey = u16;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct ActionRecord {
    pos1: BoardPos, pos2: BoardPos,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ScreenState {
    #[default] Game, 
    Settings, Help,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct GameState {
    pub board: Board,
    pub deal: Vec<Card>,
    #[serde(skip)]
    pub animation_key: AnimationKey, // used for syncing and to provide animator components with cycling keys
    pub history: Vec<ActionRecord>,
    pub undo_stack: Vec<usize>,
    pub already_won: bool,
    pub num_wins: i32,

    pub screen_state: ScreenState,

    pub allow_undo: bool,
    pub skin: Skin,
}

impl GameState {
    pub fn new_deal(rng: &mut impl Rng) -> Vec<Card> {
        let mut deck = Vec::with_capacity(DECK_SIZE);
        for rank in RANKS {
            for suit in Suit::iter() {
                deck.push(Card { rank, suit });
            }
        }

        deck.shuffle(rng);
        deck
    }

    pub fn init() -> Self {
        let mut res = Self {
            board: Board::empty(),
            deal: vec![],
            animation_key: 0,
            history: vec![],
            undo_stack: vec![],
            already_won: false,
            num_wins: 0,
            screen_state: ScreenState::Game,
            allow_undo: true,
            skin: Skin::default(),
        };

        res.new_game();
        res
    }

    pub fn new_game(&mut self) {
        let deal = Self::new_deal(&mut rand::rng());
        self.board = Board::from_deal(&deal);
        self.deal = deal;
        self.history.clear();
        self.undo_stack.clear();
        self.already_won = false;
        // LocalStorage.save_game_state(&self);
    }
}