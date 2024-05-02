pub mod game;
pub mod map;
pub mod oper;
pub mod rule;
pub mod scene;

use bevy::app::*;
use bevy::prelude::*;

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
            2.0,
            TimerMode::Repeating,
        )))
        .add_systems(
            Startup,
            (
                w_game,
                crate::map::systems::add_map,
                crate::map::systems::update_map_name,
            )
                .chain(),
        )
        /*
         * Note that we have used .chain() on the systems.
         * This is because we want them to run in exactly the order they're listed in the code.
         */
        .add_systems(
            Update,
            (crate::map::systems::show_map, bevy::window::close_on_esc).chain(),
        )
        .run();
    /*
     * Controlling data in terms of specific resources or components and
     * adding systems to an existing schedule.
     */
}

fn w_game() {
    info!("w_game");
}

// fn exit_system(mut exit: EventWriter<AppExit>, keyboard_input: Res<ButtonInput<KeyCode>>) {
//     if keyboard_input.just_released(KeyCode::Escape) {
//         exit.send(AppExit);
//     }
// }
