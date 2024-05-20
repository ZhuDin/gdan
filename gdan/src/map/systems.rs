// Systems: normal Rust functions

use bevy::prelude::*;

pub fn init_map(mut commands: Commands) {
    info!("init_map");
    /*
     * accessing resources using Res/ResMut
     * accessing components of entities using queries (Query)
     * creating/destroying entities, components, and resources using Commands (Commands)
     * sending/receiving events using EventWriter/EventReader
     */
    let label_x: u32 = 1;
    let label_y: u32 = 1;

    let unit_x: f32 = 5040.;
    let unit_y: f32 = 3600.;

    commands.insert_resource(crate::map::resources::MapInfo {
        scale: 1.,
        unit_x,
        unit_y,
        label_x,
        label_y,
        // level 21: 10.meter/72.pixel
        // level 22: 5.meter/72.pixel
        satellite_map_level: 21,
        meter_per_pixel: 10. / 72.,
    });

    commands.insert_resource(crate::map::resources::Camera2dCoords(Vec2::new(0., 0.)));

    commands.insert_resource(crate::map::resources::Camera3dCoords(Vec3::new(
        // label_x as f32 / 2. * unit_x - unit_x / 4.,
        // label_y as f32 / 2. * unit_y - unit_y,
        // -2.5, 4.5, 9.0,
        0., 6., 8.,
    )));

    commands.insert_resource(crate::map::resources::Camera3dProjection {
        persp_fov: 0.,
        ortho_scale: 0.,
    });

    commands.insert_resource(crate::map::resources::MouseCoords {
        pre_x: 0.,
        pre_y: 0.,
        x: 0.,
        y: 0.,
    });

    commands.insert_resource(crate::map::resources::Circle001Coords {
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

pub fn camera3dbundle(
    mut commands: Commands,
    camera3dcoords: Res<crate::map::resources::Camera3dCoords>,
) {
    info!("camera3dbundle");
    /*
     * Bevy uses a right-handed Y-up coordinate system for the game world.
     * The coordinate system is the same for 3D and 2D, for consistency.
     * It is easiest to explain in terms of 2D:
     * The X axis goes from left to right (+X points right).
     * The Y axis goes from bottom to top (+Y points up).
     * The Z axis goes from far to near (+Z points towards you, out of the screen).
     * For 2D, the origin (X=0.0; Y=0.0) is at the center of the screen by default.
     * When you are working with 2D sprites, you can put the background on Z=0.0,
     * and place other sprites at increasing positive Z coordinates to layer them on top.
     *
     * In 3D, the axes are oriented the same way:
     * Y points up
     * The forward direction is -Z
     * This is a right-handed coordinate system.
     * You can use the fingers of your right hand to visualize the 3 axes: thumb=X, index=Y, middle=Z.
     */
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(
                camera3dcoords.0.x,
                camera3dcoords.0.y,
                camera3dcoords.0.z,
            )
            .looking_at(Vec3::ZERO, Vec3::Y),
            // projection: Projection::from(OrthographicProjection { ..default() }),
            ..default()
        },
        crate::map::entities::MapCamera3d,
        crate::map::entities::MapMenu,
    ));
}

