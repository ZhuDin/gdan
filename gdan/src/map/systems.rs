// Systems: normal Rust functions

use bevy::ecs::query::*;
use bevy::ecs::system::*;
use bevy::input::mouse::*;
use bevy::log::*;
use bevy::prelude::*;

/*
 * accessing resources using Res/ResMut
 * accessing components of entities using queries (Query)
 * creating/destroying entities, components, and resources using Commands (Commands)
 * sending/receiving events using EventWriter/EventReader
 */
pub fn init_map(mut commands: Commands) {
    info!("init_map");

    let label_x: u32 = 3;
    let label_y: u32 = 4;

    let unit_x: f32 = 1440.;
    let unit_y: f32 = 810.;

    commands.insert_resource(crate::map::resources::MapInfo {
        scale: 5.,
        unit_x,
        unit_y,
        label_x,
        label_y,
        // level 21: 10.meter/72.pixel
        // level 22: 5.meter/72.pixel
        satellite_map_level: 21,
        meter_per_pixel: 10. / 72.,
    });

    commands.insert_resource(crate::map::resources::Camera2dCoords(Vec2::new(
        label_x as f32 / 2. * unit_x - unit_x / 4.,
        label_y as f32 / 2. * unit_y - unit_y,
    )));

    commands.insert_resource(crate::map::resources::MouseCoords {
        pre_x: 0.,
        pre_y: 0.,
        x: 0.,
        y: 0.,
    });
}

pub fn camera2dbundle(
    mut commands: Commands,
    map_info: Res<crate::map::resources::MapInfo>,
    camera2dcoords: Res<crate::map::resources::Camera2dCoords>,
) {
    info!("camera2dbundle");

    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(camera2dcoords.0.x, camera2dcoords.0.y, 0.0),
            projection: OrthographicProjection {
                /*
                 * The projection contains the near and far values,
                 * which indicate the minimum and maximum Z coordinate (depth) that can be rendered,
                 * relative to the position (transform) of the camera.
                 * don't forget to set `near` and `far`
                 */
                near: -1000.0,
                far: 1000.0,
                scale: map_info.scale,
                ..default()
            },
            ..default()
        },
        // bevy::core_pipeline::core_2d::Camera2dBundle::default(),
        crate::map::entities::MapCamera2d,
        crate::map::entities::MapMenu,
    ));
}

pub fn map_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("map_menu");
    commands.spawn((
        TextBundle::from_section(
            "show map",
            TextStyle {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: 24.,
                color: Color::WHITE,
            },
        ),
        crate::map::entities::MapMenu,
    ));
}

pub fn add_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    map_info: Res<crate::map::resources::MapInfo>,
) {
    info!("add_map");

    for x in 0..map_info.label_x {
        for y in 0..map_info.label_y {
            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load(
                        "wg/ncly/level21/".to_string()
                            + (x + 1).to_string().as_str()
                            + "-"
                            + (y + 1).to_string().as_str()
                            + ".png",
                    ),
                    transform: Transform::from_xyz(
                        x as f32 * map_info.unit_x,
                        y as f32 * map_info.unit_y,
                        0.,
                    ),
                    ..default()
                },
                crate::map::entities::MapNC,
                crate::map::entities::MapMenu,
            ));
        }
    }
}

pub fn map_scale_wander(
    mut query_camera_projection: Query<
        &mut OrthographicProjection,
        With<crate::map::entities::MapMenu>,
    >,
    mut query_camera_transform: Query<
        &mut Transform,
        (
            With<crate::map::entities::MapMenu>,
            With<crate::map::entities::MapCamera2d>,
        ),
    >,
    buttons: Res<ButtonInput<MouseButton>>,
    mut scroll_evr: EventReader<MouseWheel>,
    q_windows: Query<&Window, With<bevy::window::PrimaryWindow>>,
    mut mouse_coords: ResMut<crate::map::resources::MouseCoords>,
    // mut map_info: ResMut<crate::map::resources::MapInfo>,
) {
    let mut projection = query_camera_projection.single_mut();
    for ev in scroll_evr.read() {
        match ev.unit {
            MouseScrollUnit::Line => {
                if ev.y > 0. && projection.scale > 0.8 {
                    projection.scale /= 1.25;
                } else if ev.y < 0. && projection.scale < 8. {
                    projection.scale *= 1.25;
                }
            }
            MouseScrollUnit::Pixel => {
                // println!(
                //     "Scroll (pixel units): vertical: {}, horizontal: {}",
                //     ev.y, ev.x
                // );
                // map_info.scale -= 0.1;
            }
        }
    }
    let mut transform = query_camera_transform.single_mut();
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(position) = q_windows.single().cursor_position() {
            mouse_coords.pre_x = position.x;
            mouse_coords.pre_y = position.y;
        } else {
            info!("Cursor is not in the game window.");
        }
    }
    if buttons.just_released(MouseButton::Left) {
        if let Some(position) = q_windows.single().cursor_position() {
            if position.x > mouse_coords.pre_x && position.y > mouse_coords.pre_y {
                transform.translation.x -= position.x - mouse_coords.pre_x;
                transform.translation.y += position.y - mouse_coords.pre_y;
            }
            if position.x > mouse_coords.pre_x && position.y < mouse_coords.pre_y {
                transform.translation.x -= position.x - mouse_coords.pre_x;
                transform.translation.y -= mouse_coords.pre_y - position.y;
            }
            if position.x < mouse_coords.pre_x && position.y > mouse_coords.pre_y {
                transform.translation.x += mouse_coords.pre_x - position.x;
                transform.translation.y += position.y - mouse_coords.pre_y;
            }
            if position.x < mouse_coords.pre_x && position.y < mouse_coords.pre_y {
                transform.translation.x += mouse_coords.pre_x - position.x;
                transform.translation.y -= mouse_coords.pre_y - position.y;
            }
        } else {
            info!("Cursor is not in the game window.");
        }
    }
}

pub fn despawn_map_menu(
    query_enemy: Query<Entity, With<crate::map::entities::MapMenu>>,
    mut commands: Commands,
) {
    info!("despawn_map_menu");
    for entity_id in query_enemy.iter() {
        // commands.entity(entity_id).remove::<MainInfo>();
        commands.entity(entity_id).despawn();
        // .insert(Friendly);
    }
}
