// Entities: a simple type containing a unique integer
use bevy::ecs::component::Component;

/*
 * Component types that are empty structs (contain no data) are called marker components.
 * They are useful as "tags" to identify specific entities, or enable certain behaviors.
 * Conceptually, an entity represents a set of values for different components.
 * Each component is a Rust type (struct or enum) and an entity can be used to store a value of that type.
 * In Bevy, Entity is this value. It consists of two integers: the ID and
 * the "generation" (allowing IDs to be reused, after you despawn old entities).
 * You can create ("spawn") new entities and
 * destroy ("despawn") entities using Commands or exclusive World access.
 */

/*
 fn setup(mut commands: Commands) {
    // create a new entity
    commands.spawn((
        // Initialize all your components and bundles here
        Enemy,
        Health {
            hp: 100.0,
            extra: 25.0,
        },
        AiMode::Passive,
        // ...
    ));

    // If you want to get the Entity ID, just call `.id()` after spawn
    let my_entity = commands.spawn((/* ... */)).id();

    // destroy an entity, removing all data associated with it
    commands.entity(my_entity).despawn();
}
 */

#[derive(Component)]
pub struct MapMenu;

#[derive(Component)]
pub struct MapNC;

#[derive(Component)]
pub struct MapHexagon;