pub fn map_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    state: Res<State<crate::MyAppState>>,
) {
    info!("map_menu");
    /*
     * For UI, Bevy follows the same convention as most other UI toolkits, the Web, etc.
     * The origin is at the top left corner of the screen
     * The Y axis points downwards
     * X goes from 0.0 (left screen edge) to the number of screen pixels (right screen edge)
     * Y goes from 0.0 (top screen edge) to the number of screen pixels (bottom screen edge)
     * The units represent logical (compensated for DPI scaling) screen pixels.
     * UI layout flows from top to bottom, similar to a web page.
     */
    match state.get() {
        crate::MyAppState::MapMenu => {
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

            /*
             * map Button
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
                    crate::map::entities::Map3d,
                    crate::map::entities::MapMenu,
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
                            crate::map::entities::Map3d,
                            crate::map::entities::MapMenu,
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
                                crate::map::entities::Map3d,
                                crate::map::entities::MapMenu,
                            ));
                        });
                });
        }
        crate::MyAppState::Map3D => {}
        _ => (),
    }
}

pub fn map_menu_system(
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
    windows: Query<&Window>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut next_state: ResMut<NextState<crate::MyAppState>>,
    mut text_query: Query<&mut Text>,
    mut gizmos: Gizmos,
    circle_001_coords: Res<crate::map::resources::Circle001Coords>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => match state.get() {
                crate::MyAppState::MapMenu => {
                    if text.sections[0].value == "3D".to_string() {
                        next_state.set(crate::MyAppState::Map3D);
                        info!("AppState::Map3D");
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
    if buttons.just_pressed(MouseButton::Left) {
        let Some(cursor_position) = windows.single().cursor_position() else {
            return;
        };
        info!("({},{})", cursor_position.x, cursor_position.y);
    }
    gizmos.circle_2d(
        bevy::prelude::Vec2 {
            x: circle_001_coords.x,
            y: circle_001_coords.y,
        },
        10.,
        Color::WHITE,
    );
}

pub fn add_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    state: Res<State<crate::MyAppState>>,
) {
    info!("add_map");
    match state.get() {
        crate::MyAppState::MapMenu => {
            // for x in 0..map_info.label_x {
            // for y in 0..map_info.label_y {
            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load(
                        // "wg/ncly/level21/".to_string()
                        //     + (x + 1).to_string().as_str()
                        //     + "-"
                        //     + (y + 1).to_string().as_str()
                        //     + ".png",
                        "wg/mlx/1-5040p-3600p.png",
                        // "branding/bevy_bird_dark.png",
                    ),

                    ..default()
                },
                crate::map::entities::MapNC,
                crate::map::entities::MapMenu,
            ));
            // }
            // }

            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load("wg/mlx/hexagon.png"),
                    transform: Transform {
                        translation: Vec3 {
                            x: 0.,
                            y: 0.,
                            z: 1.,
                        },
                        scale: Vec3 {
                            x: 0.42,
                            y: 0.42,
                            z: 0.,
                        },
                        ..default()
                    },
                    ..default()
                },
                crate::map::entities::MapHexagon,
                crate::map::entities::MapNC,
                crate::map::entities::MapMenu,
            ));
        }

        crate::MyAppState::Map3D => {
            // plane
            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(Plane3d::default().mesh().size(5.0, 5.0)),
                    material: materials.add(Color::rgb(0.3, 0.5, 0.3)),
                    ..default()
                },
                crate::map::entities::MapMenu,
            ));

            // cube
            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
                    material: materials.add(Color::rgb(0.8, 0.7, 0.6)),
                    transform: Transform::from_xyz(0.0, 0.5, 0.0),
                    ..default()
                },
                crate::map::entities::MapMenu,
            ));

            // light
            commands.spawn((
                PointLightBundle {
                    point_light: PointLight {
                        shadows_enabled: true,
                        ..default()
                    },
                    transform: Transform::from_xyz(4.0, 8.0, 4.0),
                    ..default()
                },
                crate::map::entities::MapMenu,
            ));

            // example instructions
            commands.spawn((
                TextBundle::from_section(
                    "Press 'D' to toggle drawing gizmos on top of everything else in the scene\n\
            Press 'P' to toggle perspective for line gizmos\n\
            Hold 'Left' or 'Right' to change the line width of straight gizmos\n\
            Hold 'Up' or 'Down' to change the line width of round gizmos\n\
            Press '1' or '2' to toggle the visibility of straight gizmos or round gizmos\n\
            Press 'A' to show all AABB boxes\n\
            Press 'K' or 'J' to cycle through primitives rendered with gizmos\n\
            Press 'H' or 'L' to decrease/increase the amount of segments in the primitives",
                    TextStyle {
                        font_size: 20.,
                        ..default()
                    },
                )
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(8.0),
                    left: Val::Px(8.0),
                    ..default()
                }),
                crate::map::entities::MapMenu,
            ));
        }
        _ => (),
    }
}

pub fn map2d_scale_wander(
    mut query_camera_projection: Query<
        &mut OrthographicProjection,
        With<crate::map::entities::MapMenu>,
    >,
    mut query_camera2d_transform: Query<
        &mut Transform,
        (
            With<crate::map::entities::MapCamera2d>,
            Without<crate::map::entities::MapCamera3d>,
        ),
    >,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut scroll_evr: EventReader<bevy::input::mouse::MouseWheel>,
    // q_windows: Query<&Window, With<bevy::window::PrimaryWindow>>,
    // mouse_coords: ResMut<crate::map::resources::MouseCoords>,
    map_info: ResMut<crate::map::resources::MapInfo>,
) {
    /*
     * The cursor position and any other window (screen-space) coordinates follow the same conventions as UI.
     */
    let mut projection = query_camera_projection.single_mut();
    for ev in scroll_evr.read() {
        match ev.unit {
            bevy::input::mouse::MouseScrollUnit::Line => {
                if ev.y > 0. && projection.scale > 1. {
                    projection.scale /= 1.25;
                } else if ev.y < 0. && projection.scale < 5. {
                    projection.scale *= 1.25;
                }
            }
            bevy::input::mouse::MouseScrollUnit::Pixel => {
                // println!(
                //     "Scroll (pixel units): vertical: {}, horizontal: {}",
                //     ev.y, ev.x
                // );
                // map_info.scale -= 0.1;
            }
        }
    }
    let mut transform = query_camera2d_transform.single_mut();
    if (keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp))
        && transform.translation.y < map_info.unit_y * 0.55
    {
        transform.translation.y += 10.;
    }

    if (keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown))
        && transform.translation.y > -map_info.unit_y * 0.55
    {
        transform.translation.y -= 10.;
    }

    if (keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight))
        && transform.translation.x < map_info.unit_x * 0.55
    {
        transform.translation.x += 10.;
    }

    if (keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft))
        && transform.translation.x > -map_info.unit_x * 0.55
    {
        transform.translation.x -= 10.;
    }
}

