use crate::moving::{self, *};
use bevy::color::palettes;
use bevy::prelude::*;
use rand::seq::SliceRandom;
use rand::Rng;
use std::f32::consts::FRAC_PI_2;
use std::f32::consts::PI;

#[derive(Debug, Component)]
pub struct MovablePiece {
    pub axis: moving::Axis,
    pub rotate: SideRotation,
    pub left_angle: f32,
}

#[derive(Debug, Component, Reflect, Default, Clone, Copy)]
#[reflect(Component)]
pub struct Piece {
    pub init_pos: Vec3,
    pub size: f32,
}

impl Piece {
    pub fn has_up_face(&self) -> bool {
        self.init_pos.y == 1.0
    }
    pub fn has_down_face(&self) -> bool {
        self.init_pos.y == -1.0
    }
    pub fn has_left_face(&self) -> bool {
        self.init_pos.x == -1.0
    }
    pub fn has_right_face(&self) -> bool {
        self.init_pos.x == 1.0
    }
    pub fn has_front_face(&self) -> bool {
        self.init_pos.z == 1.0
    }
    pub fn has_back_face(&self) -> bool {
        self.init_pos.z == -1.0
    }
}

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
    // 块大小
    pub piece_size: f32,
    // 旋转速度
    pub rotate_speed: f32,
    pub front_color: Color,
    pub back_color: Color,
    pub left_color: Color,
    pub right_color: Color,
    pub up_color: Color,
    pub down_color: Color,
    // 游玩模式
    pub play_mode: PlayMode,
    // 相机缩放速度
    pub camera_zoom_speed: f32,
}

impl Default for CubeSettings {
    fn default() -> Self {
        Self {
            piece_size: 1.0,
            rotate_speed: 1.0,
            front_color: palettes::css::GREEN.into(),
            back_color: palettes::css::BLUE.into(),
            left_color: palettes::css::ORANGE.into(),
            right_color: palettes::css::RED.into(),
            up_color: palettes::css::WHITE.into(),
            down_color: palettes::css::YELLOW.into(),
            play_mode: PlayMode::Practice,
            camera_zoom_speed: 1.01,
        }
    }
}

// 重置魔方
#[derive(Debug, Default, Event)]
pub struct ResetEvent;

// 打乱魔方
#[derive(Debug, Default, Event)]
pub struct ScrambleEvent;

pub fn setup_cube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    cube_settings: Res<CubeSettings>,
) {
    create_cube(&mut commands, &mut meshes, &mut materials, &cube_settings);
}

fn create_cube(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    cube_settings: &Res<CubeSettings>,
) {
    // cubes
    for x in [-1.0, 0.0, 1.0] {
        for y in [-1.0, 0.0, 1.0] {
            for z in [-1.0, 0.0, 1.0] {
                let piece = Piece {
                    init_pos: Vec3::new(x, y, z),
                    size: cube_settings.piece_size,
                };
                commands
                    .spawn((
                        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0).mesh())),
                        MeshMaterial3d(materials.add(Color::BLACK)),
                        Transform::from_translation(Vec3::new(x, y, z)),
                        piece,
                        RayCastPickable,
                    ))
                    .observe(handle_drag_start)
                    .observe(handle_move)
                    .observe(handle_drag_end)
                    .with_children(|parent| {
                        // 外部贴纸
                        spawn_stickers(parent, piece, meshes, materials, cube_settings);
                    });
            }
        }
    }
}

fn spawn_stickers(
    parent: &mut ChildBuilder,
    piece: Piece,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    cube_settings: &CubeSettings,
) {
    let sticker_size = 0.9 * cube_settings.piece_size;

    if piece.has_up_face() {
        let transform =
            Transform::from_translation(Vec3::new(0.0, 0.5 * cube_settings.piece_size + 0.01, 0.0));
        parent.spawn((
            Mesh3d(meshes.add(Cuboid::new(sticker_size, 0.01, sticker_size).mesh())),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: cube_settings.up_color,
                unlit: true,
                ..default()
            })),
            transform,
        ));
    }

    if piece.has_down_face() {
        let mut transform = Transform::from_translation(Vec3::new(
            0.0,
            -0.5 * cube_settings.piece_size - 0.01,
            0.0,
        ));
        transform.rotate_x(PI);
        parent.spawn((
            Mesh3d(meshes.add(Cuboid::new(sticker_size, 0.01, sticker_size).mesh())),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: cube_settings.down_color,
                unlit: true,
                ..default()
            })),
            transform,
        ));
    }
    if piece.has_left_face() {
        let mut transform = Transform::from_translation(Vec3::new(
            -0.5 * cube_settings.piece_size - 0.01,
            0.0,
            0.0,
        ));
        transform.rotate_z(FRAC_PI_2);
        parent.spawn((
            Mesh3d(meshes.add(Cuboid::new(sticker_size, 0.01, sticker_size).mesh())),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: cube_settings.left_color,
                unlit: true,
                ..default()
            })),
            transform,
        ));
    }

    if piece.has_right_face() {
        let mut transform =
            Transform::from_translation(Vec3::new(0.5 * cube_settings.piece_size + 0.01, 0.0, 0.0));
        transform.rotate_z(-FRAC_PI_2);
        parent.spawn((
            Mesh3d(meshes.add(Cuboid::new(sticker_size, 0.01, sticker_size).mesh())),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: cube_settings.right_color,
                unlit: true,
                ..default()
            })),
            transform,
        ));
    }

    if piece.has_front_face() {
        let mut transform =
            Transform::from_translation(Vec3::new(0.0, 0.0, 0.5 * cube_settings.piece_size + 0.01));
        transform.rotate_x(FRAC_PI_2);
        parent.spawn((
            Mesh3d(meshes.add(Cuboid::new(sticker_size, 0.01, sticker_size).mesh())),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: cube_settings.front_color,
                unlit: true,
                ..default()
            })),
            transform,
        ));
    }

    if piece.has_back_face() {
        let mut transform = Transform::from_translation(Vec3::new(
            0.0,
            0.0,
            -0.5 * cube_settings.piece_size - 0.01,
        ));
        transform.rotate_x(-FRAC_PI_2);
        parent.spawn((
            Mesh3d(meshes.add(Cuboid::new(sticker_size, 0.01, sticker_size).mesh())),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: cube_settings.back_color,
                unlit: true,
                ..default()
            })),
            transform,
        ));
    }
}

pub fn reset_cube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    cube_settings: Res<CubeSettings>,
    mut events: EventReader<ResetEvent>,
    q_pieces: Query<Entity, With<Piece>>,
) {
    for _ in events.read() {
        // 移除原有魔方
        for piece in &q_pieces {
            commands.entity(piece).despawn_recursive();
        }
        // 重建魔方
        create_cube(&mut commands, &mut meshes, &mut materials, &cube_settings);
    }
}

pub fn scramble_cube(
    mut events: EventReader<ScrambleEvent>,
    mut side_move_queue: ResMut<SideMoveQueue>,
) {
    for _ in events.read() {
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
