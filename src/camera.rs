use std::f32::consts::TAU;

use crate::cube::*;
use crate::moving::*;
use bevy::input::mouse::MouseMotion;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use bevy_mod_picking::PickingCameraBundle;
use bevy_mod_raycast::Intersection;
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

pub fn move_camera(
    mut q_camera: Query<&mut Transform, With<Camera>>,
    cube_settings: Res<CubeSettings>,
    mut motion_evr: EventReader<MouseMotion>,
    buttons: Res<Input<MouseButton>>,
    q_intersection: Query<&Intersection<MyRaycastSet>>,
) {
    if buttons.pressed(MouseButton::Left) {
        let inter = q_intersection.iter().last();
        // println!("inter: {:?}", inter);
        if inter.is_none() || inter.unwrap().position().is_none() {
            for motion in motion_evr.iter() {
                for mut transform in &mut q_camera {
                    println!("transform.local_y(): {:?}, motion.delta: {:?}", transform.local_y(), motion.delta);
                    let axis = (transform.local_y() * motion.delta.extend(0.0)).normalize();
                    println!("axis: {:?}", axis);
                    transform.rotate_around(Vec3::ZERO, Quat::from_axis_angle(axis, 0.001 * TAU * motion.delta.x));
                }
            }
        }
    }
}