pub fn map3d_scale_wander(
    mut query_camera3d_projection: Query<&mut Projection, With<crate::map::entities::MapCamera3d>>,
    mut query_camera3d_transform: Query<&mut Transform, With<crate::map::entities::MapCamera3d>>,
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut scroll_evr: EventReader<bevy::input::mouse::MouseWheel>,
    q_windows: Query<&Window, With<bevy::window::PrimaryWindow>>,
    mut mouse_coords: ResMut<crate::map::resources::MouseCoords>,
    // mut map_info: ResMut<crate::map::resources::MapInfo>,
) {
    let mut transform = query_camera3d_transform.single_mut();
    transform.rotate_around(Vec3::ZERO, Quat::from_rotation_y(time.delta_seconds() / 2.));

    for ev in scroll_evr.read() {
        match ev.unit {
            bevy::input::mouse::MouseScrollUnit::Line => {
                /*
                 * Do not use the transform scale to "zoom" a camera!
                 * It just stretches the image, which is not "zooming".
                 * It might also cause other issues and incompatibilities. Use the projection to zoom.
                 * For an orthographic projection, change the scale. For a perspective projection, change the FOV.
                 */
                let camera3d_projection = query_camera3d_projection.single_mut().into_inner();
                match camera3d_projection {
                    // 3D cameras can use either a Perspective or an Orthographic projection.
                    // Perspective is the default, and most common, choice.
                    Projection::Perspective(persp) => {
                        if ev.y > 0. && persp.fov > 0.5 {
                            // we have a perspective projection
                            persp.fov /= 1.25;
                        } else if ev.y < 0. && persp.fov < 1.5 {
                            persp.fov *= 1.25;
                        }
                    }
                    Projection::Orthographic(ortho) => {
                        if ev.y > 0. && ortho.scale > 0.5 {
                            // we have an orthographic projection
                            ortho.scale /= 1.25; // zoom in
                        } else if ev.y < 0. && ortho.scale < 1.5 {
                            ortho.scale *= 1.25; // zoom out
                        }
                    }
                }
            }
            bevy::input::mouse::MouseScrollUnit::Pixel => {
                // println!(
                //     "Scroll (pixel units): vertical: {}, horizontal: {}",
                //     ev.y, ev.x
                // );
                // map_info.scale -= 0.1;
            }
        }
    }
    if buttons.just_pressed(MouseButton::Right) {
        if let Some(position) = q_windows.single().cursor_position() {
            mouse_coords.pre_x = position.x;
            mouse_coords.pre_y = position.y;
        } else {
            info!("Cursor is not in the game window.");
        }
    }
    if buttons.just_released(MouseButton::Right) {
        if let Some(position) = q_windows.single().cursor_position() {
            if position.x > mouse_coords.pre_x && position.y > mouse_coords.pre_y {}
            if position.x > mouse_coords.pre_x && position.y < mouse_coords.pre_y {}
            if position.x < mouse_coords.pre_x && position.y > mouse_coords.pre_y {}
            if position.x < mouse_coords.pre_x && position.y < mouse_coords.pre_y {}
        } else {
            info!("Cursor is not in the game window.");
        }
    }
    if keyboard.just_pressed(KeyCode::ArrowUp) {
        let mut camera3d_transform = query_camera3d_transform.single_mut();
        camera3d_transform.translation.z -= 1.;
    }
    if keyboard.just_pressed(KeyCode::ArrowDown) {
        let mut camera3d_transform = query_camera3d_transform.single_mut();
        camera3d_transform.translation.z += 1.;
    }
    if keyboard.just_pressed(KeyCode::ArrowLeft) {
        let mut camera3d_transform = query_camera3d_transform.single_mut();
        camera3d_transform.translation.x -= 1.;
    }
    if keyboard.just_pressed(KeyCode::ArrowRight) {
        let mut camera3d_transform = query_camera3d_transform.single_mut();
        camera3d_transform.translation.x += 1.;
    }
}

