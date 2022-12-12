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

pub fn choose_pieces_from_side_move_event(
    mut commands: Commands,
    mut query: Query<(Entity, &Transform), With<Piece>>,
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
                        commands.entity(entity).insert(MovablePiece {
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
                        commands.entity(entity).insert(MovablePiece {
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
                        commands.entity(entity).insert(MovablePiece {
                            axis: Axis::Z,
                            rotate: event.rotate,
                        });
                    }
                }
            }
        }
    }
}

pub fn rotate_cube(mut movable_pieces: Query<(&MovablePiece, &mut Transform)>) {
    for (movable_piece, mut transform) in &mut movable_pieces {
        info!("rotate - movable cube={:?}, transform={}", &movable_piece, transform.translation);
        let axis = match movable_piece.axis {
            Axis::X => { Vec3::X }
            Axis::Y => { Vec3::Y }
            Axis::Z => { Vec3::Z }
        };
        let angle = match movable_piece.rotate {
            SideRotation::Clockwise90 => { FRAC_PI_2 }
            SideRotation::Clockwise180 => { PI }
            SideRotation::Clockwise270 => { 3.0 * PI / 2.0 }
            SideRotation::Counterclockwise90 => { -FRAC_PI_2 }
            SideRotation::Counterclockwise180 => { -PI }
            SideRotation::Counterclockwise270 => { -3.0 * PI / 2.0 }
        };
        transform.rotate_around(Vec3::ZERO, Quat::from_axis_angle(axis, angle));
    }
}
