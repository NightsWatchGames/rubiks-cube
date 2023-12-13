use crate::moving::{self, *};
use bevy::prelude::*;
use bevy_mod_picking::events::Move;
use bevy_mod_picking::prelude::DragEnd;
use bevy_mod_picking::prelude::DragStart;
use bevy_mod_picking::prelude::On;
use bevy_mod_picking::prelude::Pointer;
use bevy_mod_picking::prelude::RaycastPickTarget;
use bevy_mod_picking::PickableBundle;
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
            cube_order: 3,
            piece_size: 1.0,
            rotate_speed: 1.0,
            front_color: Color::GREEN,
            back_color: Color::BLUE,
            left_color: Color::ORANGE,
            right_color: Color::RED,
            up_color: Color::WHITE,
            down_color: Color::YELLOW,
            play_mode: PlayMode::Practice,
            camera_zoom_speed: 1.01,
        }
    }
}

impl CubeSettings {
    // 魔方整体坐标的边界值（正数）
    pub fn coordinate_boundary(&self) -> f32 {
        (self.cube_order as f32 / 2.0) * self.piece_size
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
                    .spawn(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                        material: materials.add(Color::BLACK.into()),
                        transform: Transform::from_translation(Vec3::new(x, y, z)),
                        ..default()
                    })
                    .insert(piece)
                    .insert(PickableBundle::default())
                    .insert(RaycastPickTarget::default())
                    // .insert(RaycastMesh::<MyRaycastSet>::default())
                    .insert(On::<Pointer<DragStart>>::run(handle_drag_start))
                    .insert(On::<Pointer<Move>>::run(handle_move))
                    .insert(On::<Pointer<DragEnd>>::run(handle_drag_end))
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
        parent.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane {
                size: sticker_size,
                subdivisions: 0,
            })),
            material: materials.add(StandardMaterial {
                base_color: cube_settings.up_color,
                unlit: true,
                ..default()
            }),
            transform: transform,
            ..Default::default()
        });
    }

    if piece.has_down_face() {
        let mut transform = Transform::from_translation(Vec3::new(
            0.0,
            -0.5 * cube_settings.piece_size - 0.01,
            0.0,
        ));
        transform.rotate_x(PI);
        parent.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane {
                size: sticker_size,
                subdivisions: 0,
            })),
            material: materials.add(StandardMaterial {
                base_color: cube_settings.down_color,
                unlit: true,
                ..default()
            }),
            transform: transform,
            ..Default::default()
        });
    }
    if piece.has_left_face() {
        let mut transform = Transform::from_translation(Vec3::new(
            -0.5 * cube_settings.piece_size - 0.01,
            0.0,
            0.0,
        ));
        transform.rotate_z(FRAC_PI_2);
        parent.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane {
                size: sticker_size,
                subdivisions: 0,
            })),
            material: materials.add(StandardMaterial {
                base_color: cube_settings.left_color,
                unlit: true,
                ..default()
            }),
            transform: transform,
            ..Default::default()
        });
    }

    if piece.has_right_face() {
        let mut transform =
            Transform::from_translation(Vec3::new(0.5 * cube_settings.piece_size + 0.01, 0.0, 0.0));
        transform.rotate_z(-FRAC_PI_2);
        parent.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane {
                size: sticker_size,
                subdivisions: 0,
            })),
            material: materials.add(StandardMaterial {
                base_color: cube_settings.right_color,
                unlit: true,
                ..default()
            }),
            transform: transform,
            ..Default::default()
        });
    }

    if piece.has_front_face() {
        let mut transform =
            Transform::from_translation(Vec3::new(0.0, 0.0, 0.5 * cube_settings.piece_size + 0.01));
        transform.rotate_x(FRAC_PI_2);
        parent.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane {
                size: sticker_size,
                subdivisions: 0,
            })),
            material: materials.add(StandardMaterial {
                base_color: cube_settings.front_color,
                unlit: true,
                ..default()
            }),
            transform: transform,
            ..Default::default()
        });
    }

    if piece.has_back_face() {
        let mut transform = Transform::from_translation(Vec3::new(
            0.0,
            0.0,
            -0.5 * cube_settings.piece_size - 0.01,
        ));
        transform.rotate_x(-FRAC_PI_2);
        parent.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane {
                size: sticker_size,
                subdivisions: 0,
            })),
            material: materials.add(StandardMaterial {
                base_color: cube_settings.back_color,
                unlit: true,
                ..default()
            }),
            transform: transform,
            ..Default::default()
        });
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
    for _ in events.iter() {
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
