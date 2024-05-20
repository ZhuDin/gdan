use bevy::prelude::*;

pub fn camera2dbundle(mut commands: Commands) {
    info!("camera2dbundle");
    commands.spawn((Camera2dBundle::default(), crate::oper::entities::OperMenu));
}

pub fn camera3dbundle(mut commands: Commands) {
    info!("camera3dbundle");
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 6., 12.0)
                .looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
            ..default()
        },
        crate::oper::entities::OperMenu,
    ));
}

pub fn oper_setup(
    mut commands: Commands,
    state: Res<State<crate::MyAppState>>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
    mut images: ResMut<Assets<Image>>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
) {
    match state.get() {
        crate::MyAppState::OperMenu => {
            info!("oper_setup");
            let shapes = [
                bevy::sprite::Mesh2dHandle(meshes.add(Circle { radius: 50.0 })),
                bevy::sprite::Mesh2dHandle(meshes.add(Ellipse::new(25.0, 50.0))),
                bevy::sprite::Mesh2dHandle(meshes.add(Capsule2d::new(25.0, 50.0))),
                bevy::sprite::Mesh2dHandle(meshes.add(Rectangle::new(50.0, 100.0))),
                bevy::sprite::Mesh2dHandle(meshes.add(RegularPolygon::new(50.0, 6))),
                bevy::sprite::Mesh2dHandle(meshes.add(Triangle2d::new(
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
                    bevy::sprite::MaterialMesh2dBundle {
                        mesh: shape,
                        material: color_materials.add(color),
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

            /*
             * Oper Button
             */
            commands
                .spawn((
                    bevy::ui::node_bundles::NodeBundle {
                        style: bevy::ui::Style {
                            width: bevy::ui::Val::Percent(100.0),
                            height: bevy::ui::Val::Percent(100.0),
                            align_items: bevy::ui::AlignItems::End,
                            justify_content: bevy::ui::JustifyContent::Center,
                            ..default()
                        },
                        ..default()
                    },
                    crate::oper::entities::Oper3D,
                    crate::oper::entities::OperMenu,
                ))
                .with_children(|parent| {
                    parent
                        .spawn((
                            bevy::ui::node_bundles::ButtonBundle {
                                style: bevy::ui::Style {
                                    width: bevy::ui::Val::Px(120.0),
                                    height: bevy::ui::Val::Px(50.0),
                                    border: bevy::ui::UiRect::all(bevy::ui::Val::Px(5.0)),
                                    // horizontally center child text
                                    justify_content: bevy::ui::JustifyContent::Center,
                                    // vertically center child text
                                    align_items: bevy::ui::AlignItems::Center,
                                    ..default()
                                },
                                border_color: bevy::ui::BorderColor(
                                    bevy::render::color::Color::BLACK,
                                ),
                                background_color: bevy::render::color::Color::rgb(0.15, 0.15, 0.15)
                                    .into(),
                                ..default()
                            },
                            crate::oper::entities::Oper3D,
                            crate::oper::entities::OperMenu,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                bevy::ui::node_bundles::TextBundle::from_section(
                                    "3D",
                                    bevy::text::TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: 40.0,
                                        color: bevy::render::color::Color::rgb(0.9, 0.9, 0.9),
                                    },
                                ),
                                crate::oper::entities::Oper3D,
                                crate::oper::entities::OperMenu,
                            ));
                        });
                });
        }
        crate::MyAppState::Oper3D => {
            /// Creates a colorful test pattern
            fn uv_debug_texture() -> Image {
                const TEXTURE_SIZE: usize = 8;

                let mut palette: [u8; 32] = [
                    255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255,
                    102, 255, 198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
                ];

                let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
                for y in 0..TEXTURE_SIZE {
                    let offset = TEXTURE_SIZE * y * 4;
                    texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
                    palette.rotate_right(4);
                }

                Image::new_fill(
                    bevy::render::render_resource::Extent3d {
                        width: TEXTURE_SIZE as u32,
                        height: TEXTURE_SIZE as u32,
                        depth_or_array_layers: 1,
                    },
                    bevy::render::render_resource::TextureDimension::D2,
                    &texture_data,
                    bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb,
                    bevy::render::render_asset::RenderAssetUsages::RENDER_WORLD,
                )
            }

            let debug_material = standard_materials.add(StandardMaterial {
                base_color_texture: Some(images.add(uv_debug_texture())),
                ..default()
            });

            let shapes = [
                meshes.add(Cuboid::default()),
                meshes.add(Capsule3d::default()),
                meshes.add(Torus::default()),
                meshes.add(Cylinder::default()),
                meshes.add(Sphere::default().mesh().ico(5).unwrap()),
                meshes.add(Sphere::default().mesh().uv(32, 18)),
            ];

            let num_shapes = shapes.len();

            for (i, shape) in shapes.into_iter().enumerate() {
                commands.spawn((
                    PbrBundle {
                        mesh: shape,
                        material: debug_material.clone(),
                        transform: Transform::from_xyz(
                            -12. / 2. + i as f32 / (num_shapes - 1) as f32 * 12.,
                            2.0,
                            0.0,
                        )
                        .with_rotation(Quat::from_rotation_x(-std::f32::consts::PI / 4.)),
                        ..default()
                    },
                    crate::oper::entities::Oper3D,
                    crate::oper::entities::OperMenu,
                ));
            }

            commands.spawn((
                PointLightBundle {
                    point_light: PointLight {
                        shadows_enabled: true,
                        intensity: 10_000_000.,
                        range: 100.0,
                        ..default()
                    },
                    transform: Transform::from_xyz(8.0, 16.0, 8.0),
                    ..default()
                },
                crate::oper::entities::OperMenu,
            ));

            // ground plane
            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(Plane3d::default().mesh().size(50.0, 50.0)),
                    material: standard_materials.add(Color::SILVER),
                    ..default()
                },
                crate::oper::entities::OperMenu,
            ));
        }
        _ => (),
    }
}

pub fn oper_menu_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    state: Res<State<crate::MyAppState>>,
    mut next_state: ResMut<NextState<crate::MyAppState>>,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => match state.get() {
                crate::MyAppState::OperMenu => {
                    if text.sections[0].value == "3D".to_string() {
                        next_state.set(crate::MyAppState::Oper3D);
                        info!("AppState::Oper3D");
                    }
                }
                _ => (),
            },
            Interaction::Hovered => {
                // text.sections[0].value = "Hover".to_string();
                *color = Color::rgb(0.25, 0.25, 0.25).into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                // text.sections[0].value = "Button".to_string();
                *color = Color::rgb(0.15, 0.15, 0.15).into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

pub fn oper3d_scale_wander(
    mut query: Query<&mut Transform, With<crate::oper::entities::Oper3D>>,
    time: Res<Time>,
) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_seconds() / 2.);
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
