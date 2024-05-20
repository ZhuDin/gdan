/*
 * If there is only one global instance (singleton) of something, such as configuration / settings.
 * and it is standalone (not associated with other data), create a Resource.
 * To create a new resource type, simply define a Rust struct or enum, and derive the Resource trait.
 * Resources allow you to store a single global instance of some data type, independently of entities.
 */

#[derive(bevy::ecs::system::Resource)]
pub struct GreetTimer(pub bevy::time::Timer);

#[derive(bevy::ecs::system::Resource)]
pub struct MapInfo {
    pub scale: f32,
    pub unit_x: f32,
    pub unit_y: f32,
    pub label_x: u32,
    pub label_y: u32,
    pub satellite_map_level: u8,
    pub meter_per_pixel: f32,
}

/// We will store the world position of the mouse cursor here.
#[derive(bevy::ecs::system::Resource, Default)]
pub struct Camera2dCoords(pub bevy::math::Vec2);

#[derive(bevy::ecs::system::Resource, Default)]
pub struct Camera3dCoords(pub bevy::math::Vec3);

#[derive(bevy::ecs::system::Resource, Default)]
pub struct Camera3dProjection {
    pub persp_fov: f32,
    pub ortho_scale: f32,
}

#[derive(bevy::ecs::system::Resource, Default)]
pub struct MouseCoords {
    pub pre_x: f32,
    pub pre_y: f32,
    pub x: f32,
    pub y: f32,
}

#[derive(bevy::ecs::system::Resource, Default)]
pub struct Circle001Coords {
    pub pre_x: f32,
    pub pre_y: f32,
    pub x: f32,
    pub y: f32,
}
