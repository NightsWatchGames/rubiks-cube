use std::f32::consts::TAU;
use bevy::prelude::*;
use bevy::transform::TransformSystem::TransformPropagate;
use rand::seq::SliceRandom;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(StageLabel)]
enum GameStage {
    ChooseCubes,
    EstablishHierarchy,
    Cleanup,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)

        // 自定义stage
        .add_stage_before(CoreStage::Update, GameStage::ChooseCubes, SystemStage::parallel())
        .add_stage_before(CoreStage::Update, GameStage::EstablishHierarchy, SystemStage::parallel())
        .add_stage_after(CoreStage::PostUpdate, GameStage::Cleanup, SystemStage::parallel())

        .add_system_to_stage(GameStage::ChooseCubes, debug_random_layer)
        .add_system_to_stage(GameStage::EstablishHierarchy, establish_hierarchy)
        .add_system_to_stage(CoreStage::Update, rotate_cube)
        .add_system_to_stage(GameStage::Cleanup, cleanup_center)
        .add_system_to_stage(GameStage::Cleanup, cleanup_movable_cubes)
        .add_system_to_stage(GameStage::Cleanup, cleanup_hierarchy)
        .run();
}

#[derive(Debug, Component)]
struct Center;

#[derive(Debug, Component)]
struct MovableCube;

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
                material: materials.add(Color::YELLOW.into()),
                transform: Transform::from_translation(Vec3::new(x, y, z)),
                ..default()
            });
        }
    }

    let y = 1.0;
    for x in [-1.0, 0.0, 1.0] {
        for z in [-1.0, 0.0, 1.0] {
            commands
            .spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                material: materials.add(Color::RED.into()),
                transform: Transform::from_translation(Vec3::new(x, y, z)),
                ..default()
            });
        }
    }

    let y = -1.0;
    for x in [-1.0, 0.0, 1.0] {
        for z in [-1.0, 0.0, 1.0] {
            commands
            .spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                material: materials.add(Color::BLUE.into()),
                transform: Transform::from_translation(Vec3::new(x, y, z)),
                ..default()
            });
        }
    }

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(5.0, 5.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn establish_hierarchy(mut commands: Commands, cubes: Query<Entity, With<MovableCube>>, center: Query<Entity, With<Center>>) {
    // 组建父子关系
    for cube_entity in &cubes {
        commands.entity(center.single()).add_child(cube_entity);
    }
}

fn cleanup_hierarchy(mut commands: Commands, cubes: Query<Entity, With<MovableCube>>) {
    // 清除父子关系
    for cube_entity in &cubes {
        commands.entity(cube_entity).remove_parent();
    }
}

fn rotate_cube(mut center: Query<&mut Transform, With<Center>>, time: Res<Time>) {
    // 旋转父物体
    center.single_mut().rotate_y(0.3 * TAU * time.delta_seconds());
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

fn debug_random_layer(mut commands: Commands, cubes: Query<(Entity, &mut Transform)>) {
    let layers = vec![-1.0f32, 0.0, 1.0];
    let choice = layers.choose(&mut rand::thread_rng()).unwrap().clone();
    for (entity, transform) in &cubes {
        if transform.translation.y == choice {
            if transform.translation.x == 0.0 && transform.translation.z == 0.0 {
                commands.entity(entity).insert(Center);
            } else {
                commands.entity(entity).insert(MovableCube);
            }
        }
    }
}