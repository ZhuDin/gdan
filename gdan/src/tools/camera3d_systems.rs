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
                    persp.fov /= 1.1;
                } else if ev.y < 0. && persp.fov > 0.2 {
                    persp.fov *= 1.1;
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

pub fn camera_location(
    mut query_camera3d_transform: Query<&mut Transform, With<Camera3d>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut camera_transform = query_camera3d_transform.single_mut();

    if keyboard.pressed(KeyCode::KeyA) {
        camera_transform.translation.x -= 1.;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        camera_transform.translation.x += 1.;
    }
    if keyboard.pressed(KeyCode::KeyW) {
        camera_transform.translation.y += 1.;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        camera_transform.translation.y -= 1.;
    }
    if keyboard.pressed(KeyCode::KeyQ) {
        camera_transform.translation.z += 1.;
    }
    if keyboard.pressed(KeyCode::KeyE) {
        camera_transform.translation.z -= 1.;
    }
    if mouse_buttons.pressed(MouseButton::Right) {
        camera_transform
            .rotate_around(Vec3::ZERO, Quat::from_rotation_z(time.delta_seconds() / 2.));
    }
}
