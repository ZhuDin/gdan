pub mod game;
pub mod map;
pub mod oper;
pub mod rule;
pub mod scene;

use bevy::app::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct MainInfo;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum MyAppState {
    #[default]
    MainMenu,
    MapMenu,
    // OperMenu,
    // RuleMenu,
    // SceneMenu,
    // GameMenu,
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
        .add_plugins(DefaultPlugins)
        .insert_resource(crate::map::resources::GreetTimer(Timer::from_seconds(
            5.0,
            TimerMode::Repeating,
        )))
        // Register the state type(s) in the app builder
        // Specify the initial value:
        // .insert_state(MyAppState::Main)
        // Or use the default (if the type impls Default):
        .init_state::<MyAppState>()
        // .add_systems(Startup, (w_game_setup,).chain())
        .add_systems(Update, bevy::window::close_on_esc)
        /*
         * MainMenu
         * Note that we have used .chain() on the systems.
         * This is because we want them to run in exactly the order they're listed in the code.
         */
        .add_systems(OnEnter(MyAppState::MainMenu), (w_game_setup,).chain())
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
            (crate::map::systems::add_map,).chain(),
        )
        .add_systems(
            Update,
            (
                crate::map::systems::show_map,
                crate::map::systems::map_scale,
            )
                .chain()
                .run_if(in_state(MyAppState::MapMenu)),
        )
        .add_systems(
            OnExit(MyAppState::MapMenu),
            (crate::map::systems::despawn_map_menu,),
        )
        .run();
    /*
     * Controlling data in terms of specific resources or components and
     * adding systems to an existing schedule.
     */
}

fn w_game_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("w_game_setup");

    commands.spawn((Camera2dBundle::default(), MainInfo));

    commands.spawn((
        TextBundle::from_section(
            "w_game",
            TextStyle {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: 18.,
                color: Color::WHITE,
            },
        ),
        MainInfo,
    ));

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            MainInfo,
        ))
        .with_children(|parent| {
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
                    MainInfo,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            "Map",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        ),
                        MainInfo,
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
            // &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    state: Res<State<MyAppState>>,
    mut next_state: ResMut<NextState<MyAppState>>,
    // mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, mut border_color /*, children*/) in &mut interaction_query {
        // let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                // text.sections[0].value = "Press".to_string();
                *color = Color::rgb(0.35, 0.75, 0.35).into();
                border_color.0 = Color::RED;

                match state.get() {
                    MyAppState::MainMenu => {
                        next_state.set(MyAppState::MapMenu);
                        info!("w_game_system -> MyAppState::LoadingMap");
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

// You can remove components on existing entities, using Commands or exclusive World access.
fn despawn_main_menu(query_enemy: Query<Entity, With<MainInfo>>, mut commands: Commands) {
    info!("despawn_main_menu");
    for entity_id in query_enemy.iter() {
        // commands.entity(entity_id).remove::<MainInfo>();
        commands.entity(entity_id).despawn();
        // .insert(Friendly);
    }
}

// fn exit_system(mut exit: EventWriter<AppExit>, keyboard_input: Res<ButtonInput<KeyCode>>) {
//     if keyboard_input.just_released(KeyCode::Escape) {
//         exit.send(AppExit);
//     }
// }
