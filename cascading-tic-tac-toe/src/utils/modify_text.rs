use bevy::asset::Handle;
use bevy::prelude::{Children, Color, Font, Query, Text};

pub fn modify_text(
    children: &Children,
    cell_text_query: &mut Query<&mut Text>,
    text: String,
    style: (Option<Handle<Font>>, Option<f32>, Option<Color>)
) {
    for child in children.iter() {
        if let Ok(mut cell_text) = cell_text_query.get_mut(*child) {
            cell_text.sections[0].value = text.clone();
            let (ref font, font_size, color) = style;
            cell_text.sections[0].style.font = match font.clone() {
                Some(font) => font,
                None => cell_text.sections[0].clone().style.font
            };
            cell_text.sections[0].style.font_size = match font_size.clone() {
                Some(font_size) => font_size,
                None => cell_text.sections[0].clone().style.font_size
            };
            cell_text.sections[0].style.color = match color.clone() {
                Some(color) => color,
                None => cell_text.sections[0].clone().style.color
            };
        }
    }
}