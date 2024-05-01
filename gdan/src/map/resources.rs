use bevy::ecs::system::*;
use bevy::time::*;

/*
 * If there is only one global instance (singleton) of something,
 * and it is standalone (not associated with other data), create a Resource.
 */

#[derive(Resource)]
pub struct GreetTimer(pub Timer);
