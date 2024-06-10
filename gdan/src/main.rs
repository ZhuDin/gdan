pub mod game;
pub mod map;
pub mod oper;
pub mod rule;
pub mod scene;
pub mod tools;

use bevy::prelude::*;

#[derive(bevy::ecs::component::Component)]
pub struct MainMenu;

// We can create our own gizmo config group!
#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct MyRoundGizmos {}

#[derive(bevy::ecs::schedule::States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum MyAppState {
    MainMenu,
    MapMenu,
    Map3D,
    OperMenu,
    Oper3D,
    RuleMenu,
    SceneMenu,
    #[default]
    Scene3D,
    GameMenu,
}

/*
 * All app logic in Bevy uses the Entity Component System paradigm, which is often shortened to ECS.
 * ECS is a software pattern that involves breaking your program up into Entities, Components, and Systems.
 * Entities are unique "things" that are assigned groups of Components, which are processed using Systems.
 * Bevy ECS is Bevy's implementation of the ECS pattern. Unlike other Rust ECS implementations,
 * which often require complex lifetimes, traits, builder patterns, or macros,
 * Bevy ECS uses normal Rust datatypes for all of these concepts.
 */
fn main() {
    /*
     * what sort of data does our App really store?
     * Looking at the docs linked, we find three fields: world, schedule, and runner.
     * The world field stores all of our game's data;
     * The schedule holds the systems that operate on this data (and the order in which they do so).
     * The runner interprets the schedule to control the broad execution strategy.
     */
    App::new()
        .add_plugins(bevy::DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(crate::map::resources::GreetTimer(
            bevy::time::Timer::from_seconds(5.0, bevy::time::TimerMode::Repeating),
        ))
        // Register the state type(s) in the app builder
        // Specify the initial value:
        // .insert_state(MyAppState::Main)
        // Or use the default (if the type impls Default):
        .init_state::<MyAppState>()
        .init_state::<crate::game::entities::GameState>()
        .init_gizmo_group::<crate::MyRoundGizmos>()
        // .add_systems(Startup, ().chain())
        .add_systems(Update, close_on_esc)
        /*
         * MainMenu
         * Note that we have used .chain() on the systems.
         * This is because we want them to run in exactly the order they're listed in the code.
         */
        .add_systems(
            OnEnter(MyAppState::MainMenu),
            (camera2dbundle, tips_info, w_game_setup).chain(),
        )
        .add_systems(
            Update,
            (w_game_system,)
                .chain()
                .run_if(in_state(MyAppState::MainMenu)),
        )
        .add_systems(OnExit(MyAppState::MainMenu), (despawn_main_menu,))
        /*
         * MapMenu
         */
        .add_systems(
            OnEnter(MyAppState::MapMenu),
            (
                crate::map::systems::init_map,
                crate::map::systems::camera2dbundle,
                crate::map::systems::map_menu,
                crate::map::systems::add_map,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                back_main_menu,
                crate::map::systems::draw_hexagon_2d,
                crate::map::systems::map2d_scale_wander,
                crate::map::systems::add_oper,
            )
                .chain()
                .run_if(in_state(MyAppState::MapMenu)),
        )
        .add_systems(
            OnExit(MyAppState::MapMenu),
            (crate::map::systems::despawn_map_menu,),
        )
        /*
         * Map3D
         */
        .add_systems(
            OnEnter(MyAppState::Map3D),
            (
                crate::map::systems::camera3dbundle,
                crate::map::systems::map_menu,
                crate::map::systems::add_map,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                back_main_menu,
                crate::map::systems::map_menu_system,
                crate::map::systems::map3d_scale_wander,
                crate::map::systems::draw_line_collection,
            )
                .chain()
                .run_if(in_state(MyAppState::Map3D)),
        )
        .add_systems(
            OnExit(MyAppState::Map3D),
            (crate::map::systems::despawn_map_menu,),
        )
        /*
         * OperMenu
         */
        .add_systems(
            OnEnter(MyAppState::OperMenu),
            (
                crate::oper::systems::camera2dbundle,
                crate::oper::systems::oper_setup,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (back_main_menu, crate::oper::systems::oper_menu_system)
                .chain()
                .run_if(in_state(MyAppState::OperMenu)),
        )
        .add_systems(
            OnExit(MyAppState::OperMenu),
            (crate::oper::systems::despawn_oper_menu,),
        )
        /*
         * Oper3D
         */
        .add_systems(
            OnEnter(MyAppState::Oper3D),
            (
                crate::oper::systems::camera3dbundle,
                crate::oper::systems::oper_setup,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                back_main_menu,
                crate::oper::systems::oper_menu_system,
                crate::oper::systems::oper3d_scale_wander,
            )
                .chain()
                .run_if(in_state(MyAppState::Oper3D)),
        )
        .add_systems(
            OnExit(MyAppState::Oper3D),
            (crate::oper::systems::despawn_oper_menu,),
        )
        /*
         * RuleMenu
         */
        .add_systems(
            OnEnter(MyAppState::RuleMenu),
            (crate::rule::systems::camera2dbundle,).chain(),
        )
        .add_systems(
            Update,
            (
                back_main_menu,
                crate::rule::systems::draw_rule,
                crate::rule::systems::draw_cursor,
            )
                .chain()
                .run_if(in_state(MyAppState::RuleMenu)),
        )
        .add_systems(
            OnExit(MyAppState::RuleMenu),
            (crate::rule::systems::despawn_rule_menu,),
        )
        /*
         * Scene3D
         */
        .add_systems(
            OnEnter(MyAppState::Scene3D),
            (
                crate::scene::systems::camera3dbundle,
                crate::scene::systems::show_map,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (back_main_menu, crate::rule::systems::draw_cursor)
                .chain()
                .run_if(in_state(MyAppState::Scene3D)),
        )
        .add_systems(
            OnExit(MyAppState::Scene3D),
            (crate::rule::systems::despawn_rule_menu,),
        )
        /*
         * GameMenu
         */
        .add_systems(
            OnEnter(MyAppState::GameMenu),
            (
                crate::game::systems::camera2dbundle,
                crate::game::systems::game_setup,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                back_main_menu,
                crate::game::systems::update_text,
                crate::game::systems::spin,
                crate::game::systems::update_volumes,
                crate::game::systems::update_test_state,
                crate::game::systems::render_oper,
                crate::game::systems::aabb_intersection_system
                    .run_if(in_state(crate::game::entities::GameState::AabbSweep)),
                crate::game::systems::circle_intersection_system
                    .run_if(in_state(crate::game::entities::GameState::CircleSweep)),
                crate::game::systems::ray_cast_system
                    .run_if(in_state(crate::game::entities::GameState::RayCast)),
                crate::game::systems::aabb_cast_system
                    .run_if(in_state(crate::game::entities::GameState::AabbCast)),
                crate::game::systems::bounding_circle_cast_system
                    .run_if(in_state(crate::game::entities::GameState::CircleCast)),
                crate::game::systems::render_volumes,
            )
                .chain()
                .run_if(in_state(MyAppState::GameMenu)),
        )
        .add_systems(
            OnExit(MyAppState::GameMenu),
            (crate::game::systems::despawn_game_menu,),
        )
        .run();
    /*
     * Controlling data in terms of specific resources or components and
     * adding systems to an existing schedule.
     */
}

pub fn camera2dbundle(mut commands: bevy::ecs::system::Commands) {
    info!("camera2dbundle");
    commands.spawn((
        bevy::core_pipeline::core_2d::Camera2dBundle::default(),
        MainMenu,
    ));
}

fn w_game_setup(
    mut commands: bevy::ecs::system::Commands,
    asset_server: bevy::ecs::system::Res<bevy::asset::AssetServer>,
) {
    info!("w_game_setup");

    commands
        .spawn((
            bevy::ui::node_bundles::NodeBundle {
                style: bevy::ui::Style {
                    width: bevy::ui::Val::Percent(100.0),
                    height: bevy::ui::Val::Percent(100.0),
                    align_items: bevy::ui::AlignItems::Center,
                    justify_content: bevy::ui::JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            MainMenu,
        ))
        .with_children(|parent| {
            /*
             * map Button
             */
            parent
                .spawn((
                    bevy::ui::node_bundles::ButtonBundle {
                        style: bevy::ui::Style {
                            width: bevy::ui::Val::Px(150.0),
                            height: bevy::ui::Val::Px(65.0),
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
                    MainMenu,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        bevy::ui::node_bundles::TextBundle::from_section(
                            "Map",
                            bevy::text::TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 40.0,
                                color: bevy::render::color::Color::rgb(0.9, 0.9, 0.9),
                            },
                        ),
                        MainMenu,
                    ));
                });

            /*
             * oper Button
             */
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(150.0),
                            height: Val::Px(65.0),
                            border: UiRect::all(Val::Px(5.0)),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                        ..default()
                    },
                    MainMenu,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            "Oper",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        ),
                        MainMenu,
                    ));
                });

            /*
             * rule Button
             */
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(150.0),
                            height: Val::Px(65.0),
                            border: UiRect::all(Val::Px(5.0)),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                        ..default()
                    },
                    MainMenu,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            "Rule",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        ),
                        MainMenu,
                    ));
                });

            /*
             * game Button
             */
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(150.0),
                            height: Val::Px(65.0),
                            border: UiRect::all(Val::Px(5.0)),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                        ..default()
                    },
                    MainMenu,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            "Game",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        ),
                        MainMenu,
                    ));
                });
        });
}

