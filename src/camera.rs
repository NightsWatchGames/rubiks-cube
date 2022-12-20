use crate::cube::*;
use crate::moving::*;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use bevy_mod_picking::PickingCameraBundle;
use bevy_mod_raycast::RaycastSource;

pub fn setup_camera(
    mut commands: Commands,
) {
    // camera
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(PickingCameraBundle::default())
        .insert(RaycastSource::<MyRaycastSet>::new());
}

pub fn zoom_camera(
    mut scroll_evr: EventReader<MouseWheel>,
    mut q_camera: Query<&mut Transform, With<Camera>>,
    cube_settings: Res<CubeSettings>,
) {
    for ev in scroll_evr.iter() {
        let mut transform = q_camera.single_mut();
        match ev.unit {
            MouseScrollUnit::Line => {
                if ev.x + ev.y > 0.0 {
                    transform.translation.x = cube_settings.camera_zoom_speed * transform.translation.x;
                    transform.translation.y = cube_settings.camera_zoom_speed * transform.translation.y;
                    transform.translation.z = cube_settings.camera_zoom_speed * transform.translation.z;
                } else {
                    transform.translation.x = transform.translation.x / cube_settings.camera_zoom_speed;
                    transform.translation.y = transform.translation.y / cube_settings.camera_zoom_speed;
                    transform.translation.z = transform.translation.z / cube_settings.camera_zoom_speed;
                }
            }
            MouseScrollUnit::Pixel => {
                if ev.x + ev.y > 0.0 {
                    transform.translation.x = cube_settings.camera_zoom_speed * transform.translation.x;
                    transform.translation.y = cube_settings.camera_zoom_speed * transform.translation.y;
                    transform.translation.z = cube_settings.camera_zoom_speed * transform.translation.z;
                } else {
                    transform.translation.x = transform.translation.x / cube_settings.camera_zoom_speed;
                    transform.translation.y = transform.translation.y / cube_settings.camera_zoom_speed;
                    transform.translation.z = transform.translation.z / cube_settings.camera_zoom_speed;
                }
            }
        }
    }
}
