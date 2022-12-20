use bevy::prelude::*;
use bevy::time::FixedTimestep;
use bevy::utils::Instant;
use bevy::transform::TransformSystem;
use bevy_inspector_egui::prelude::*;
use bevy_mod_picking::{PickableBundle, PickingCameraBundle, DefaultPickingPlugins, DebugCursorPickingPlugin, DebugEventsPickingPlugin};
use bevy_mod_raycast::{DefaultRaycastingPlugin, RaycastSource, RaycastMesh, Intersection, RaycastMethod, RaycastSystem};
use bevy_egui::EguiPlugin;
use std::collections::VecDeque;
use std::time::Duration;

use cube::*;
use debug::*;
use moving::*;
use ui::*;
use camera::*;

mod cube;
mod debug;
mod moving;
mod ui;
mod camera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        // .add_plugin(WorldInspectorPlugin::new())
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(DebugCursorPickingPlugin)
        .add_plugin(DebugEventsPickingPlugin)
        .add_plugin(DefaultRaycastingPlugin::<MyRaycastSet>::default())
        .add_startup_system(setup_camera)
        .add_startup_system(setup_cube)
        .insert_resource(CubeSettings::default())
        .insert_resource(SideMoveQueue(VecDeque::new()))
        .insert_resource(MouseDraggingRecorder { start_pos: None, piece: None})
        .insert_resource(DebugRandomTimer(Timer::new(
            Duration::from_secs(1),
            TimerMode::Repeating,
        )))
        .insert_resource(TimekeepingTimer(Instant::now()))
        .register_type::<Piece>()
        .add_event::<ScrambleEvent>()
        .add_event::<ResetEvent>()

        .add_system_to_stage(
            CoreStage::First,
            update_raycast_with_cursor.before(RaycastSystem::BuildRays::<MyRaycastSet>),
        )

        .add_system_to_stage(CoreStage::PreUpdate, choose_movable_pieces)

        .add_system_to_stage(CoreStage::Update, rotate_cube)
        // .add_system_to_stage(CoreStage::Update, intersection)
        .add_system_to_stage(CoreStage::Update, game_ui)

        .add_system_to_stage(CoreStage::PostUpdate, mouse_dragging)
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .after(TransformSystem::TransformPropagate)
                .with_system(piece_translation_round)
                .with_system(cleanup_movable_pieces.after(piece_translation_round))
        )

        .add_system(scramble_cube)
        .add_system(reset_cube)
        .add_system(zoom_camera)
        .run();
}


fn intersection(query: Query<&Intersection<MyRaycastSet>>) {
    for intersection in &query {
        info!(
            "Distance {:?}, Position {:?}",
            intersection.distance(),
            intersection.position()
        );
    }
}

fn update_raycast_with_cursor(
    mut cursor: EventReader<CursorMoved>,
    mut query: Query<&mut RaycastSource<MyRaycastSet>>,
) {
    // Grab the most recent cursor event if it exists:
    let cursor_position = match cursor.iter().last() {
        Some(cursor_moved) => cursor_moved.position,
        None => return,
    };

    for mut pick_source in &mut query {
        pick_source.cast_method = RaycastMethod::Screenspace(cursor_position);
    }
}