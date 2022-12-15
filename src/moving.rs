use crate::cube::*;
use bevy::prelude::*;
use std::collections::VecDeque;
use std::f32::consts::FRAC_PI_2;
use std::f32::consts::PI;
use std::f32::consts::TAU;

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

#[derive(Debug, Resource)]
pub struct SideMoveQueue(pub VecDeque<SideMoveEvent>);

pub fn choose_movable_pieces(
    mut commands: Commands,
    mut q_pieces: Query<(Entity, &Transform), With<Piece>>,
    q_movable_pieces: Query<Entity, With<MovablePiece>>,
    mut side_move_queue: ResMut<SideMoveQueue>,
) {
    if q_movable_pieces.is_empty() {
        // 从SideMoveQueue消费一个
        if side_move_queue.0.is_empty() {
            return;
        }
        let event = side_move_queue.0.pop_front().unwrap();
        dbg!(event);
        for (_, transform) in &q_pieces {
            info!(
                "before choosing, cube translation={}",
                transform.translation
            );
        }

        let left_angle = match event.rotate {
            SideRotation::Clockwise90 => FRAC_PI_2,
            SideRotation::Clockwise180 => PI,
            SideRotation::Clockwise270 => 3.0 * PI / 2.0,
            SideRotation::Counterclockwise90 => -FRAC_PI_2,
            SideRotation::Counterclockwise180 => -PI,
            SideRotation::Counterclockwise270 => -3.0 * PI / 2.0,
        };

        let side = event.side;
        match side.0 {
            Axis::X => {
                for (entity, transform) in &mut q_pieces {
                    if transform.translation.x == side.1 {
                        info!("insert movable cube: translation={}", transform.translation);
                        commands.entity(entity).insert(MovablePiece {
                            axis: Axis::X,
                            rotate: event.rotate,
                            left_angle,
                        });
                    }
                }
            }
            Axis::Y => {
                for (entity, transform) in &mut q_pieces {
                    if transform.translation.y == side.1 {
                        info!("insert movable cube: translation={}", transform.translation);
                        commands.entity(entity).insert(MovablePiece {
                            axis: Axis::Y,
                            rotate: event.rotate,
                            left_angle,
                        });
                    }
                }
            }
            Axis::Z => {
                for (entity, transform) in &mut q_pieces {
                    if transform.translation.z == side.1 {
                        info!("insert movable cube: translation={}", transform.translation);
                        commands.entity(entity).insert(MovablePiece {
                            axis: Axis::Z,
                            rotate: event.rotate,
                            left_angle,
                        });
                    }
                }
            }
        }
    }
}

pub fn cleanup_movable_pieces(
    mut commands: Commands,
    mut movable_pieces: Query<(Entity, &mut Transform, &MovablePiece)>,
) {
    for (entity, mut transform, movable_piece) in &mut movable_pieces {
        if movable_piece.left_angle == 0.0 {
            commands.entity(entity).remove::<MovablePiece>();
        }
    }
}

// 纠正旋转后的坐标值误差
pub fn piece_translation_round(mut movable_pieces: Query<(&mut Transform, &MovablePiece)>) {
    for (mut transform, movable_piece) in &mut movable_pieces {
        if movable_piece.left_angle == 0.0 {
            transform.translation.x = transform.translation.x.round();
            transform.translation.y = transform.translation.y.round();
            transform.translation.z = transform.translation.z.round();
        }
    }
}

pub fn rotate_cube(
    mut movable_pieces: Query<(&mut MovablePiece, &mut Transform)>,
    cube_settings: Res<CubeSettings>,
    time: Res<Time>,
) {
    for (mut movable_piece, mut transform) in &mut movable_pieces {
        info!(
            "rotate - movable cube={:?}, transform={}",
            &movable_piece, transform.translation
        );
        let axis = match movable_piece.axis {
            Axis::X => Vec3::X,
            Axis::Y => Vec3::Y,
            Axis::Z => Vec3::Z,
        };
        let mut angle = match movable_piece.rotate {
            SideRotation::Clockwise90 | SideRotation::Clockwise180 | SideRotation::Clockwise270 => {
                cube_settings.rotate_speed * TAU * time.delta_seconds()
            }
            SideRotation::Counterclockwise90
            | SideRotation::Counterclockwise180
            | SideRotation::Counterclockwise270 => {
                -cube_settings.rotate_speed * TAU * time.delta_seconds()
            }
        };

        let mut new_left_angle = movable_piece.left_angle - angle;
        match movable_piece.rotate {
            SideRotation::Clockwise90 | SideRotation::Clockwise180 | SideRotation::Clockwise270 => {
                if new_left_angle < 0.0 {
                    angle = movable_piece.left_angle;
                    new_left_angle = 0.0;
                }
            }
            SideRotation::Counterclockwise90
            | SideRotation::Counterclockwise180
            | SideRotation::Counterclockwise270 => {
                if new_left_angle > 0.0 {
                    angle = movable_piece.left_angle;
                    new_left_angle = 0.0;
                }
            }
        };

        transform.rotate_around(Vec3::new(0.0, 0.0, 0.0), Quat::from_axis_angle(axis, angle));
        movable_piece.left_angle = new_left_angle;
    }
}
