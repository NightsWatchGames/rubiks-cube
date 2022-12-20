use bevy::{prelude::*, reflect};
use bevy_mod_picking::PickableBundle;
use bevy_mod_raycast::RaycastMesh;
use crate::moving::{*, self};
use crate::debug::*;
use rand::seq::SliceRandom;
use rand::Rng;

#[derive(Debug, Component)]
pub struct MovablePiece {
    pub axis: moving::Axis,
    pub rotate: SideRotation,
    pub left_angle: f32,
}

#[derive(Debug, Component, Reflect, Default)]
#[reflect(Component)]
pub struct Piece(pub Vec3);

#[derive(Debug, PartialEq, Eq)]
pub enum PlayMode {
    // 练习模式
    Practice,
    // 计时模式
    Timekeeping,
}

/// 魔方设置
#[derive(Debug, Resource)]
pub struct CubeSettings {
    // 几阶魔方
    pub cube_order: u8,
    // 块大小
    pub piece_size: f32,
    // 旋转速度
    pub rotate_speed: f32,
    pub front_color: Color,
    pub back_color: Color,
    pub left_color: Color,
    pub right_color: Color,
    pub top_color: Color,
    pub bottom_color: Color,
    pub play_mode: PlayMode,
}

impl Default for CubeSettings {
    fn default() -> Self {
        Self {
            cube_order: 3,
            piece_size: 1.0,
            rotate_speed: 1.0,
            front_color: Color::GREEN,
            back_color: Color::BLUE,
            left_color: Color::ORANGE,
            right_color: Color::RED,
            top_color: Color::WHITE,
            bottom_color: Color::YELLOW,
            play_mode: PlayMode::Practice,
        }
    }
}

#[derive(Debug, Component)]
pub enum Face {
    // 上面
    U,
    // 下面
    D,
    // 左面
    L,
    // 右面
    R,
    // 前面
    F,
    // 后面
    B,
}

// 重置魔方
#[derive(Debug, Default)]
pub struct ResetEvent;

// 打乱魔方
#[derive(Debug, Default)]
pub struct ScrambleEvent;

pub fn setup_cube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    create_cube(&mut commands, &mut meshes, &mut materials);
}

fn create_cube(commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<StandardMaterial>>,) {
    // cubes
    let y = 0.0;
    for x in [-1.0, 0.0, 1.0] {
        for z in [-1.0, 0.0, 1.0] {
            commands
                .spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                    material: materials.add(debug_random_color().into()),
                    transform: Transform::from_translation(Vec3::new(x, y, z)),
                    ..default()
                })
                .insert(Piece (Vec3::new(x, y, z)))
                .insert(PickableBundle::default())
                .insert(RaycastMesh::<MyRaycastSet>::default());
        }
    }

    let y = 1.0;
    for x in [-1.0, 0.0, 1.0] {
        for z in [-1.0, 0.0, 1.0] {
            commands
                .spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                    material: materials.add(debug_random_color().into()),
                    transform: Transform::from_translation(Vec3::new(x, y, z)),
                    ..default()
                })
                .insert(Piece (Vec3::new(x, y, z)))
                .insert(PickableBundle::default())
                .insert(RaycastMesh::<MyRaycastSet>::default());
        }
    }

    let y = -1.0;
    for x in [-1.0, 0.0, 1.0] {
        for z in [-1.0, 0.0, 1.0] {
            commands
                .spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                    material: materials.add(debug_random_color().into()),
                    transform: Transform::from_translation(Vec3::new(x, y, z)),
                    ..default()
                })
                .insert(Piece (Vec3::new(x, y, z)))
                .insert(PickableBundle::default())
                .insert(RaycastMesh::<MyRaycastSet>::default());
        }
    }
}

pub fn reset_cube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut events: EventReader<ResetEvent>, 
    q_pieces: Query<Entity, With<Piece>>) {
    for _ in events.iter() {
        // 移除原有魔方
        for piece in &q_pieces {
            commands.entity(piece).despawn_recursive();
        }
        // 重建魔方
        create_cube(&mut commands, &mut meshes, &mut materials);
    }
}

pub fn scramble_cube(
    mut events: EventReader<ScrambleEvent>,
    mut side_move_queue: ResMut<SideMoveQueue>,
) {
    for _ in events.iter() {
        for _ in 0..5 {
            let axis_value = vec![-1.0f32, 0.0, 1.0]
                .choose(&mut rand::thread_rng())
                .unwrap()
                .clone();
            let axis = match rand::thread_rng().gen_range(0..3) {
                0 => moving::Axis::X,
                1 => moving::Axis::Y,
                2 => moving::Axis::Z,
                _ => moving::Axis::X,
            };
            // let axis = moving::Axis::Y;
            let rotate = match rand::thread_rng().gen_range(0..3) {
                0 => SideRotation::Clockwise90,
                1 => SideRotation::Clockwise180,
                2 => SideRotation::Counterclockwise90,
                _ => SideRotation::Clockwise90,
            };
            side_move_queue.0.push_back(SideMoveEvent {
                side: (axis, axis_value),
                rotate,
            })
        }
    }
}