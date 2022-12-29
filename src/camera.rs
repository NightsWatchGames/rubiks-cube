use std::f32::consts::TAU;

use crate::cube::*;
use crate::moving::*;
use bevy::input::mouse::MouseMotion;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use bevy_mod_picking::PickingCameraBundle;
use bevy_mod_raycast::Intersection;
use bevy_mod_raycast::RaycastSource;

pub fn setup_camera(mut commands: Commands) {
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
                // motion.delta.x 鼠标左滑为负、右滑为正
                // motion.delta.y 鼠标上滑为负、下滑为正
                for mut transform in &mut q_camera {
                    println!("camera translation: {}", transform.translation);
                    let mut hitted = 0;
                    // 相机在魔方前面
                    if transform.translation.z >= cube_settings.coordinate_boundary()
                        && transform.translation.z.abs() / transform.translation.x.abs() >= 1.0
                        && transform.translation.z.abs() / transform.translation.y.abs() >= 1.0
                    {
                        hitted += 1;
                        println!("Forward face");
                        if motion.delta.x.abs() > 0.001 {
                            transform.rotate_around(
                                Vec3::ZERO,
                                Quat::from_axis_angle(Vec3::Y, 0.001 * -motion.delta.x * TAU),
                            );
                        }
                        if motion.delta.y.abs() > 0.001 {
                            transform.rotate_around(
                                Vec3::ZERO,
                                Quat::from_axis_angle(Vec3::X, 0.001 * -motion.delta.y * TAU),
                            );
                        }
                    }
                    // 相机在魔方后面
                    else if transform.translation.z <= -cube_settings.coordinate_boundary()
                        && transform.translation.z.abs() / transform.translation.x.abs() >= 1.0
                        && transform.translation.z.abs() / transform.translation.y.abs() >= 1.0
                    {
                        hitted += 1;
                        println!("Back face {}, {}", transform.translation.z.abs() / transform.translation.x.abs(), transform.translation.z.abs() / transform.translation.y.abs());
                        if motion.delta.x.abs() > 0.001 {
                            transform.rotate_around(
                                Vec3::ZERO,
                                Quat::from_axis_angle(Vec3::Y, 0.001 * -motion.delta.x * TAU),
                            );
                        }
                        if motion.delta.y.abs() > 0.001 {
                            transform.rotate_around(
                                Vec3::ZERO,
                                Quat::from_axis_angle(Vec3::X, 0.001 * motion.delta.y * TAU),
                            );
                        }
                    }
                    // 相机在魔方左面
                    else if transform.translation.x < -cube_settings.coordinate_boundary()
                        && transform.translation.x.abs() / transform.translation.z.abs() > 1.0
                        && transform.translation.x.abs() / transform.translation.y.abs() > 1.0
                    {
                        hitted += 1;
                        println!("Left face");
                        if motion.delta.x.abs() > 0.001 {
                            transform.rotate_around(
                                Vec3::ZERO,
                                Quat::from_axis_angle(Vec3::Y, 0.001 * -motion.delta.x * TAU),
                            );
                        }
                        if motion.delta.y.abs() > 0.001 {
                            transform.rotate_around(
                                Vec3::ZERO,
                                Quat::from_axis_angle(Vec3::Z, 0.001 * -motion.delta.y * TAU),
                            );
                        }
                    }
                    // 相机在魔方右面
                    else if transform.translation.x > cube_settings.coordinate_boundary()
                        && transform.translation.x.abs() / transform.translation.z.abs() > 1.0
                        && transform.translation.x.abs() / transform.translation.y.abs() > 1.0
                    {
                        hitted += 1;
                        println!("Right face");
                        if motion.delta.x.abs() > 0.001 {
                            transform.rotate_around(
                                Vec3::ZERO,
                                Quat::from_axis_angle(Vec3::Y, 0.001 * -motion.delta.x * TAU),
                            );
                        }
                        if motion.delta.y.abs() > 0.001 {
                            transform.rotate_around(
                                Vec3::ZERO,
                                Quat::from_axis_angle(Vec3::Z, 0.001 * motion.delta.y * TAU),
                            );
                        }
                    }
                    // 相机在魔方上面
                    else if transform.translation.y > cube_settings.coordinate_boundary()
                        && transform.translation.y.abs() / transform.translation.x.abs() > 1.0
                        && transform.translation.y.abs() / transform.translation.z.abs() > 1.0
                    {
                        hitted += 1;
                        dbg!(transform.translation.y.abs());
                        dbg!(transform.translation.z.abs());
                        println!("Up face, {}, {}", transform.translation.y.abs() / transform.translation.x.abs(), transform.translation.y.abs() / transform.translation.z.abs());
                        if motion.delta.x.abs() > 0.001 {
                            transform.rotate_around(
                                Vec3::ZERO,
                                Quat::from_axis_angle(Vec3::Z, 0.001 * -motion.delta.x * TAU),
                            );
                        }
                        if motion.delta.y.abs() > 0.001 {
                            transform.rotate_around(
                                Vec3::ZERO,
                                Quat::from_axis_angle(Vec3::X, 0.001 * motion.delta.y * TAU),
                            );
                        }
                    }
                    // 相机在魔方下面
                    else if transform.translation.y < -cube_settings.coordinate_boundary()
                        && transform.translation.y.abs() / transform.translation.x.abs() > 1.0
                        && transform.translation.y.abs() / transform.translation.z.abs() > 1.0
                    {
                        hitted += 1;
                        println!("Down face");
                        if motion.delta.x.abs() > 0.001 {
                            transform.rotate_around(
                                Vec3::ZERO,
                                Quat::from_axis_angle(Vec3::Z, 0.001 * -motion.delta.x * TAU),
                            );
                        }
                        if motion.delta.y.abs() > 0.001 {
                            transform.rotate_around(
                                Vec3::ZERO,
                                Quat::from_axis_angle(Vec3::X, 0.001 * -motion.delta.y * TAU),
                            );
                        }
                    }
                    // hitted 应该是1
                    println!("hitted: {}", hitted);
                }
            }
        }
    }
}
