use std::time::Duration;
use bevy::prelude::*;
use bevy::time::FixedTimestep;
use bevy_inspector_egui::prelude::*;

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
    EstablishHierarchy,
    Cleanup,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup)
        .add_event::<SideMoveEvent>()
        .insert_resource(DebugRandomTimer(Timer::new(
            Duration::from_secs(2),
            TimerMode::Repeating,
        )))
        // 自定义stage
        .add_stage_before(
            CoreStage::Update,
            GameStage::ChooseCubes,
            SystemStage::parallel(),
        )
        .add_stage_before(
            CoreStage::Update,
            GameStage::EstablishHierarchy,
            SystemStage::parallel(),
        )
        .add_stage_after(
            CoreStage::PostUpdate,
            GameStage::Cleanup,
            SystemStage::parallel(),
        )
        .add_system_to_stage(GameStage::ChooseCubes, choose_cubes_from_side_move_event)
        .add_system_to_stage(GameStage::EstablishHierarchy, establish_hierarchy)

        .add_system_to_stage(CoreStage::Update, debug_print_transform_before_rotated.before(rotate_cube))
        .add_system_to_stage(CoreStage::Update, rotate_cube)
        .add_system_to_stage(CoreStage::Update, debug_print_transform_after_rotated.after(rotate_cube))

        .add_system_to_stage(GameStage::Cleanup, cleanup_hierarchy)
        .add_system_to_stage(GameStage::Cleanup, cleanup_center.after(cleanup_hierarchy))
        .add_system_to_stage(GameStage::Cleanup, cleanup_movable_cubes.after(cleanup_hierarchy))
        .add_system(debug_random_side_move_event)
        // .add_system(debug_print_global_transform.after(bevy::transform::TransformSystem::TransformPropagate))
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

fn establish_hierarchy(
    mut commands: Commands,
    movable_cubes: Query<Entity, With<MovableCube>>,
    center: Query<Entity, With<Center>>,
) {
    // 组建父子关系
    for center_entity in &center {
        for cube_entity in &movable_cubes {
            commands.entity(center_entity).add_child(cube_entity);
        }
    }
}

fn cleanup_hierarchy(
    mut commands: Commands,
    mut movable_cubes: Query<(Entity, &mut Transform, &GlobalTransform), With<MovableCube>>,
) {
    // 清除父子关系
    for (cube_entity, mut transform, global_transform) in &mut movable_cubes {
        commands.entity(cube_entity).remove_parent();

        // 更新transform，不再是相对parent位置
        // transform.translation = global_transform.translation();
        info!("cleanup_hierarchy - cube={:?}, transform={}, global transform={}", cube_entity, transform.translation, global_transform.translation());
        transform.translation = Vec3::new(
            global_transform.translation().x.round(),
            global_transform.translation().y.round(),
            global_transform.translation().z.round(),
        );
        info!("cleanup_hierarchy - updated cube={:?}, transform={}, global transform={}", cube_entity, transform.translation, global_transform.translation());
    }
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
