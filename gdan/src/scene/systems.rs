use bevy::prelude::*;

pub fn camera3dbundle(mut commands: crate::Commands) {
    info!("camera3dbundle");

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 300.0).looking_at(Vec3::ZERO, Vec3::Y),
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
        mesh: meshes.add(Rectangle::new(700., 500.)),
        material: materials.add(StandardMaterial {
            base_color_texture: Some(texture_handle.clone()),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 0.0, 0.),
        ..default()
    });

    let subway_2 = asset_server.load("wg/mlx/map/subway-2.png");
    commands.spawn(PbrBundle {
        mesh: meshes.add(Rectangle::new(8.19, 6.99)),
        material: materials.add(StandardMaterial {
            base_color_texture: Some(subway_2.clone()),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 0.0, -20.),
        ..default()
    });

    let ktv_1 = asset_server.load("wg/mlx/map/ktv-1.png");
    commands.spawn(PbrBundle {
        mesh: meshes.add(Rectangle::new(15. * 5., 20. * 5.)),
        material: materials.add(StandardMaterial {
            base_color_texture: Some(ktv_1.clone()),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        }),
        transform: Transform::from_xyz(-30.0, -10.0, 5.),
        ..default()
    });
}
