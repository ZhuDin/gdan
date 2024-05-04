use bevy::prelude::*;

pub fn camera2dbundle(mut commands: Commands) {
    info!("camera2dbundle");
    commands.spawn((Camera2dBundle::default(), crate::oper::entities::OperMenu));
}

pub fn despawn_oper_menu(
    query_enemy: Query<Entity, With<crate::oper::entities::OperMenu>>,
    mut commands: Commands,
) {
    info!("despawn_oper_menu");
    for entity_id in query_enemy.iter() {
        // commands.entity(entity_id).remove::<MainInfo>();
        commands.entity(entity_id).despawn();
        // .insert(Friendly);
    }
}
