use bevy::prelude::*;
use crate::scramble::{*, self};

#[derive(Debug, Component)]
pub struct MovablePiece {
    pub axis: scramble::Axis,
    pub rotate: SideRotation,
}

#[derive(Debug, Component)]
pub struct Piece;

/// 魔方设置
#[derive(Debug, Resource)]
pub struct CubeSettings {
    /// 几阶魔方
    pub cube_order: u8,
    /// 块大小
    pub piece_size: f32,
    pub front_color: Color,
    pub back_color: Color,
    pub left_color: Color,
    pub right_color: Color,
    pub top_color: Color,
    pub bottom_color: Color,
}

impl Default for CubeSettings {
    fn default() -> Self {
        Self {
            cube_order: 3,
            piece_size: 1.0,
            front_color: Color::GREEN,
            back_color: Color::BLUE,
            left_color: Color::ORANGE,
            right_color: Color::RED,
            top_color: Color::WHITE,
            bottom_color: Color::YELLOW,
        }
    }
}