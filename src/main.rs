use bevy::prelude::*;
use bevy::time::FixedTimestep;
use bevy::transform::TransformSystem;
use bevy_inspector_egui::prelude::*;
use bevy_mod_picking::{PickableBundle, PickingCameraBundle, DefaultPickingPlugins, DebugCursorPickingPlugin, DebugEventsPickingPlugin};
use bevy_mod_raycast::{DefaultRaycastingPlugin, RaycastSource, RaycastMesh, Intersection, RaycastMethod, RaycastSystem};
use std::collections::VecDeque;
use std::time::Duration;

use cube::*;
use debug::*;
use moving::*;
use scramble::*;

mod cube;
mod debug;
mod moving;
mod scramble;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(DebugCursorPickingPlugin)
        .add_plugin(DebugEventsPickingPlugin)
        .add_plugin(DefaultRaycastingPlugin::<MyRaycastSet>::default())
        .add_startup_system(setup)
        .insert_resource(CubeSettings::default())
        .insert_resource(SideMoveQueue(VecDeque::new()))
        .insert_resource(MouseDraggingRecorder { start_pos: None, piece: None})
        .insert_resource(DebugRandomTimer(Timer::new(
            Duration::from_secs(1),
            TimerMode::Repeating,
        )))
        .register_type::<Piece>()

        .add_system_to_stage(
            CoreStage::First,
            update_raycast_with_cursor.before(RaycastSystem::BuildRays::<MyRaycastSet>),
        )

        .add_system_to_stage(CoreStage::PreUpdate, choose_movable_pieces)

        .add_system_to_stage(CoreStage::Update, rotate_cube)
        // .add_system_to_stage(CoreStage::Update, intersection)

        .add_system_to_stage(CoreStage::PostUpdate, mouse_dragging)
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .after(TransformSystem::TransformPropagate)
                .with_system(piece_translation_round)
                .with_system(cleanup_movable_pieces.after(piece_translation_round))
        )

        // .add_system(debug_random_side_move_event)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
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

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    }).insert(PickingCameraBundle::default())
    .insert(RaycastSource::<MyRaycastSet>::new());
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