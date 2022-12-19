use std::f32::consts::FRAC_PI_2;
use std::f32::consts::PI;
use bevy::prelude::*;
use rand::seq::SliceRandom;
use rand::Rng;
use crate::cube::*;
use crate::moving;
use crate::moving::*;

#[derive(Debug, Default)]
pub struct ScrambleEvent;

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
