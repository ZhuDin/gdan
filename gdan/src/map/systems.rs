// Systems: normal Rust functions

use bevy::prelude::*;

/*
 * accessing resources using Res/ResMut
 * accessing components of entities using queries (Query)
 * creating/destroying entities, components, and resources using Commands (Commands)
 * sending/receiving events using EventWriter/EventReader
 */
pub fn init_map(mut commands: Commands) {
    info!("init_map");

    let label_x: u32 = 3;
    let label_y: u32 = 4;

    let unit_x: f32 = 1440.;
    let unit_y: f32 = 810.;

    commands.insert_resource(crate::map::resources::MapInfo {
        scale: 5.,
        unit_x,
        unit_y,
        label_x,
        label_y,
        // level 21: 10.meter/72.pixel
        // level 22: 5.meter/72.pixel
        satellite_map_level: 21,
        meter_per_pixel: 10. / 72.,
    });

    commands.insert_resource(crate::map::resources::Camera3dCoords(Vec3::new(
        // label_x as f32 / 2. * unit_x - unit_x / 4.,
        // label_y as f32 / 2. * unit_y - unit_y,
        0., 0., 20.,
    )));

    commands.insert_resource(crate::map::resources::Camera2dCoords(Vec2::new(
        label_x as f32 / 2. * unit_x - unit_x / 4.,
        label_y as f32 / 2. * unit_y - unit_y,
    )));

    commands.insert_resource(crate::map::resources::MouseCoords {
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
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(3.0, 5.0, camera3dcoords.0.z)
                .looking_at(Vec3::ZERO, Vec3::Y),

            ..default()
        },
        crate::map::entities::MapCamera3d,
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
                        border_color: bevy::ui::BorderColor(bevy::render::color::Color::BLACK),
                        background_color: bevy::render::color::Color::rgb(0.15, 0.15, 0.15).into(),
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
    mut next_state: ResMut<NextState<crate::MyAppState>>,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => match state.get() {
                crate::MyAppState::MapMenu => {
                    if text.sections[0].value == "3D".to_string() {
                        next_state.set(crate::MyAppState::Map3D);
                        info!("w_game_system -> MyAppState::Map3D");
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

pub fn add_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    state: Res<State<crate::MyAppState>>,
    map_info: Res<crate::map::resources::MapInfo>,
) {
    info!("add_map");
    match state.get() {
        crate::MyAppState::MapMenu => {
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

            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load("wg/ncly/level21/hexagon.png"),

                    transform: Transform {
                        translation: Vec3 {
                            x: map_info.label_x as f32 / 2. * map_info.unit_x
                                - map_info.unit_x / 2.,
                            y: map_info.label_y as f32 / 2. * map_info.unit_y
                                - map_info.unit_y / 2.,
                            z: 1.,
                        },
                        scale: Vec3 {
                            x: 0.378,
                            y: 0.378,
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
            commands.spawn(PbrBundle {
                mesh: meshes.add(Rectangle::new(8., 8.)).clone(),
                material: materials.add(StandardMaterial {
                    base_color_texture: Some(asset_server.load("wg/ncly/level21/1-1.png").clone()),
                    alpha_mode: AlphaMode::Blend,
                    unlit: true,
                    ..default()
                }),
                transform: Transform::from_xyz(0.0, 0.0, 0.)
                    .with_rotation(Quat::from_rotation_x(-core::f32::consts::PI / 5.0)),
                ..default()
            });
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
    buttons: Res<ButtonInput<MouseButton>>,
    mut scroll_evr: EventReader<bevy::input::mouse::MouseWheel>,
    q_windows: Query<&Window, With<bevy::window::PrimaryWindow>>,
    mut mouse_coords: ResMut<crate::map::resources::MouseCoords>,
    // mut map_info: ResMut<crate::map::resources::MapInfo>,
) {
    let mut projection = query_camera_projection.single_mut();
    for ev in scroll_evr.read() {
        match ev.unit {
            bevy::input::mouse::MouseScrollUnit::Line => {
                if ev.y > 0. && projection.scale > 0.8 {
                    projection.scale /= 1.25;
                } else if ev.y < 0. && projection.scale < 8. {
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
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(position) = q_windows.single().cursor_position() {
            mouse_coords.pre_x = position.x;
            mouse_coords.pre_y = position.y;
        } else {
            info!("Cursor is not in the game window.");
        }
    }
    if buttons.just_released(MouseButton::Left) {
        if let Some(position) = q_windows.single().cursor_position() {
            if position.x > mouse_coords.pre_x && position.y > mouse_coords.pre_y {
                transform.translation.x -= position.x - mouse_coords.pre_x;
                transform.translation.y += position.y - mouse_coords.pre_y;
            }
            if position.x > mouse_coords.pre_x && position.y < mouse_coords.pre_y {
                transform.translation.x -= position.x - mouse_coords.pre_x;
                transform.translation.y -= mouse_coords.pre_y - position.y;
            }
            if position.x < mouse_coords.pre_x && position.y > mouse_coords.pre_y {
                transform.translation.x += mouse_coords.pre_x - position.x;
                transform.translation.y += position.y - mouse_coords.pre_y;
            }
            if position.x < mouse_coords.pre_x && position.y < mouse_coords.pre_y {
                transform.translation.x += mouse_coords.pre_x - position.x;
                transform.translation.y -= mouse_coords.pre_y - position.y;
            }
        } else {
            info!("Cursor is not in the game window.");
        }
    }
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
