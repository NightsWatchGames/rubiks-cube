use crate::cube::*;
use bevy::prelude::*;
use bevy_mod_picking::backend::PointerHits;
use bevy_mod_picking::prelude::*;
use bevy_mod_raycast::prelude::*;
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

#[derive(Debug, Clone, Copy, Event)]
pub struct SideMoveEvent {
    // 旋转的面，对应固定的x/y/z坐标值
    pub side: (Axis, f32),
    // 旋转
    pub rotate: SideRotation,
}

#[derive(Debug, Resource)]
pub struct SideMoveQueue(pub VecDeque<SideMoveEvent>);

#[derive(Debug, Resource)]
pub struct MouseDraggingRecorder {
    pub start_pos: Option<Vec3>,
    pub piece: Option<Entity>,
}

impl MouseDraggingRecorder {
    pub fn clear(&mut self) {
        self.start_pos = None;
        self.piece = None;
    }
}

#[derive(Reflect)]
pub struct MyRaycastSet;

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
    movable_pieces: Query<(Entity, &MovablePiece)>,
) {
    for (entity, movable_piece) in &movable_pieces {
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
        // info!(
        //     "rotate - movable cube={:?}, transform={}",
        //     &movable_piece, transform.translation
        // );
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

pub fn handle_drag_start(
    listener: Listener<Pointer<DragStart>>,
    mut recorder: ResMut<MouseDraggingRecorder>,
) {
    // recorder开始记录
    println!("drag start event: {:?}", listener);
    let piece_entity = listener.target;
    recorder.piece = Some(piece_entity.clone());
    recorder.start_pos = listener.hit.position;

    info!("MouseDraggingRecorder started {:?}", recorder);
}

// 监测鼠标拖动距离，当鼠标拖动距离超过临界值时，触发一个面旋转
pub fn handle_move(
    listener: Listener<Pointer<Move>>,
    mut recorder: ResMut<MouseDraggingRecorder>,
    mut side_move_queue: ResMut<SideMoveQueue>,
    q_pieces: Query<&Transform, With<Piece>>,
) {
    if listener.event.hit.position.is_some() && recorder.start_pos.is_some() {
        let start_pos = recorder.start_pos.unwrap();
        let current_pos = listener.event.hit.position.unwrap();

        // 鼠标拽动距离超过临界值
        if start_pos.distance(current_pos) > 0.5 {
            // 触发旋转
            info!("Trigger side move event, end_pos: {:?}", current_pos);
            let translation = q_pieces.get(recorder.piece.unwrap()).unwrap().translation;
            let event = gen_side_move_event(translation, recorder.start_pos.unwrap(), current_pos);
            info!("gen event: {:?}", event);
            if event.is_some() {
                side_move_queue.0.push_back(event.unwrap());
            }

            // 清除recorder
            recorder.clear();
        }
    }
}

pub fn handle_drag_end(mut recorder: ResMut<MouseDraggingRecorder>) {
    println!("drag end event");
    recorder.clear();
}

fn gen_side_move_event(
    piece_translation: Vec3,
    start_pos: Vec3,
    end_pos: Vec3,
) -> Option<SideMoveEvent> {
    if (start_pos.x.abs() - 1.5).abs() < 0.001 {
        let delta_y = end_pos.y - start_pos.y;
        let delta_z = end_pos.z - start_pos.z;
        if delta_y.abs() > delta_z.abs() {
            // y轴变化大，沿z轴旋转
            let rotate = {
                // 右面
                if start_pos.x > 0.0 {
                    if delta_y > 0.0 {
                        SideRotation::Clockwise90
                    } else {
                        SideRotation::Counterclockwise90
                    }
                // 左面
                } else {
                    if delta_y > 0.0 {
                        SideRotation::Counterclockwise90
                    } else {
                        SideRotation::Clockwise90
                    }
                }
            };
            return Some(SideMoveEvent {
                side: (Axis::Z, piece_translation.z.round()),
                rotate,
            });
        } else {
            // z轴变化大，沿y轴旋转
            let rotate = {
                // 右面
                if start_pos.x > 0.0 {
                    if delta_z > 0.0 {
                        SideRotation::Counterclockwise90
                    } else {
                        SideRotation::Clockwise90
                    }
                // 左面
                } else {
                    if delta_z > 0.0 {
                        SideRotation::Clockwise90
                    } else {
                        SideRotation::Counterclockwise90
                    }
                }
            };
            return Some(SideMoveEvent {
                side: (Axis::Y, piece_translation.y.round()),
                rotate,
            });
        }
    } else if (start_pos.y.abs() - 1.5).abs() < 0.001 {
        let delta_x = end_pos.x - start_pos.x;
        let delta_z = end_pos.z - start_pos.z;
        if delta_x.abs() > delta_z.abs() {
            // x轴变化大，沿z轴旋转
            let rotate = {
                // 上面
                if start_pos.y > 0.0 {
                    if delta_x > 0.0 {
                        SideRotation::Counterclockwise90
                    } else {
                        SideRotation::Clockwise90
                    }
                // 下面
                } else {
                    if delta_x > 0.0 {
                        SideRotation::Clockwise90
                    } else {
                        SideRotation::Counterclockwise90
                    }
                }
            };
            return Some(SideMoveEvent {
                side: (Axis::Z, piece_translation.z.round()),
                rotate,
            });
        } else {
            // z轴变化大，沿x轴旋转
            let rotate = {
                // 上面
                if start_pos.y > 0.0 {
                    if delta_z > 0.0 {
                        SideRotation::Clockwise90
                    } else {
                        SideRotation::Counterclockwise90
                    }
                // 下面
                } else {
                    if delta_z > 0.0 {
                        SideRotation::Counterclockwise90
                    } else {
                        SideRotation::Clockwise90
                    }
                }
            };
            return Some(SideMoveEvent {
                side: (Axis::X, piece_translation.x.round()),
                rotate,
            });
        }
    } else {
        let delta_x = end_pos.x - start_pos.x;
        let delta_y = end_pos.y - start_pos.y;
        if delta_x.abs() > delta_y.abs() {
            // x轴变化大，沿y轴旋转
            let rotate = {
                // 前面
                if start_pos.z > 0.0 {
                    if delta_x > 0.0 {
                        SideRotation::Clockwise90
                    } else {
                        SideRotation::Counterclockwise90
                    }
                // 后面
                } else {
                    if delta_x > 0.0 {
                        SideRotation::Counterclockwise90
                    } else {
                        SideRotation::Clockwise90
                    }
                }
            };
            return Some(SideMoveEvent {
                side: (Axis::Y, piece_translation.y.round()),
                rotate,
            });
        } else {
            // y轴变化大，沿x轴旋转
            let rotate = {
                // 前面
                if start_pos.z > 0.0 {
                    if delta_y > 0.0 {
                        SideRotation::Counterclockwise90
                    } else {
                        SideRotation::Clockwise90
                    }
                // 后面
                } else {
                    if delta_y > 0.0 {
                        SideRotation::Clockwise90
                    } else {
                        SideRotation::Counterclockwise90
                    }
                }
            };
            return Some(SideMoveEvent {
                side: (Axis::X, piece_translation.x.round()),
                rotate,
            });
        }
    }
}
