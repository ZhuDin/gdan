use bevy::prelude::*;
use std::f32::consts::*;

pub fn camera2dbundle(mut commands: Commands) {
    info!("camera2dbundle");
    commands.spawn((Camera2dBundle::default(), crate::rule::entities::RuleMenu));
}

pub fn draw_rule(
    mut gizmos: Gizmos,
    mut my_gizmos: Gizmos<crate::rule::entities::MyRoundGizmos>,
    time: Res<Time>,
) {
    let sin = time.elapsed_seconds().sin() * 50.;
    gizmos.line_2d(Vec2::Y * -sin, Vec2::splat(-80.), Color::RED);
    gizmos.ray_2d(Vec2::Y * sin, Vec2::splat(80.), Color::GREEN);

    // Triangle
    gizmos.linestrip_gradient_2d([
        (Vec2::Y * 300., Color::BLUE),
        (Vec2::new(-255., -155.), Color::RED),
        (Vec2::new(255., -155.), Color::GREEN),
        (Vec2::Y * 300., Color::BLUE),
    ]);

    gizmos.rect_2d(
        Vec2::ZERO,
        time.elapsed_seconds() / 3.,
        Vec2::splat(300.),
        Color::BLACK,
    );

    // The circles have 32 line-segments by default.
    my_gizmos.circle_2d(Vec2::ZERO, 120., Color::BLACK);
    my_gizmos.ellipse_2d(
        Vec2::ZERO,
        time.elapsed_seconds() % TAU,
        Vec2::new(100., 200.),
        Color::YELLOW_GREEN,
    );
    // You may want to increase this for larger circles.
    my_gizmos
        .circle_2d(Vec2::ZERO, 300., Color::NAVY)
        .segments(64);

    // Arcs default amount of segments is linearly interpolated between
    // 1 and 32, using the arc length as scalar.
    my_gizmos.arc_2d(Vec2::ZERO, sin / 10., PI / 2., 350., Color::ORANGE_RED);

    gizmos.arrow_2d(
        Vec2::ZERO,
        Vec2::from_angle(sin / -10. + PI / 2.) * 50.,
        Color::YELLOW,
    );
}

pub fn draw_cursor(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window, With<bevy::window::PrimaryWindow>>,
    mut gizmos: Gizmos,
) {
    let (camera, camera_transform) = camera_query.single();

    let Some(cursor_position) = windows.single().cursor_position() else {
        return;
    };

    // Calculate a world position based on the cursor's position.
    let Some(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };

    gizmos.circle_2d(point, 10., Color::WHITE);
}

pub fn despawn_rule_menu(
    query_enemy: Query<Entity, With<crate::rule::entities::RuleMenu>>,
    mut commands: Commands,
) {
    info!("despawn_rule_menu");
    for entity_id in query_enemy.iter() {
        // commands.entity(entity_id).remove::<MainInfo>();
        commands.entity(entity_id).despawn();
        // .insert(Friendly);
    }
}
