use dioxus::prelude::*;
use glam::Vec2;
use strum::IntoEnumIterator;

use crate::{components::{CARD_BORDER_RADIUS_RATIO, CARD_HEIGHT_RATIO, CardComponent, CardFrame, EMOJI_MAP, Emoji, Movement, SkinTrait, rem}, game::{ANIMATION_DURATION, AnimationAct, AnimationKey, Board, BoardPos, Card, DepotRole, GameStatus, NUM_DEPOTS, Skin, Suit, SuitCount, SuitCountExt}};

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
    // let game_status = GameStatus::Lost { danger: Suit::Carrot };

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

    let suit_count_width = 18f32;
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

    let boat_pos = board.boat_pos;
    let animate_boat = board.animate_boat;

    let boater_width = 10f32;
    let boater_y = 50.;
    let boater_shift = 0.;
    let boater_left_pos = Vec2::new(river_x - boater_shift, boater_y);
    let boater_right_pos = Vec2::new(river_x + river_width - boater_width + boater_shift, boater_y);

    let boater = (0..1).map(|_|{
        let mut translate = boater_right_pos - boater_left_pos;
        if boat_pos != DepotRole::Left { translate = -translate; }
        let pos = if boat_pos == DepotRole::Left {boater_left_pos} else {boater_right_pos};

        let transform = if boat_pos == DepotRole::Left {
            "transform: rotateY(180deg);"
        } else {""};

        rsx!{
            div {
                key: "{animation_key},boat",
                style: "--translateX: {rem(translate.x)}; --translateY: {rem(translate.y)};",
                
                div {
                    position: "absolute",
                    top: rem(pos.y),
                    left: rem(pos.x),
                    transform_style: "preserve-3d",
                    perspective: "333rem",
                    animation: if animate_boat {"{ANIMATION_DURATION.as_secs_f32()}s movement-boat"},

                    img {
                        style: "width: {rem(boater_width)}; {transform}",
                        src: EMOJI_MAP["🚣"]
                    }
                }
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

    let moving_card = |p1: Vec2, p2: Vec2, card: Card| rsx! {
        Movement {
            src_translate_vec: p1 - p2,
            CardComponent {
                position: p2,
                width: card_width,
                card: card,
                skin,
            }
        }
    };

    let anims = board.animation_acts.iter().enumerate().map(|(i, act)| {
        match act {
            AnimationAct::Move (cards, pos1, pos2) => {
                let mut pos1 = *pos1;
                let mut pos2 = *pos2;

                let nodes = cards.iter().map(move |card| {
                    let p1 = get_pos(pos1.depot_index, pos1.card_index);
                    let p2 = get_pos(pos2.depot_index, pos2.card_index);
                    let res = moving_card(p1, p2, *card);
                    pos1.card_index += 1;
                    pos2.card_index += 1;
                    res
                });

                rsx! {
                    Fragment {
                        key: "{animation_key},{i}", // needed to force remounts, so animations don't get "stale" and refuse to replay
                        {nodes}
                    }
                }
            },
        }
    });

    let is_won = game_status == GameStatus::Won;
    let lose_message = if let GameStatus::Lost { danger } = game_status {
        let pred = match danger {
            Suit::Fox => "?", // shouldn't happen
            Suit::Rabbit => "foxes",
            Suit::Carrot => "rabbits",
        };
        let prey = match danger {
            Suit::Fox => "Foxes",
            Suit::Rabbit => "Rabbits",
            Suit::Carrot => "Carrots",
        };

        rsx! {
            div {
                position: "absolute",
                top: rem(25.),
                left: rem(17.5),
                width: rem(59.),
                background_color: "#000",
                padding: rem(3.),
                color: "#fff",
                font_size: rem(7.),
                border_radius: rem(2.),
                text_align: "center",
                "GAME OVER",
                p {
                    font_size: rem(3.),
                    "Too many {pred}! {prey} were eaten…"
                }
            }
        }
    } else {rsx!{}};

    rsx! {
        div {
            position: "absolute",
            top: rem(position.y),
            left: rem(position.x),

            // {test_card},

            {river},
            {boater},
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

            {anims},

            if is_won {
                div {
                    position: "absolute",
                    top: rem(25.),
                    left: rem(17.5),
                    width: rem(59.),
                    background_color: "#505",
                    padding: rem(3.),
                    color: "#fff",
                    font_size: rem(7.),
                    border_radius: rem(2.),
                    text_align: "center",
                    "YOU WIN!",
                }
            } else {
                {lose_message}
            }
        }
    }
}