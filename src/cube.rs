use bevy::prelude::*;
use crate::scramble::{*, self};

#[derive(Debug, Component)]
pub struct MovableCube {
    pub axis: scramble::Axis,
    pub rotate: SideRotation,
}

#[derive(Debug, Component)]
pub struct Cube;