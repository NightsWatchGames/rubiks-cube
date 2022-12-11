use bevy::prelude::*;
use bevy::time::FixedTimestep;
use bevy_inspector_egui::prelude::*;
use std::time::Duration;

use cube::*;
use debug::*;
use scramble::*;

use crate::scramble::Axis;

mod cube;
mod debug;
mod scramble;

#[derive(Debug, Clone, PartialEq, Eq, Hash, StageLabel)]
enum GameStage {
    ChooseCubes,
    Cleanup,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup)
        .add_event::<SideMoveEvent>()
        .insert_resource(DebugRandomTimer(Timer::new(
            Duration::from_secs(1),
            TimerMode::Repeating,
        )))
        // 自定义stage
        .add_stage_before(
            CoreStage::Update,
            GameStage::ChooseCubes,
            SystemStage::parallel(),
        )
        .add_stage_after(
            CoreStage::PostUpdate,
            GameStage::Cleanup,
            SystemStage::parallel(),
        )
        .add_system_to_stage(CoreStage::PreUpdate, choose_cubes_from_side_move_event)
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
            translation_round.after(bevy::transform::TransformSystem::TransformPropagate),
        )
        .add_system_to_stage(GameStage::Cleanup, cleanup_center)
        .add_system_to_stage(GameStage::Cleanup, cleanup_movable_cubes)
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
                .insert(Cube);
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
                .insert(Cube);
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
                .insert(Cube);
        }
    }

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn cleanup_center(mut commands: Commands, center: Query<Entity, With<Center>>) {
    for entity in &center {
        commands.entity(entity).remove::<Center>();
    }
}

fn cleanup_movable_cubes(mut commands: Commands, movable_cubes: Query<Entity, With<MovableCube>>) {
    for entity in &movable_cubes {
        commands.entity(entity).remove::<MovableCube>();
    }
}

// 纠正旋转后的坐标值误差
fn translation_round(mut movable_cubes: Query<&mut Transform, With<MovableCube>>) {
    for mut transform in &mut movable_cubes {
        transform.translation.x = transform.translation.x.round();
        transform.translation.y = transform.translation.y.round();
        transform.translation.z = transform.translation.z.round();
    }
}