pub fn draw_line_collection(
    mut gizmos: Gizmos,
    mut my_gizmos: Gizmos<crate::MyRoundGizmos>,
    time: Res<Time>,
) {
    gizmos.cuboid(
        Transform::from_translation(Vec3::Y * 0.5).with_scale(Vec3::splat(1.25)),
        Color::BLACK,
    );
    gizmos.rect(
        Vec3::new(time.elapsed_seconds().cos() * 2.5, 1., 0.),
        Quat::from_rotation_y(std::f32::consts::PI / 2.),
        Vec2::splat(2.),
        Color::GREEN,
    );

    my_gizmos.sphere(Vec3::new(1., 0.5, 0.), Quat::IDENTITY, 0.5, Color::RED);

    for y in [0., 0.5, 1.] {
        gizmos.ray(
            Vec3::new(1., y, 0.),
            Vec3::new(-3., (time.elapsed_seconds() * 3.).sin(), 0.),
            Color::BLUE,
        );
    }

    my_gizmos
        .arc_3d(
            180.0_f32.to_radians(),
            0.2,
            Vec3::ONE,
            Quat::from_rotation_arc(Vec3::Y, Vec3::ONE.normalize()),
            Color::ORANGE,
        )
        .segments(10);

    // Circles have 32 line-segments by default.
    my_gizmos.circle(Vec3::ZERO, Direction3d::Y, 3., Color::BLACK);
    // You may want to increase this for larger circles or spheres.
    my_gizmos
        .circle(Vec3::ZERO, Direction3d::Y, 3.1, Color::NAVY)
        .segments(64);
    my_gizmos
        .sphere(Vec3::ZERO, Quat::IDENTITY, 3.2, Color::BLACK)
        .circle_segments(64);

    gizmos.arrow(Vec3::ZERO, Vec3::ONE * 1.5, Color::YELLOW);
}

pub fn add_oper() {}

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
