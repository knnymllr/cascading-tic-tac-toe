use crate::{CellState};
use bevy::ecs::component::Component;
use bevy::prelude::{NextState, ResMut, States};

#[derive(Component, Clone)]
pub struct TicTacToeCell {
    pub cell_id: u32,
    pub state: CellState,
}

#[derive(Component, Clone)]
pub struct GridCell {
    // Max board 4,294,967,295 x 4,294,967,295
    // max number of games, n = 4,294,967,292 (MAX - 3)
    // Third option: No target, no time, play til you want to quit
    pub position: (u32, u32),  
    pub cell: TicTacToeCell,
}

/// current: State
/// next: ResMux<NextState>
#[derive(Debug)]
pub struct StateWrapper<'w, T: States> {
    pub current: T,
    pub next: ResMut<'w, NextState<T>>,
}
