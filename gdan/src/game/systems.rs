use bevy::prelude::*;

pub fn camera2dbundle(mut commands: Commands) {
    info!("camera2dbundle");
    commands.spawn((Camera2dBundle::default(), crate::game::entities::GameMenu));
}

pub fn despawn_game_menu(
    query_enemy: Query<Entity, With<crate::game::entities::GameMenu>>,
    mut commands: Commands,
) {
    info!("despawn_game_menu");
    for entity_id in query_enemy.iter() {
        // commands.entity(entity_id).remove::<MainInfo>();
        commands.entity(entity_id).despawn();
        // .insert(Friendly);
    }
}
