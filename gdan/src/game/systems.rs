use bevy::math::bounding::*;
use bevy::prelude::*;

pub fn camera2dbundle(mut commands: Commands) {
    info!("camera2dbundle");
    commands.spawn((Camera2dBundle::default(), crate::game::entities::GameMenu));
}

pub fn game_setup(mut commands: Commands, loader: Res<AssetServer>) {
    commands.spawn((
        SpatialBundle {
            transform: Transform::from_xyz(-125., 75., 0.),
            ..default()
        },
        crate::game::components::Shape::Circle(Circle::new(45.)),
        crate::game::components::DesiredVolume::Aabb,
        crate::game::components::Intersects::default(),
        crate::game::entities::GameMenu,
    ));

    commands.spawn((
        SpatialBundle {
            transform: Transform::from_xyz(0., 75., 0.),
            ..default()
        },
        crate::game::components::Shape::Rectangle(Rectangle::new(80., 80.)),
        crate::game::components::Spin,
        crate::game::components::DesiredVolume::Circle,
        crate::game::components::Intersects::default(),
        crate::game::entities::GameMenu,
    ));

    commands.spawn((
        SpatialBundle {
            transform: Transform::from_xyz(125., 75., 0.),
            ..default()
        },
        crate::game::components::Shape::Triangle(Triangle2d::new(
            Vec2::new(-40., -40.),
            Vec2::new(-20., 40.),
            Vec2::new(40., 50.),
        )),
        crate::game::components::Spin,
        crate::game::components::DesiredVolume::Aabb,
        crate::game::components::Intersects::default(),
        crate::game::entities::GameMenu,
    ));

    commands.spawn((
        SpatialBundle {
            transform: Transform::from_xyz(-125., -75., 0.),
            ..default()
        },
        crate::game::components::Shape::Line(Segment2d::new(
            Direction2d::from_xy(1., 0.3).unwrap(),
            90.,
        )),
        crate::game::components::Spin,
        crate::game::components::DesiredVolume::Circle,
        crate::game::components::Intersects::default(),
        crate::game::entities::GameMenu,
    ));

    commands.spawn((
        SpatialBundle {
            transform: Transform::from_xyz(0., -75., 0.),
            ..default()
        },
        crate::game::components::Shape::Capsule(Capsule2d::new(25., 50.)),
        crate::game::components::Spin,
        crate::game::components::DesiredVolume::Aabb,
        crate::game::components::Intersects::default(),
        crate::game::entities::GameMenu,
    ));

    commands.spawn((
        SpatialBundle {
            transform: Transform::from_xyz(125., -75., 0.),
            ..default()
        },
        crate::game::components::Shape::Polygon(RegularPolygon::new(50., 6)),
        crate::game::components::Spin,
        crate::game::components::DesiredVolume::Circle,
        crate::game::components::Intersects::default(),
        crate::game::entities::GameMenu,
    ));

    commands.spawn((
        TextBundle::from_section(
            "",
            TextStyle {
                font: loader.load("fonts/FiraMono-Medium.ttf"),
                font_size: 26.0,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
        crate::game::entities::GameMenu,
    ));
}

pub fn render_oper(
    mut gizmos: Gizmos,
    query: Query<(&crate::game::components::Shape, &Transform)>,
) {
    let color = Color::GRAY;
    for (shape, transform) in query.iter() {
        let translation = transform.translation.xy();
        let rotation = transform.rotation.to_euler(EulerRot::YXZ).2;
        match shape {
            crate::game::components::Shape::Rectangle(r) => {
                gizmos.primitive_2d(*r, translation, rotation, color);
            }
            crate::game::components::Shape::Circle(c) => {
                gizmos.primitive_2d(*c, translation, rotation, color);
            }
            crate::game::components::Shape::Triangle(t) => {
                gizmos.primitive_2d(*t, translation, rotation, color);
            }
            crate::game::components::Shape::Line(l) => {
                gizmos.primitive_2d(*l, translation, rotation, color);
            }
            crate::game::components::Shape::Capsule(c) => {
                gizmos.primitive_2d(*c, translation, rotation, color);
            }
            crate::game::components::Shape::Polygon(p) => {
                gizmos.primitive_2d(*p, translation, rotation, color);
            }
        }
    }
}

pub fn render_volumes(
    mut gizmos: Gizmos,
    query: Query<(
        &crate::game::components::CurrentVolume,
        &crate::game::components::Intersects,
    )>,
) {
    for (volume, intersects) in query.iter() {
        let color = if **intersects {
            Color::CYAN
        } else {
            Color::ORANGE_RED
        };
        match volume {
            crate::game::components::CurrentVolume::Aabb(a) => {
                gizmos.rect_2d(a.center(), 0., a.half_size() * 2., color);
            }
            crate::game::components::CurrentVolume::Circle(c) => {
                gizmos.circle_2d(c.center(), c.radius(), color);
            }
        }
    }
}

pub fn update_text(
    mut text: Query<&mut Text>,
    cur_state: Res<State<crate::game::entities::GameState>>,
) {
    // if !cur_state.is_changed() {
    //     return;
    // }

    let mut text = text.single_mut();
    let text = &mut text.sections[0].value;
    text.clear();

    text.push_str("Intersection test:\n");
    use crate::game::entities::GameState::*;
    for &state in &[AabbSweep, CircleSweep, RayCast, AabbCast, CircleCast] {
        let s = if **cur_state == state { "*" } else { " " };
        text.push_str(&format!(" {s} {state:?} {s}\n"));
    }
    text.push_str("\npress Space to cycle");
}

pub fn spin(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<crate::game::components::Spin>>,
) {
    for mut transform in query.iter_mut() {
        transform.rotation *= Quat::from_rotation_z(time.delta_seconds() / 5.);
    }
}

pub fn update_volumes(
    mut commands: Commands,
    query: Query<
        (
            Entity,
            &crate::game::components::DesiredVolume,
            &crate::game::components::Shape,
            &Transform,
        ),
        Or<(
            Changed<crate::game::components::DesiredVolume>,
            Changed<crate::game::components::Shape>,
            Changed<Transform>,
        )>,
    >,
) {
    for (entity, desired_volume, shape, transform) in query.iter() {
        let translation = transform.translation.xy();
        let rotation = transform.rotation.to_euler(EulerRot::YXZ).2;
        match desired_volume {
            crate::game::components::DesiredVolume::Aabb => {
                let aabb = match shape {
                    crate::game::components::Shape::Rectangle(r) => {
                        r.aabb_2d(translation, rotation)
                    }
                    crate::game::components::Shape::Circle(c) => c.aabb_2d(translation, rotation),
                    crate::game::components::Shape::Triangle(t) => t.aabb_2d(translation, rotation),
                    crate::game::components::Shape::Line(l) => l.aabb_2d(translation, rotation),
                    crate::game::components::Shape::Capsule(c) => c.aabb_2d(translation, rotation),
                    crate::game::components::Shape::Polygon(p) => p.aabb_2d(translation, rotation),
                };
                commands
                    .entity(entity)
                    .insert(crate::game::components::CurrentVolume::Aabb(aabb));
            }
            crate::game::components::DesiredVolume::Circle => {
                let circle = match shape {
                    crate::game::components::Shape::Rectangle(r) => {
                        r.bounding_circle(translation, rotation)
                    }
                    crate::game::components::Shape::Circle(c) => {
                        c.bounding_circle(translation, rotation)
                    }
                    crate::game::components::Shape::Triangle(t) => {
                        t.bounding_circle(translation, rotation)
                    }
                    crate::game::components::Shape::Line(l) => {
                        l.bounding_circle(translation, rotation)
                    }
                    crate::game::components::Shape::Capsule(c) => {
                        c.bounding_circle(translation, rotation)
                    }
                    crate::game::components::Shape::Polygon(p) => {
                        p.bounding_circle(translation, rotation)
                    }
                };
                commands
                    .entity(entity)
                    .insert(crate::game::components::CurrentVolume::Circle(circle));
            }
        }
    }
}

pub fn update_test_state(
    keycode: Res<ButtonInput<KeyCode>>,
    cur_state: Res<State<crate::game::entities::GameState>>,
    mut state: ResMut<NextState<crate::game::entities::GameState>>,
) {
    if !keycode.just_pressed(KeyCode::Space) {
        return;
    }
    info!("update_test_state");
    use crate::game::entities::GameState::*;
    let next = match **cur_state {
        AabbSweep => CircleSweep,
        CircleSweep => RayCast,
        RayCast => AabbCast,
        AabbCast => CircleCast,
        CircleCast => AabbSweep,
    };
    state.set(next);
}

pub fn aabb_intersection_system(
    mut gizmos: Gizmos,
    time: Res<Time>,
    mut volumes: Query<(
        &crate::game::components::CurrentVolume,
        &mut crate::game::components::Intersects,
    )>,
) {
    let center = get_intersection_position(&time);
    let aabb = Aabb2d::new(center, Vec2::splat(50.));
    gizmos.rect_2d(center, 0., aabb.half_size() * 2., Color::YELLOW);

    for (volume, mut intersects) in volumes.iter_mut() {
        let hit = match volume {
            crate::game::components::CurrentVolume::Aabb(a) => aabb.intersects(a),
            crate::game::components::CurrentVolume::Circle(c) => aabb.intersects(c),
        };

        **intersects = hit;
    }
}

pub fn circle_intersection_system(
    mut gizmos: Gizmos,
    time: Res<Time>,
    mut volumes: Query<(
        &crate::game::components::CurrentVolume,
        &mut crate::game::components::Intersects,
    )>,
) {
    let center = get_intersection_position(&time);
    let circle = BoundingCircle::new(center, 50.);
    gizmos.circle_2d(center, circle.radius(), Color::YELLOW);

    for (volume, mut intersects) in volumes.iter_mut() {
        let hit = match volume {
            crate::game::components::CurrentVolume::Aabb(a) => circle.intersects(a),
            crate::game::components::CurrentVolume::Circle(c) => circle.intersects(c),
        };

        **intersects = hit;
    }
}

pub fn get_intersection_position(time: &Time) -> Vec2 {
    let x = (0.8 * time.elapsed_seconds()).cos() * 250.;
    let y = (0.4 * time.elapsed_seconds()).sin() * 100.;
    Vec2::new(x, y)
}

pub fn bounding_circle_cast_system(
    mut gizmos: Gizmos,
    time: Res<Time>,
    mut volumes: Query<(
        &crate::game::components::CurrentVolume,
        &mut crate::game::components::Intersects,
    )>,
) {
    let ray_cast = get_and_draw_ray(&mut gizmos, &time);
    let circle_cast = BoundingCircleCast {
        circle: BoundingCircle::new(Vec2::ZERO, 15.),
        ray: ray_cast,
    };

    for (volume, mut intersects) in volumes.iter_mut() {
        let toi = match *volume {
            crate::game::components::CurrentVolume::Aabb(_) => None,
            crate::game::components::CurrentVolume::Circle(c) => circle_cast.circle_collision_at(c),
        };

        **intersects = toi.is_some();
        if let Some(toi) = toi {
            gizmos.circle_2d(
                circle_cast.ray.ray.origin
                    + *circle_cast.ray.ray.direction * toi
                    + circle_cast.circle.center(),
                circle_cast.circle.radius(),
                Color::GREEN,
            );
        }
    }
}

pub fn aabb_cast_system(
    mut gizmos: Gizmos,
    time: Res<Time>,
    mut volumes: Query<(
        &crate::game::components::CurrentVolume,
        &mut crate::game::components::Intersects,
    )>,
) {
    let ray_cast = get_and_draw_ray(&mut gizmos, &time);
    let aabb_cast = AabbCast2d {
        aabb: Aabb2d::new(Vec2::ZERO, Vec2::splat(15.)),
        ray: ray_cast,
    };

    for (volume, mut intersects) in volumes.iter_mut() {
        let toi = match *volume {
            crate::game::components::CurrentVolume::Aabb(a) => aabb_cast.aabb_collision_at(a),
            crate::game::components::CurrentVolume::Circle(_) => None,
        };

        **intersects = toi.is_some();
        if let Some(toi) = toi {
            gizmos.rect_2d(
                aabb_cast.ray.ray.origin
                    + *aabb_cast.ray.ray.direction * toi
                    + aabb_cast.aabb.center(),
                0.,
                aabb_cast.aabb.half_size() * 2.,
                Color::GREEN,
            );
        }
    }
}

pub fn ray_cast_system(
    mut gizmos: Gizmos,
    time: Res<Time>,
    mut volumes: Query<(
        &crate::game::components::CurrentVolume,
        &mut crate::game::components::Intersects,
    )>,
) {
    let ray_cast = get_and_draw_ray(&mut gizmos, &time);

    for (volume, mut intersects) in volumes.iter_mut() {
        let toi = match volume {
            crate::game::components::CurrentVolume::Aabb(a) => ray_cast.aabb_intersection_at(a),
            crate::game::components::CurrentVolume::Circle(c) => ray_cast.circle_intersection_at(c),
        };
        **intersects = toi.is_some();
        if let Some(toi) = toi {
            for r in [1., 2., 3.] {
                gizmos.circle_2d(
                    ray_cast.ray.origin + *ray_cast.ray.direction * toi,
                    r,
                    Color::GREEN,
                );
            }
        }
    }
}

pub fn get_and_draw_ray(gizmos: &mut Gizmos, time: &Time) -> RayCast2d {
    let ray = Vec2::new(time.elapsed_seconds().cos(), time.elapsed_seconds().sin());
    let dist = 150. + (0.5 * time.elapsed_seconds()).sin().abs() * 500.;

    let aabb_ray = Ray2d {
        origin: ray * 250.,
        direction: Direction2d::new_unchecked(-ray),
    };
    let ray_cast = RayCast2d::from_ray(aabb_ray, dist - 20.);

    draw_ray(gizmos, &ray_cast);
    ray_cast
}

pub fn draw_ray(gizmos: &mut Gizmos, ray: &RayCast2d) {
    gizmos.line_2d(
        ray.ray.origin,
        ray.ray.origin + *ray.ray.direction * ray.max,
        Color::WHITE,
    );
    for r in [1., 2., 3.] {
        gizmos.circle_2d(ray.ray.origin, r, Color::FUCHSIA);
    }
}

pub fn despawn_game_menu(
    query_enemy: Query<Entity, With<crate::game::entities::GameMenu>>,
    mut commands: Commands,
) {
    info!("despawn_game_menu");
    for entity_id in query_enemy.iter() {
        // commands.entity(entity_id).remove::<MainInfo>();
        commands.entity(entity_id).despawn();
        // .insert(Friendly);
    }
}
