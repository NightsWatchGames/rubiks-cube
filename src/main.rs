use bevy::prelude::*;
use bevy::transform::TransformSystem;
use bevy::utils::Instant;
use bevy_egui::EguiPlugin;
use std::collections::VecDeque;

use camera::*;
use cube::*;
use moving::*;
use ui::*;

mod camera;
mod cube;
mod moving;
mod ui;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MeshPickingPlugin))
        .add_plugins(EguiPlugin)
        .insert_resource(MeshPickingSettings {
            require_markers: true,
            ..Default::default()
        })
        .add_systems(Startup, (setup_camera, setup_cube))
        .insert_resource(CubeSettings::default())
        .insert_resource(SideMoveQueue(VecDeque::new()))
        .insert_resource(MouseDraggingRecorder {
            start_pos: None,
            piece: None,
            triggered: false,
        })
        .insert_resource(TimekeepingTimer(Instant::now()))
        .register_type::<Piece>()
        .add_event::<ScrambleEvent>()
        .add_event::<ResetEvent>()
        .add_systems(PreUpdate, (choose_movable_pieces,))
        .add_systems(
            Update,
            (
                rotate_cube,
                game_ui,
                scramble_cube,
                reset_cube,
                zoom_camera,
                move_camera,
            ),
        )
        .add_systems(
            PostUpdate,
            ((
                piece_translation_round,
                cleanup_movable_pieces.after(piece_translation_round),
            )
                .after(TransformSystem::TransformPropagate),),
        )
        .run();
}
