use bevy::ecs::system::*;
use bevy::time::*;

/*
 * If there is only one global instance (singleton) of something, such as configuration / settings.
 * and it is standalone (not associated with other data), create a Resource.
 * To create a new resource type, simply define a Rust struct or enum, and derive the Resource trait.
 */

#[derive(Resource)]
pub struct GreetTimer(pub Timer);
