// Components: Rust structs that implement the Component trait
use bevy::ecs::component::Component;

#[derive(Component)]
pub struct MapSize {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

#[derive(Component)]
pub struct MapName(pub String);
