use bevy::ecs::system::*;
use bevy::time::*;

#[derive(Resource)]
pub struct GreetTimer(pub Timer);
