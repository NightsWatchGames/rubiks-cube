use crate::cube::*;
use bevy::prelude::*;
use std::f32::consts::FRAC_PI_2;
use std::f32::consts::PI;

#[derive(Debug, Clone, Copy)]
pub enum SideRotation {
    Clockwise90,
    Clockwise180,
    Clockwise270,
    Counterclockwise90,
    Counterclockwise180,
    Counterclockwise270,
}

#[derive(Debug, Clone, Copy)]
pub enum Axis {
    X,
    Y,
    Z,
}

#[derive(Debug, Clone, Copy)]
pub struct SideMoveEvent {
    // 旋转的面，对应固定的x/y/z坐标值
    pub side: (Axis, f32),
    // 旋转
    pub rotate: SideRotation,
}

pub fn choose_cubes_from_side_move_event(
    mut commands: Commands,
    mut query: Query<(Entity, &Transform), With<Cube>>,
    mut side_move_event: EventReader<SideMoveEvent>,
) {
    for event in side_move_event.iter() {
        dbg!(event);
        for (_, transform) in &query {
            info!(
                "before choosing, cube translation={}",
                transform.translation
            );
        }
        let side = event.side;
        match side.0 {
            Axis::X => {
                for (entity, transform) in &mut query {
                    if transform.translation.x == side.1 {
                        if transform.translation.y == 0.0 && transform.translation.z == 0.0 {
                            info!("insert center: translation={}", transform.translation);
                            commands.entity(entity).insert(Center);
                        } else {
                            info!("insert movable cube: translation={}", transform.translation);
                            commands.entity(entity).insert(MovableCube);
                        }
                    }
                }
            }
            Axis::Y => {
                for (entity, transform) in &mut query {
                    if transform.translation.y == side.1 {
                        if transform.translation.x == 0.0 && transform.translation.z == 0.0 {
                            info!("insert center: translation={}", transform.translation);
                            commands.entity(entity).insert(Center);
                        } else {
                            info!("insert movable cube: translation={}", transform.translation);
                            commands.entity(entity).insert(MovableCube);
                        }
                    }
                }
            }
            Axis::Z => {
                for (entity, transform) in &mut query {
                    if transform.translation.z == side.1 {
                        if transform.translation.x == 0.0 && transform.translation.y == 0.0 {
                            info!("insert center: translation={}", transform.translation);
                            commands.entity(entity).insert(Center);
                        } else {
                            info!("insert movable cube: translation={}", transform.translation);
                            commands.entity(entity).insert(MovableCube);
                        }
                    }
                }
            }
        }
    }
}

pub fn rotate_cube(
    mut center: Query<(&Transform, &GlobalTransform), Added<Center>>,
    mut movable_cubes: Query<&mut Transform, (Without<Center>, With<MovableCube>)>,
    mut side_move_event: EventReader<SideMoveEvent>,
) {
    for (center, global_transform) in &mut center {
        for event in side_move_event.iter() {
            info!(
                "rotate - center transform={}, global transform={} event={:?}",
                center.translation,
                global_transform.translation(),
                event
            );
            for mut movable_cube in &mut movable_cubes {
                match event.rotate {
                    SideRotation::Clockwise90 => match event.side.0 {
                        Axis::X => {
                            movable_cube.rotate_around(
                                center.translation,
                                Quat::from_axis_angle(Vec3::X, FRAC_PI_2),
                            );
                        }
                        Axis::Y => {
                            movable_cube.rotate_around(
                                center.translation,
                                Quat::from_axis_angle(Vec3::Y, FRAC_PI_2),
                            );
                        }
                        Axis::Z => {
                            movable_cube.rotate_around(
                                center.translation,
                                Quat::from_axis_angle(Vec3::Z, FRAC_PI_2),
                            );
                        }
                    },
                    SideRotation::Clockwise180 => match event.side.0 {
                        Axis::X => {
                            movable_cube.rotate_around(
                                center.translation,
                                Quat::from_axis_angle(Vec3::X, PI),
                            );
                        }
                        Axis::Y => {
                            movable_cube.rotate_around(
                                center.translation,
                                Quat::from_axis_angle(Vec3::Y, PI),
                            );
                        }
                        Axis::Z => {
                            movable_cube.rotate_around(
                                center.translation,
                                Quat::from_axis_angle(Vec3::Z, PI),
                            );
                        }
                    },
                    SideRotation::Clockwise270 => match event.side.0 {
                        Axis::X => {
                            movable_cube.rotate_around(
                                center.translation,
                                Quat::from_axis_angle(Vec3::X, 3.0 * PI / 2.0),
                            );
                        }
                        Axis::Y => {
                            movable_cube.rotate_around(
                                center.translation,
                                Quat::from_axis_angle(Vec3::Y, 3.0 * PI / 2.0),
                            );
                        }
                        Axis::Z => {
                            movable_cube.rotate_around(
                                center.translation,
                                Quat::from_axis_angle(Vec3::Z, 3.0 * PI / 2.0),
                            );
                        }
                    },
                    SideRotation::Counterclockwise90 => match event.side.0 {
                        Axis::X => {
                            movable_cube.rotate_around(
                                center.translation,
                                Quat::from_axis_angle(Vec3::X, -FRAC_PI_2),
                            );
                        }
                        Axis::Y => {
                            movable_cube.rotate_around(
                                center.translation,
                                Quat::from_axis_angle(Vec3::Y, -FRAC_PI_2),
                            );
                        }
                        Axis::Z => {
                            movable_cube.rotate_around(
                                center.translation,
                                Quat::from_axis_angle(Vec3::Z, -FRAC_PI_2),
                            );
                        }
                    },
                    SideRotation::Counterclockwise180 => match event.side.0 {
                        Axis::X => {
                            movable_cube.rotate_around(
                                center.translation,
                                Quat::from_axis_angle(Vec3::X, -PI),
                            );
                        }
                        Axis::Y => {
                            movable_cube.rotate_around(
                                center.translation,
                                Quat::from_axis_angle(Vec3::Y, -PI),
                            );
                        }
                        Axis::Z => {
                            movable_cube.rotate_around(
                                center.translation,
                                Quat::from_axis_angle(Vec3::Z, -PI),
                            );
                        }
                    },
                    SideRotation::Counterclockwise270 => match event.side.0 {
                        Axis::X => {
                            movable_cube.rotate_around(
                                center.translation,
                                Quat::from_axis_angle(Vec3::X, -3.0 * PI / 2.0),
                            );
                        }
                        Axis::Y => {
                            movable_cube.rotate_around(
                                center.translation,
                                Quat::from_axis_angle(Vec3::Y, -3.0 * PI / 2.0),
                            );
                        }
                        Axis::Z => {
                            movable_cube.rotate_around(
                                center.translation,
                                Quat::from_axis_angle(Vec3::Z, -3.0 * PI / 2.0),
                            );
                        }
                    },
                }
            }
        }
    }
}
