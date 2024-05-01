// Entities: a simple type containing a unique integer
use bevy::ecs::component::Component;

/*
 * Component types that are empty structs (contain no data) are called marker components.
 * They are useful as "tags" to identify specific entities, or enable certain behaviors.
 */

#[derive(Component)]
pub struct MapInfo;
