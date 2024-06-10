use bevy::prelude::*;

pub fn camera3dbundle(mut commands: crate::Commands) {
    info!("camera3dbundle");

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        crate::scene::entities::SceneCamera3d,
    ));
}

pub fn show_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    info!("show_map");
    // load a texture and retrieve its aspect ratio
    let texture_handle = asset_server.load("wg/mlx/map/1-8819p-6299p.png");

    // textured quad - normal
    commands.spawn(PbrBundle {
        mesh: meshes.add(Rectangle::new(8.819, 6.299)),
        material: materials.add(StandardMaterial {
            base_color_texture: Some(texture_handle.clone()),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 0.0, 0.),
        ..default()
    });
}
