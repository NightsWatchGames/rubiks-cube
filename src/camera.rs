use std::f32::consts::TAU;

use crate::cube::*;
use crate::moving::*;
use bevy::input::mouse::MouseMotion;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use bevy_mod_picking::backend::PointerHits;
use bevy_mod_picking::prelude::*;

pub fn setup_camera(mut commands: Commands) {
    // camera
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(RaycastPickCamera::default());
}

// TODO 平滑放大缩小 参考 https://github.com/cart/card_combinator/blob/main/src/game/camera.rs
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
                    transform.translation.x =
                        cube_settings.camera_zoom_speed * transform.translation.x;
                    transform.translation.y =
                        cube_settings.camera_zoom_speed * transform.translation.y;
                    transform.translation.z =
                        cube_settings.camera_zoom_speed * transform.translation.z;
                } else {
                    transform.translation.x =
                        transform.translation.x / cube_settings.camera_zoom_speed;
                    transform.translation.y =
                        transform.translation.y / cube_settings.camera_zoom_speed;
                    transform.translation.z =
                        transform.translation.z / cube_settings.camera_zoom_speed;
                }
            }
            MouseScrollUnit::Pixel => {
                if ev.x + ev.y > 0.0 {
                    transform.translation.x =
                        cube_settings.camera_zoom_speed * transform.translation.x;
                    transform.translation.y =
                        cube_settings.camera_zoom_speed * transform.translation.y;
                    transform.translation.z =
                        cube_settings.camera_zoom_speed * transform.translation.z;
                } else {
                    transform.translation.x =
                        transform.translation.x / cube_settings.camera_zoom_speed;
                    transform.translation.y =
                        transform.translation.y / cube_settings.camera_zoom_speed;
                    transform.translation.z =
                        transform.translation.z / cube_settings.camera_zoom_speed;
                }
            }
        }
    }
}

pub fn move_camera(
    mut q_camera: Query<&mut Transform, With<Camera>>,
    mut motion_evr: EventReader<MouseMotion>,
    buttons: Res<Input<MouseButton>>,
    recorder: Res<MouseDraggingRecorder>,
) {
    if buttons.pressed(MouseButton::Left) {
        if recorder.piece.is_none() || recorder.start_pos.is_none() {
            println!("move camera");
            for motion in motion_evr.iter() {
                // motion.delta.x 鼠标左滑为负、右滑为正
                // motion.delta.y 鼠标上滑为负、下滑为正
                for mut transform in &mut q_camera {
                    // println!("camera translation: {}, motion.delta: {}", transform.translation, motion.delta);
                    if motion.delta.x.abs() > 0.001 {
                        // 水平转动，相机只需要围绕y轴转动即可
                        let max = transform
                            .translation
                            .x
                            .abs()
                            .max(transform.translation.y.abs())
                            .max(transform.translation.z.abs());
                        let quat = Quat::from_euler(
                            EulerRot::XYZ,
                            0.0,
                            0.0002 * -motion.delta.x * max * TAU, // 乘以max是为了跟上下滑动保持相同速率
                            0.0,
                        );
                        transform.rotate_around(Vec3::ZERO, quat);
                    }
                    if motion.delta.y.abs() > 0.001 {
                        // 垂直转动，需要同时围绕x轴和z轴转动，而且转动角度跟与坐标轴夹角角度成反比
                        let quat = Quat::from_euler(
                            EulerRot::XYZ,
                            0.0002 * -motion.delta.y * transform.translation.z * TAU,
                            0.0,
                            0.0002 * motion.delta.y * transform.translation.x * TAU,
                        );
                        transform.rotate_around(Vec3::ZERO, quat);
                    }
                }
            }
        }
    }
}
