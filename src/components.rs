use crate::CellState;
use bevy::ecs::component::Component;
use bevy::prelude::{Entity, NextState, ResMut, Resource, States};

#[derive(Component, Clone)]
pub struct TicTacToeCell {
    pub cell_id: u32,
    pub state: CellState,
}

#[derive(Debug)]
pub struct StateWrapper<'w, T: States> {
    pub current: T,
    pub next: ResMut<'w, NextState<T>>,
}

#[derive(Resource)]
pub struct MainCamera {
    pub id: Option<Entity>
}

#[derive(Component)]
pub struct OnMainMenuScreen;

#[derive(Component)]
pub struct OnSettingsMenuScreen;

#[derive(Component)]
pub struct SelectedOption;

#[derive(Component)]
pub enum MenuButtonAction {
    Play,
    Settings,
    Quit,
}