fn w_game_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    state: Res<State<MyAppState>>,
    mut next_state: ResMut<NextState<MyAppState>>,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                *color = Color::rgb(0.35, 0.75, 0.35).into();
                border_color.0 = Color::RED;

                match state.get() {
                    MyAppState::MainMenu => {
                        if text.sections[0].value == "Map".to_string() {
                            next_state.set(MyAppState::MapMenu);
                            info!("AppState::MapMenu");
                        }
                        if text.sections[0].value == "Oper".to_string() {
                            next_state.set(MyAppState::OperMenu);
                            info!("AppState::OperMenu");
                        }
                        if text.sections[0].value == "Rule".to_string() {
                            next_state.set(MyAppState::RuleMenu);
                            info!("AppState::RuleMenu");
                        }
                        if text.sections[0].value == "Game".to_string() {
                            next_state.set(MyAppState::GameMenu);
                            info!("AppState::GameMenu");
                        }
                    }
                    _ => (),
                }
            }
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

pub fn back_main_menu(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<crate::MyAppState>>,
) {
    if keyboard.just_pressed(KeyCode::KeyB) {
        next_state.set(crate::MyAppState::MainMenu);
        info!("AppState::MainMenu");
    }
}

pub fn tips_info(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("tips_info");
    commands.spawn((
        TextBundle::from_section(
            " press 'Esc' to Closing window\n press 'B' back to Main Menu",
            TextStyle {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: 24.,
                color: Color::WHITE,
            },
        ),
        MainMenu,
    ));
}

// You can remove components on existing entities, using Commands or exclusive World access.
fn despawn_main_menu(query_enemy: Query<Entity, With<MainMenu>>, mut commands: Commands) {
    info!("despawn_main_menu");
    for entity_id in query_enemy.iter() {
        // commands.entity(entity_id).remove::<MainInfo>();
        commands.entity(entity_id).despawn();
        // .insert(Friendly);
    }
}

pub fn close_on_esc(
    mut commands: Commands,
    state: Res<State<MyAppState>>,
    mut next_state: ResMut<NextState<MyAppState>>,
    focused_windows: Query<(Entity, &Window)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut exit: EventWriter<bevy::app::AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        match state.get() {
            MyAppState::MainMenu => {
                for (window, focus) in focused_windows.iter() {
                    if !focus.focused {
                        continue;
                    }
                    commands.entity(window).despawn();
                }
            }
            _ => {
                next_state.set(MyAppState::MainMenu);
                exit.send(bevy::app::AppExit);
            }
        }
        // std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
