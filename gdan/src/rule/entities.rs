use bevy::prelude::*;

#[derive(Component)]
pub struct RuleMenu;

// We can create our own gizmo config group!
#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct MyRoundGizmos {}
