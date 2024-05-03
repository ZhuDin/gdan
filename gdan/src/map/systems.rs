// Systems: normal Rust functions

use bevy::ecs::query::*;
use bevy::ecs::system::*;
use bevy::log::info;
use bevy::prelude::*;

/*
 * accessing resources using Res/ResMut
 * accessing components of entities using queries (Query)
 * creating/destroying entities, components, and resources using Commands (Commands)
 * sending/receiving events using EventWriter/EventReader
 */

pub fn add_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("add_map");
    commands.spawn((Camera2dBundle::default(), crate::map::entities::MapInfo));

    commands.spawn((
        crate::map::components::MapName("NC".to_string()),
        crate::map::components::MapSize { x: 1, y: 1, z: 1 },
        crate::map::entities::MapInfo,
    ));
    commands.spawn((
        crate::map::components::MapName("XinZhu".to_string()),
        crate::map::components::MapSize { x: 1, y: 1, z: 1 },
        crate::map::entities::MapInfo,
    ));

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("wg/ncly/level21/1-1.png"),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
        crate::map::entities::MapNC,
        crate::map::entities::MapInfo,
    ));
}

pub fn map_scale(
    mut query: Query<
        &mut Transform,
        (
            With<crate::map::entities::MapInfo>,
            With<crate::map::entities::MapNC>,
        ),
    >,
) {
    // Consider changing font-size instead of scaling the transform. Scaling a Text2D will scale the
    // rendered quad, resulting in a pixellated look.
    for mut transform in &mut query {
        // transform.translation = Vec3::new(400.0, 0.0, 0.0);

        let scale = 1.;
        transform.scale.x = scale;
        transform.scale.y = scale;
    }
}

// pub fn show_map(
//     query: Query<&crate::map::components::MapName, With<crate::map::components::MapInfo>>,
// ) {
//     info!("show_map");
//     for name in &query {
//         println!("hello {}!", name.0);
//     }
// }

pub fn show_map(
    time: Res<bevy::time::Time>,
    mut timer: ResMut<crate::map::resources::GreetTimer>,
    query: Query<&crate::map::components::MapName, With<crate::map::entities::MapInfo>>,
) {
    // update our timer with the time elapsed since the last update
    // if that caused the timer to finish, we say hello to everyone
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}

pub fn despawn_map_menu(
    query_enemy: Query<Entity, With<crate::map::entities::MapInfo>>,
    mut commands: Commands,
) {
    info!("despawn_main_menu");
    for entity_id in query_enemy.iter() {
        // commands.entity(entity_id).remove::<MainInfo>();
        commands.entity(entity_id).despawn();
        // .insert(Friendly);
    }
}
