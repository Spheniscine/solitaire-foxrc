use std::time::Duration;

use enum_map::EnumMap;
use enumset::EnumSet;
use extend::ext;
use rand::{Rng, seq::SliceRandom};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

use crate::{components::LocalStorage, game::{Board, BoardPos, Card, DECK_SIZE, DepotRole, NUM_RANKS, RANKS, SettingsState, Skin, Suit}};

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
    pub fn actual_suit_counts(&self) -> SuitCounts {
        SuitCounts::from_fn(|role| {
            let mut count = SuitCount::default();

            for &card in role.range().flat_map(|i| &self.depots[i]) {
                count[card.suit] += 1;
            }

            count
        })
    }

    pub fn predicted_suit_counts(&self) -> SuitCounts {
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum GameStatus {
    #[default]
    Ongoing,
    Won,
    Lost { danger: Suit },
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
        LocalStorage.save_game_state(&self);
    }

    pub fn is_busy(&self) -> bool {
        self.is_acting()
    }

    pub fn is_acting(&self) -> bool {
        !self.board.animation_acts.is_empty()
    }

    pub fn advance_animations(&mut self, key: AnimationKey) {
        if key != self.animation_key { return; }
        self.animation_key = self.animation_key.wrapping_add(1);
        
        self.board.advance_actions();

        if self.is_won() {
            if !self.already_won {
                self.num_wins += 1;
                self.already_won = true;
            }
        } else {
            // self.check_auto_moves();
        }

        if !self.is_busy() { LocalStorage.save_game_state(&self); }
    }

    pub fn game_status(&self) -> GameStatus {
        if self.is_busy() { GameStatus::Ongoing }
        else if DepotRole::Right.range().all(|i| self.board.depots[i].len() == NUM_RANKS) {
            GameStatus::Won
        } else if let Some(danger) = self.board.actual_suit_counts().find_first_danger() {
            GameStatus::Lost { danger }
        } else {
            GameStatus::Ongoing
        }
    }

    pub fn is_won(&self) -> bool {
        self.game_status() == GameStatus::Won
    }

    pub fn is_over(&self) -> bool {
        self.game_status() != GameStatus::Ongoing
    }

    pub fn onclick(&mut self, pos: BoardPos) {
        if self.is_busy() { return; }
        if self.is_over() { return; }

        if let Some(src) = self.board.selected {
            if pos == src { 
                self.board.selected = None; 
                return;
            }
            if src.depot_index == pos.depot_index && self.can_select(pos) {
                self.board.selected = Some(pos);
                return;
            }

            let dest = BoardPos::new(pos.depot_index, pos.card_index.wrapping_add(1));
            if !self.can_move(src, dest) { return; }
            self.undo_stack.push(self.history.len());
            self.do_move_raw(src, dest);
        } else {
            if self.can_select(pos) {
                self.board.selected = Some(pos);
            }
        }
    }

    pub fn can_select(&self, pos: BoardPos) -> bool {
        let depot = pos.depot_index;
        let ord = pos.card_index;

        let Some(role) = DepotRole::role(depot) else { return false };
        if role != self.board.boat_pos { return false };

        if ord >= self.board.depots[depot].len() {
            return false;
        }
        let slice = &self.board.depots[depot][ord..];
        slice.windows(2).all(|w| self.can_stack(w[0], w[1]))
    }

    pub fn can_stack(&self, back: Card, front: Card) -> bool {
        back.suit != front.suit && front.rank < back.rank
    }

    pub fn can_move(&self, pos1: BoardPos, pos2: BoardPos) -> bool {
        if pos1.depot_index == pos2.depot_index { return false; }
        let depot1 = &self.board.depots[pos1.depot_index];
        let depot2 = &self.board.depots[pos2.depot_index];
        // let num_moved = depot1.len() - pos1.card_index;
        if pos2.card_index != depot2.len() { return false; }

        let card = depot1[pos1.card_index];
        depot2.last().is_none_or(|&c| self.can_stack(c, card))
    }

    fn do_move_raw(&mut self, pos1: BoardPos, pos2: BoardPos) {
        self.board.do_move(pos1, pos2);
        self.history.push(ActionRecord { pos1, pos2 })
    }

    pub fn undo_possible(&self) -> bool {
        self.allow_undo && !self.undo_stack.is_empty()
    }

    pub fn undo(&mut self) {
        if self.is_busy() || !self.undo_possible() { return; }
        let Some(target_len) = self.undo_stack.pop() else {return};
        while self.history.len() > target_len {
            let rec = self.history.pop().unwrap();
            self.board.do_move(rec.pos2, rec.pos1);
            self.board.advance_actions(); // no animation, as repeated card moves on same card causes problems
        }
        LocalStorage.save_game_state(&self);
    }

    pub fn restart(&mut self) {
        if self.history.is_empty() || !self.undo_possible() { return; }
        self.board = Board::from_deal(&self.deal);
        self.history.clear();
        self.undo_stack.clear();

        if !self.is_busy() { LocalStorage.save_game_state(&self); }
    }

    pub fn new_settings_state(&self) -> SettingsState {
        SettingsState {
            allow_undo: self.allow_undo,
            skin: self.skin,
        }
    }

    pub fn apply_settings(&mut self, settings: &SettingsState){
        self.allow_undo = settings.allow_undo;
        self.skin = settings.skin;
        LocalStorage.save_game_state(&self);
    }
}