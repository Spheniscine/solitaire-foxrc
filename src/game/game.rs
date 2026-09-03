use enum_map::EnumMap;
use enumset::EnumSet;
use extend::ext;

use crate::game::{Board, DepotRole, Suit};

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