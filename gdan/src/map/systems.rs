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

pub fn camera2dbundle(mut commands: Commands, map_info: Res<crate::map::resources::MapInfo>) {
    info!("camera2dbundle");
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(
                (map_info.label_x as f32 * map_info.unit_x) / 2. - map_info.unit_x / 2.,
                (map_info.label_y as f32 * map_info.unit_y) / 2. - map_info.unit_y / 2.,
                0.0,
            ),
            projection: OrthographicProjection {
                /*
                 * The projection contains the near and far values,
                 * which indicate the minimum and maximum Z coordinate (depth) that can be rendered,
                 * relative to the position (transform) of the camera.
                 * don't forget to set `near` and `far`
                 */
                near: -1000.0,
                far: 1000.0,
                scale: 5.,

                ..default()
            },
            ..default()
        },
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

pub fn init_map(mut commands: Commands) {
    info!("init_map");
    commands.insert_resource(crate::map::resources::MapInfo {
        scale: 0.5,
        unit_x: 1440.,
        unit_y: 810.,
        label_x: 3,
        label_y: 4,
        // level 20: 10.meter/72.pixel
        // level 21: 5.meter/72.pixel
        satellite_map_level: 20,
        meter_per_pixel: 10. / 72.,
    });
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

pub fn map_scale(
    mut scroll_evr: EventReader<MouseWheel>,
    mut map_info: ResMut<crate::map::resources::MapInfo>,
    // mut query: Query<
    //     &mut Transform,
    //     (
    //         With<crate::map::entities::MapMenu>,
    //         With<crate::map::entities::MapNC>,
    //     ),
    // >,
) {
    for ev in scroll_evr.read() {
        match ev.unit {
            MouseScrollUnit::Line => {
                if ev.y > 0. {
                    if map_info.scale < 1.5 {
                        map_info.scale += 0.1;
                    }
                } else if ev.y < 0. {
                    if map_info.scale > 0.3 {
                        map_info.scale -= 0.1;
                    }
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
    // Consider changing font-size instead of scaling the transform. Scaling a Text2D will scale the
    // rendered quad, resulting in a pixellated look.
    // for mut transform in &mut query {
    // transform.translation = Vec3::new(400.0, 0.0, 0.0);
    // let scale = map_info.scale;
    //     transform.scale.x = scale;
    //     transform.scale.y = scale;
    // }
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
