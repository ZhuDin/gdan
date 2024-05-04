use bevy::ecs::component::Component;
use bevy::prelude::*;

#[derive(Component)]
pub struct GameMenu;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum GameState {
    AabbSweep,
    CircleSweep,
    #[default]
    RayCast,
    AabbCast,
    CircleCast,
}
