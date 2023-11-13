use std::default;

use iced::{
    application,
    widget::{
        button::{self},
        rule::{self},
        scrollable::{self},
        text::{self},
    },
    Background, Color,
};

#[derive(Clone, Copy, Debug, Default)]
pub struct Theme;

impl application::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _: &Self::Style) -> application::Appearance {
        application::Appearance {
            background_color: [1., 1., 1.].into(),
            text_color: [0.1, 0.1, 0.1].into(),
        }
    }
}
impl text::StyleSheet for Theme {
    type Style = ();
    fn appearance(&self, _: Self::Style) -> text::Appearance {
        text::Appearance::default()
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub enum ButtonStyle {
    #[default]
    Default,
    Contact,
}

impl button::StyleSheet for Theme {
    type Style = ButtonStyle;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        match style {
            ButtonStyle::Default => button::Appearance {
                border_radius: (15.).into(),
                background: Some(Background::Color([0.2, 0.2, 0.6, 0.85].into())),
                border_width: 0.85,
                text_color: [0., 0., 0.].into(),
                ..Default::default()
            },
            ButtonStyle::Contact => button::Appearance {
                border_width: 0.0,
                shadow_offset: Default::default(),
                background: Default::default(),
                border_radius: (0.).into(),
                border_color: Color::TRANSPARENT,
                text_color: [0., 0., 0.].into(),
                ..Default::default()
            },
        }
    }
}

impl scrollable::StyleSheet for Theme {
    type Style = ();

    fn active(&self, _: &Self::Style) -> scrollable::Scrollbar {
        scrollable::Scrollbar {
            background: None,
            border_radius: (0.).into(),
            border_width: 0.,
            border_color: Color::TRANSPARENT,
            scroller: scrollable::Scroller {
                color: [0., 0., 0.].into(),
                border_radius: (0.).into(),
                border_width: 0.,
                border_color: Color::TRANSPARENT,
            },
        }
    }

    fn hovered(&self, style: &Self::Style, _: bool) -> scrollable::Scrollbar {
        Self::active(&self, &style)
    }
}

impl rule::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _: &Self::Style) -> rule::Appearance {
        rule::Appearance {
            color: Color::BLACK,
            width: 0,
            radius: (0.).into(),
            fill_mode: rule::FillMode::Full,
        }
    }
}
