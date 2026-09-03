use async_std::stream::StreamExt;
use dioxus::prelude::*;
use glam::Vec2;

use crate::components::EMOJI_MAP;

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            id: "hero",
            class: "select-none",

            div {
                id: "preloaded-images",

                for asset in EMOJI_MAP.values() {
                    img {
                        src: *asset,
                        width: 1,
                        height: 1,
                    }
                }
            },
        }
    }
}