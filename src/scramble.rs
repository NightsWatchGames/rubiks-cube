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
                        info!("insert movable cube: translation={}", transform.translation);
                        commands.entity(entity).insert(MovableCube {
                            axis: Axis::X,
                            rotate: event.rotate,
                        });
                    }
                }
            }
            Axis::Y => {
                for (entity, transform) in &mut query {
                    if transform.translation.y == side.1 {
                        info!("insert movable cube: translation={}", transform.translation);
                        commands.entity(entity).insert(MovableCube {
                            axis: Axis::Y,
                            rotate: event.rotate,
                        });
                    }
                }
            }
            Axis::Z => {
                for (entity, transform) in &mut query {
                    if transform.translation.z == side.1 {
                        info!("insert movable cube: translation={}", transform.translation);
                        commands.entity(entity).insert(MovableCube {
                            axis: Axis::Z,
                            rotate: event.rotate,
                        });
                    }
                }
            }
        }
    }
}

pub fn rotate_cube(mut movable_cubes: Query<(&MovableCube, &mut Transform)>) {
    for (movable_cube, mut transform) in &mut movable_cubes {
        info!("rotate - movable cube={:?}, transform={}", &movable_cube, transform.translation);
        match movable_cube.rotate {
            SideRotation::Clockwise90 => match movable_cube.axis {
                Axis::X => {
                    transform.rotate_around(Vec3::ZERO, Quat::from_axis_angle(Vec3::X, FRAC_PI_2));
                }
                Axis::Y => {
                    transform.rotate_around(Vec3::ZERO, Quat::from_axis_angle(Vec3::Y, FRAC_PI_2));
                }
                Axis::Z => {
                    transform.rotate_around(Vec3::ZERO, Quat::from_axis_angle(Vec3::Z, FRAC_PI_2));
                }
            },
            SideRotation::Clockwise180 => match movable_cube.axis {
                Axis::X => {
                    transform.rotate_around(Vec3::ZERO, Quat::from_axis_angle(Vec3::X, PI));
                }
                Axis::Y => {
                    transform.rotate_around(Vec3::ZERO, Quat::from_axis_angle(Vec3::Y, PI));
                }
                Axis::Z => {
                    transform.rotate_around(Vec3::ZERO, Quat::from_axis_angle(Vec3::Z, PI));
                }
            },
            SideRotation::Clockwise270 => match movable_cube.axis {
                Axis::X => {
                    transform
                        .rotate_around(Vec3::ZERO, Quat::from_axis_angle(Vec3::X, 3.0 * PI / 2.0));
                }
                Axis::Y => {
                    transform
                        .rotate_around(Vec3::ZERO, Quat::from_axis_angle(Vec3::Y, 3.0 * PI / 2.0));
                }
                Axis::Z => {
                    transform
                        .rotate_around(Vec3::ZERO, Quat::from_axis_angle(Vec3::Z, 3.0 * PI / 2.0));
                }
            },
            SideRotation::Counterclockwise90 => match movable_cube.axis {
                Axis::X => {
                    transform.rotate_around(Vec3::ZERO, Quat::from_axis_angle(Vec3::X, -FRAC_PI_2));
                }
                Axis::Y => {
                    transform.rotate_around(Vec3::ZERO, Quat::from_axis_angle(Vec3::Y, -FRAC_PI_2));
                }
                Axis::Z => {
                    transform.rotate_around(Vec3::ZERO, Quat::from_axis_angle(Vec3::Z, -FRAC_PI_2));
                }
            },
            SideRotation::Counterclockwise180 => match movable_cube.axis {
                Axis::X => {
                    transform.rotate_around(Vec3::ZERO, Quat::from_axis_angle(Vec3::X, -PI));
                }
                Axis::Y => {
                    transform.rotate_around(Vec3::ZERO, Quat::from_axis_angle(Vec3::Y, -PI));
                }
                Axis::Z => {
                    transform.rotate_around(Vec3::ZERO, Quat::from_axis_angle(Vec3::Z, -PI));
                }
            },
            SideRotation::Counterclockwise270 => match movable_cube.axis {
                Axis::X => {
                    transform
                        .rotate_around(Vec3::ZERO, Quat::from_axis_angle(Vec3::X, -3.0 * PI / 2.0));
                }
                Axis::Y => {
                    transform
                        .rotate_around(Vec3::ZERO, Quat::from_axis_angle(Vec3::Y, -3.0 * PI / 2.0));
                }
                Axis::Z => {
                    transform
                        .rotate_around(Vec3::ZERO, Quat::from_axis_angle(Vec3::Z, -3.0 * PI / 2.0));
                }
            },
        }
    }
}
