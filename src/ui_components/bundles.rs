use bevy::prelude::{AlignItems, AssetServer, ButtonBundle, Color, default, ImageBundle, JustifyContent, PositionType, Res, Style, Text, TextBundle, TextStyle, UiImage, UiRect, Val};
use bevy::ui::BackgroundColor;

/// ImageBundle for creating game icon
pub fn image_bundle(
    image: UiImage
) -> ImageBundle {
    ImageBundle {
        style: Style {
            width: Val::Px(30.0),
            position_type: PositionType::Absolute,
            left: Val::Px(10.0),
            ..default()
        },
        image,
        ..default()
    }
}

/// Bundle for spawning UI buttons
pub fn button_bundle(
    style: (Val, Val, Option<UiRect>, JustifyContent, AlignItems),
    background_color: BackgroundColor
) -> ButtonBundle {

    let (width, height, margin, justify_content, align_items) = style;

    ButtonBundle {
        style: Style {
            width,
            height,
            margin: margin.unwrap_or(UiRect::default()),
            justify_content,
            align_items,
            ..Default::default()
        },
        background_color,
        ..Default::default()
    }
}

/// TextBundle for spawning text on screen
pub fn text_bundle(
    label: &str,
    asset_server: &Res<AssetServer>,
    style: (f32, Color)
) -> TextBundle {

    let (font_size, color) = style;

    TextBundle {
        text: Text::from_section(
            label,
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size,
                color,
            },
        ),
        ..Default::default()
    }
}