// Systems: normal Rust functions
use bevy::ecs::query::*;
use bevy::ecs::system::*;
use bevy::log::info;

pub fn add_map(mut commands: Commands) {
    info!("add_map");
    commands.spawn((
        crate::map::entities::MapInfo,
        crate::map::components::MapName("NC".to_string()),
        crate::map::components::MapSize { x: 1, y: 1, z: 1 },
    ));
    commands.spawn((
        crate::map::entities::MapInfo,
        crate::map::components::MapName("XinZhu".to_string()),
        crate::map::components::MapSize { x: 1, y: 1, z: 1 },
    ));
}

// pub fn show_map(
//     query: Query<&crate::map::components::MapName, With<crate::map::components::MapInfo>>,
// ) {
//     info!("show_map");
//     for name in &query {
//         println!("hello {}!", name.0);
//     }
// }

pub fn update_map_name(
    mut query: Query<&mut crate::map::components::MapName, With<crate::map::entities::MapInfo>>,
) {
    info!("update_map_name");
    for mut name in &mut query {
        if name.0 == "XinZhu" {
            name.0 = "XZ".to_string();
            break;
        }
    }
}

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
