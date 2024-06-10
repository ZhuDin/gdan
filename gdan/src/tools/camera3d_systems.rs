use bevy::prelude::*;

pub fn projection3d_zoom(
    mut query_camera: Query<&mut Projection>,
    mut scroll_evr: EventReader<bevy::input::mouse::MouseWheel>,
) {
    for ev in scroll_evr.read() {
        // 3D cameras can use either a Perspective or an Orthographic projection. Perspective is the default
        let Projection::Perspective(persp) = query_camera.single_mut().into_inner() else {
            info!("Orthographic");
            return;
        };
        match ev.unit {
            bevy::input::mouse::MouseScrollUnit::Line => {
                // info!(
                //     "Scroll (line units): vertical: {}, horizontal: {}",
                //     ev.y, ev.x
                // );
                if ev.y > 0. && persp.fov < 2. {
                    persp.fov *= 1.1;
                    info!("persp.fov {}", persp.fov);
                } else if ev.y < 0. && persp.fov > 0.2 {
                    persp.fov /= 1.1;
                    info!("persp.fov {}", persp.fov);
                }
            }
            bevy::input::mouse::MouseScrollUnit::Pixel => {
                info!(
                    "Scroll (pixel units): vertical: {}, horizontal: {}",
                    ev.y, ev.x
                );
            }
        }
    }
}

pub fn transform_location(
    mut query_camera3d_transform: Query<&mut Transform, With<Camera3d>>,
    buttons: Res<ButtonInput<MouseButton>>,
    time: Res<Time>,
) {
    let mut transform = query_camera3d_transform.single_mut();
    transform.rotate_around(Vec3::ZERO, Quat::from_rotation_z(time.delta_seconds() / 2.));
    if buttons.just_pressed(MouseButton::Left) {
        info!("just_pressed MouseButton Left");
        transform.translation.x += 1.;
    }
}
