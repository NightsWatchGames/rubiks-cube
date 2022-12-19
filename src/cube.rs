use bevy::{prelude::*, reflect};
use crate::moving::{*, self};

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