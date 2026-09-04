use dioxus::prelude::*;

use crate::{components::{CardText, VIDEO_GAMEPLAY, rem}, game::{Card, GameState, ScreenState, Suit}};

#[component]
fn Emph(children: Element) -> Element {
    rsx! {
        strong {
            color: "#ff0",
            {children}
        }
    }
}

#[component]
fn OhNo(children: Element) -> Element {
    rsx! {
        strong {
            color: "#f88",
            {children}
        }
    }
}

#[component]
pub fn Help(game_state: Signal<GameState>) -> Element {
    let st = game_state.read();
    let skin = st.skin;

    let stack_example = || {
        let mut ite = [
            Card { rank: 12, suit: Suit::Fox },
            Card { rank: 11, suit: Suit::Rabbit },
            Card { rank: 7, suit: Suit::Fox },
            Card { rank: 5, suit: Suit::Rabbit },
            Card { rank: 4, suit: Suit::Carrot },
        ].into_iter().map(|card| {
            rsx! {
                CardText { 
                    card, skin, color_mode: crate::game::ColorMode::Light,
                }
            }
        });


        let last = ite.next().unwrap();
        rsx! {
            {ite.next().unwrap()},
            for x in ite { "–", {x} },
            " can be placed on ", {last}
        }
    };

    rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center; font-size: 4.5rem; color: #fff; padding: 4rem;",
            class: "help",

            div {
                text_align: "left",

                p {
                    margin_top: "0",
                    "The deck has 39 cards: 13 ranks in each of 3 special suits: Foxes, Rabbits, and Carrots."
                }

                p {
                    "Cards stack by ", Emph {"decreasing rank (gaps allowed)"}, " and ", Emph {"unlike suit"},
                    ". Such stacks of any size can be moved as a unit. (e.g. ",{stack_example()}")"
                }

                p {
                    "You may only pick up card stacks from the side of the " Emph {"river"} " you’re currently on. The "
                    Emph {"boat"} " indicates the side you’re on. When you move a card stack across the river, the boat will
                    also move across."
                }

                p {
                    OhNo {"You lose"} " if there are ever ≥2 more Foxes than Rabbits, and ≥1 Rabbit, on one side of the river!"
                }

                p {
                    OhNo {"You lose"} " if there are ever ≥2 more Rabbits than Carrots, and ≥1 Carrot, on one side of the river!"
                }

                p {
                    "To ",Emph{"win the game"},", move all cards to the right side of the river safely."
                }
            }

            div {
                position: "absolute",
                bottom: rem(2.),
                width: "92rem",
                display: "flex",
                justify_content: "center",

                a {
                    href: VIDEO_GAMEPLAY,
                    target: "_blank",
                    text_decoration: "none",
                    margin_right: rem(4.),
                    div {
                        width: rem(30.),
                        position: "relative",
                        class: "game-button",
                        "Example video"
                    }
                }

                div {
                    width: rem(30.),
                    position: "relative",
                    class: "game-button",
                    onclick: move |_| game_state.write().screen_state = ScreenState::Game,
                    "Back to game"
                }
            }
        }
    }
}