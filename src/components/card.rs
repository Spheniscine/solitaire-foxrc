use dioxus::prelude::*;
use glam::Vec2;

use crate::{components::rem, game::ColorMode};

pub trait SkinTrait<C>: PartialEq + Clone {
    fn get_color(&self, card: &C, mode: ColorMode) -> String;
    fn render_rank(&self, card: &C) -> Element;
    fn render_suit(&self, card: &C) -> Element;
    fn render_suit_text(&self, card: &C) -> Element;
}

pub const CARD_HEIGHT_RATIO: f32 = 13. / 12.;
pub const CARD_BORDER_RADIUS_RATIO: f32 = 1.5 / 12.;

#[component]
pub fn CardComponent<C: PartialEq + Clone + 'static, S: SkinTrait<C> + 'static>(
    position: Vec2,
    width: f32,
    card: C,
    skin: S,

    number_hint: Option<usize>,
    #[props(default)]
    onclick: EventHandler<MouseEvent>,
    #[props(default)]
    ondoubleclick: EventHandler<MouseEvent>,
) -> Element {
    let pt = width / 12.;
    let pt = |x: f32| {
        rem(x * pt)
    };

    rsx! {
        div {
            style: "place-items: center;",
            position: "absolute",
            top: rem(position.y),
            left: rem(position.x),
            background_color: "#fff",
            width: pt(11.),
            height: pt(12.),
            border: "{pt(0.25)} solid #000",
            border_radius: rem(width * CARD_BORDER_RADIUS_RATIO),
            display: "grid",
            grid_template_columns: "50% 50%",
            grid_template_rows: "50% 50%",
            font_size: pt(4.75), // slightly smaller than the 5 point set in most other games
            text_align: "center",
            padding: pt(0.25),
            color: skin.get_color(&card, ColorMode::Dark),

            onclick, ondoubleclick,

            div { display: "flex", align_items: "center", pointer_events: "none", {skin.render_rank(&card)}},
            div { display: "flex", align_items: "center", pointer_events: "none", {skin.render_suit(&card)}},
            div { display: "flex", align_items: "center", pointer_events: "none", {skin.render_suit(&card)}},
            div { display: "flex", align_items: "center", pointer_events: "none", {skin.render_rank(&card)}},
        }
    }
}

#[component]
pub fn CardFrame(
    position: Vec2,
    width: f32,
    hint: Option<Element>,
    #[props(default = "#aaa".to_string())] color: String,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let pt = width / 12.;
    let pt = |x: f32| {
        rem(x * pt)
    };
    rsx! {
        div {
            display: "flex",
            align_items: "center",
            justify_content: "center",
            position: "absolute",
            top: rem(position.y),
            left: rem(position.x),
            margin: pt(0.25), // frame must be slightly smaller than card to prevent peeking out in some platforms
            width: pt(10.),
            height: pt(11.),
            border: "{pt(0.5)} solid {color}",
            text_align: "center",
            color,
            border_radius: pt(1.5),
            font_size: pt(5.),
            padding: pt(0.25),
            onclick,

            if let Some(hint) = hint {
                div {
                    {hint},
                }
            }
        }
    }
}

#[component]
pub fn CardText<C: PartialEq + Clone + 'static, S: SkinTrait<C> + 'static>(card: C, skin: S, color_mode: ColorMode) -> Element {
    rsx! {
        span {
            font_size: "1.2em",
            white_space: "nowrap",
            line_height: 1.2,
            color: skin.get_color(&card, color_mode),
            {skin.render_rank(&card)},
            span {display: "inline-block", min_width: "0.1em"},
            {skin.render_suit_text(&card)},
        }
    }
}