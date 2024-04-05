use bevy::prelude::{BackgroundColor, Color, FromWorld, Resource, World};

/// Resource containing UI theme settings
#[derive(Resource)]
pub struct UiTheme {
    pub root: BackgroundColor,
    pub border: BackgroundColor,
    pub menu: BackgroundColor,
    pub button: BackgroundColor,
    pub button_hovered: BackgroundColor,
    pub button_pressed: BackgroundColor,
    pub button_text: Color,
    pub button_text_hovered: Color,
}

impl FromWorld for UiTheme {
    fn from_world(_: &mut World) -> Self {
        UiTheme {
            root: Color::NONE.into(),
            border: Color::rgb(0.65, 0.65, 0.65).into(),
            menu: Color::rgb(0.15, 0.15, 0.15).into(),
            button: Color::rgb(0.15, 0.15, 0.15).into(),
            button_hovered: Color::rgb(0.35, 0.75, 0.35).into(),
            button_pressed: Color::rgb(0.35, 0.75, 0.35).into(),
            button_text: Color::WHITE,
            button_text_hovered: Color::rgba(1.0, 1.0, 1.0, 0.5).into(),
        }
    }
}