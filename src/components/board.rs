use dioxus::prelude::*;
use glam::Vec2;
use strum::IntoEnumIterator;

use crate::{components::{CARD_BORDER_RADIUS_RATIO, CARD_HEIGHT_RATIO, CardComponent, CardFrame, Emoji, SkinTrait, rem}, game::{AnimationKey, Board, BoardPos, Card, DepotRole, GameStatus, NUM_DEPOTS, Skin, Suit, SuitCount, SuitCountExt}};

#[component]
fn SuitCountComponent(
    position: Vec2,
    width: f32,
    skin: Skin,
    suit_count: SuitCount
) -> Element {
    let dangers = suit_count.find_dangers();

    rsx! {
        div {
            position: "absolute",
            top: rem(position.y),
            left: rem(position.x),
            font_size: rem(3.5),
            width: rem(width),
            display: "grid",
            grid_template_columns: "auto auto auto",
            text_align: "center",

            for suit in Suit::iter() {
                div {
                    position: "relative",
                    if dangers.contains(suit) {
                        div {
                            position: "absolute", top: 0, left: 0,
                            width: "100%",
                            class: "blink",
                            Emoji {
                                text: "⚠️"
                            }
                        }
                    }

                    {skin.render_suit(&Card { rank: 1, suit })}
                }
            }

            for suit in Suit::iter() {
                div {
                    color: if dangers.contains(suit) {"#ff0"} else {"#fff"},
                    "{suit_count[suit]}",
                }
            }
        }
    }
}

#[component]
pub fn BoardComponent(
    position: Vec2,
    board: Board,
    skin: Skin,
    #[props(default)]
    onclick: EventHandler<BoardPos>,
    #[props(default)]
    animation_key: AnimationKey,
    #[props(default)]
    game_status: GameStatus,
) -> Element {
    let card_width = 11f32;
    let card_height = card_width * CARD_HEIGHT_RATIO;
    let spacer_x = 1f32;
    let start_x = 2f32;

    let half_tableau_width = 3. * card_width + 2. * spacer_x;
    let start_y = 2f32;
    let tableau_y = start_y + 10f32;
    let right_tableau_x = 100. - start_x - half_tableau_width;
    // let column_card_offset = Vec2::new(0., card_height / 2.);
    let column_card_offset = Vec2::new(0., 5.825);

    let get_pos = |depot: usize, ord: usize| {
        let (role, index) = DepotRole::role_and_subindex(depot).unwrap();
        let left = match role {
            DepotRole::Left => start_x,
            DepotRole::Right => right_tableau_x,
        };

        Vec2::new(
            left + (card_width + spacer_x) * index as f32, tableau_y
        ) + column_card_offset * ord as f32
    };

    let get_hint = |_depot: usize| {
        Some(rsx!{})
    };

    let selected_height = if let Some(BoardPos { depot_index, card_index }) = board.selected {
        let d = board.depots[depot_index].len() - card_index - 1;

        card_height + column_card_offset.y * d as f32
    } else {0.};

    let river_width = card_width * 2.;
    let river_x = 50. - river_width / 2.;
    let river_height = 100f32 * 16. / 9. - 20. - start_y;

    let river = rsx! {
        div {
            position: "absolute",
            top: rem(start_y),
            left: rem(river_x),
            width: rem(river_width),
            height: rem(river_height),
            background_color: "#4276A9",
        }
    };

    let suit_counts = board.predicted_suit_counts();
    // // test
    // let mut suit_counts = suit_counts;
    // suit_counts[DepotRole::Left][Suit::Carrot] = 0;

    let suit_count_width = 17f32;
    let suit_count_components = DepotRole::iter().map(|role| {
        let pos_x = get_pos(role.id(1), 0).x + (card_width - suit_count_width) / 2.;
        rsx! {
            SuitCountComponent { 
                position: Vec2::new(pos_x, start_y),
                width: suit_count_width,
                skin,
                suit_count: suit_counts[role],
            }
        }
    });

    // let test_card = rsx! {
    //     CardComponent { 
    //         position: get_pos(0, 24),
    //         width: card_width,
    //         card: Card { suit: Suit::Fox, rank: 1 },
    //         // number_hint: if !is_face_up(depot) {i + 1},
    //         skin,
    //     }
    // };

    rsx! {
        div {
            position: "absolute",
            top: rem(position.y),
            left: rem(position.x),

            // {test_card},

            {river},
            {suit_count_components},

            for depot in 0..NUM_DEPOTS {
                if let Some(hint) = get_hint(depot) {
                    CardFrame { 
                        position: get_pos(depot, 0),
                        width: card_width,
                        hint,
                        onclick: move |_| {
                            onclick.call(BoardPos::new(depot, !0))
                        },
                    }
                }

                for i in 0..board.depots[depot].len() {
                    if board.selected == Some(BoardPos::new(depot, i)) {
                        div {
                            position: "absolute",
                            top: rem(get_pos(depot, i).y),
                            left: rem(get_pos(depot, i).x),
                            width: rem(card_width),
                            height: rem(selected_height),
                            background_color: "#ff0",
                            border_radius: rem(card_width * CARD_BORDER_RADIUS_RATIO),
                            class: "selected-halo",
                        }
                    }

                    CardComponent { 
                        position: get_pos(depot, i),
                        width: card_width,
                        card: board.depots[depot][i],
                        // number_hint: if !is_face_up(depot) {i + 1},
                        skin,
                        onclick: move |_| {
                            onclick.call(BoardPos::new(depot, i))
                        },
                    }
                }
            }
        }
    }
}