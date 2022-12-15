use crate::cube::*;
use crate::moving;
use crate::moving::*;
use bevy::prelude::*;
use rand::seq::SliceRandom;
use rand::Rng;

#[derive(Resource)]
pub struct DebugRandomTimer(pub Timer);

pub fn debug_random_side_move_event(
    mut side_move_queue: ResMut<SideMoveQueue>,
    mut timer: ResMut<DebugRandomTimer>,
    time: Res<Time>,
) {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
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
        let rotate = match rand::thread_rng().gen_range(0..6) {
            0 => SideRotation::Clockwise90,
            1 => SideRotation::Clockwise180,
            2 => SideRotation::Clockwise270,
            3 => SideRotation::Counterclockwise90,
            4 => SideRotation::Counterclockwise180,
            5 => SideRotation::Counterclockwise270,
            _ => SideRotation::Clockwise90,
        };
        side_move_queue.0.push_back(SideMoveEvent {
            side: (axis, axis_value),
            rotate,
        })
    }
}

pub fn debug_random_color() -> Color {
    let mut rng = rand::thread_rng();
    Color::rgb(rng.gen(), rng.gen(), rng.gen())
}

pub fn debug_print_global_transform(
    query: Query<&GlobalTransform, With<Piece>>,
    mut side_move_event: EventReader<SideMoveEvent>,
) {
    for event in side_move_event.iter() {
        for global_transform in &query {
            info!("cube global transform {}", global_transform.translation());
        }
    }
}

pub fn debug_print_transform_before_rotated(
    query: Query<(Entity, &Transform, &GlobalTransform), With<MovablePiece>>,
) {
    for (entity, transform, global_transform) in &query {
        info!(
            "before rotated - cube={:?}, transform={}, global transform={}",
            entity,
            transform.translation,
            global_transform.translation()
        );
    }
}
pub fn debug_print_transform_after_rotated(
    query: Query<(Entity, &Transform, &GlobalTransform), With<MovablePiece>>,
) {
    for (entity, transform, global_transform) in &query {
        info!(
            "after rotated - cube={:?}, transform={}, global transform={}",
            entity,
            transform.translation,
            global_transform.translation()
        );
    }
}
