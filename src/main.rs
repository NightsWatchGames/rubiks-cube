use bevy::prelude::*;
use bevy::time::FixedTimestep;
use bevy_inspector_egui::prelude::*;
use std::collections::VecDeque;
use std::time::Duration;

use cube::*;
use debug::*;
use scramble::*;
use moving::*;

mod cube;
mod debug;
mod scramble;
mod moving;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup)
        .insert_resource(CubeSettings::default())
        .insert_resource(SideMoveQueue (VecDeque::new()))
        .insert_resource(DebugRandomTimer(Timer::new(
            Duration::from_secs(1),
            TimerMode::Repeating,
        )))
        .add_system_to_stage(CoreStage::PreUpdate, choose_movable_pieces)

        .add_system_to_stage(
            CoreStage::Update,
            debug_print_transform_before_rotated.before(rotate_cube),
        )
        .add_system_to_stage(CoreStage::Update, rotate_cube)
        .add_system_to_stage(
            CoreStage::Update,
            debug_print_transform_after_rotated.after(rotate_cube),
        )

        .add_system_to_stage(
            CoreStage::PostUpdate,
            piece_translation_round.after(bevy::transform::TransformSystem::TransformPropagate),
        )
        .add_system_to_stage(
            CoreStage::PostUpdate,
            cleanup_movable_pieces.after(piece_translation_round),
        )

        .add_system(debug_random_side_move_event)
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
                .insert(Piece);
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
                .insert(Piece);
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
                .insert(Piece);
        }
    }

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
