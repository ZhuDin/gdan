use bevy::prelude::*;
use bevy::sprite::*;

pub fn camera2dbundle(mut commands: Commands) {
    info!("camera2dbundle");
    commands.spawn((Camera2dBundle::default(), crate::oper::entities::OperMenu));
}

pub fn oper_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    info!("oper_setup");
    let shapes = [
        Mesh2dHandle(meshes.add(Circle { radius: 50.0 })),
        Mesh2dHandle(meshes.add(Ellipse::new(25.0, 50.0))),
        Mesh2dHandle(meshes.add(Capsule2d::new(25.0, 50.0))),
        Mesh2dHandle(meshes.add(Rectangle::new(50.0, 100.0))),
        Mesh2dHandle(meshes.add(RegularPolygon::new(50.0, 6))),
        Mesh2dHandle(meshes.add(Triangle2d::new(
            Vec2::Y * 50.0,
            Vec2::new(-50.0, -50.0),
            Vec2::new(50.0, -50.0),
        ))),
    ];
    let num_shapes = shapes.len();

    for (i, shape) in shapes.into_iter().enumerate() {
        // Distribute colors evenly across the rainbow.
        let color = Color::hsl(360. * i as f32 / num_shapes as f32, 0.95, 0.7);

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: shape,
                material: materials.add(color),
                transform: Transform::from_xyz(
                    // Distribute shapes from -X_EXTENT to +X_EXTENT.
                    -600. / 2. + i as f32 / (num_shapes - 1) as f32 * 600.,
                    0.0,
                    0.0,
                ),
                ..default()
            },
            crate::oper::entities::OperMenu,
        ));
    }
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
