use std::f32::consts::TAU;

use bevy::prelude::*;
use bevy::time::FixedTimestep;

use bevy::transform::TransformSystem::TransformPropagate;

const TIME_STEP: f32 = 1.0 / 20.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system_to_stage(CoreStage::PreUpdate, establish_hierarchy)
        .add_system_to_stage(CoreStage::Update, rotate_cube)
        .add_system_to_stage(CoreStage::PostUpdate, clear_hierarchy.after(TransformPropagate))
        .run();
}

#[derive(Debug, Component)]
struct CenterCube;

#[derive(Debug, Component)]
struct Cube;

#[derive(Debug, Component)]
struct FixedLookingAtCenter(Transform);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // cube
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::RED.into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        })
        .insert(CenterCube);
    
    let transform = Transform::from_xyz(1.0, 0.0, 0.0); 
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::GREEN.into()),
            transform: transform.clone(),
            ..default()
        })
        .insert(Cube)
        .insert(FixedLookingAtCenter (transform.looking_at(Vec3::ZERO, transform.local_y())));
    let transform = Transform::from_xyz(-1.0, 0.0, 0.0); 
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::YELLOW.into()),
            transform: transform.clone(),
            ..default()
        })
        .insert(Cube)
        .insert(FixedLookingAtCenter (transform.looking_at(Vec3::ZERO, transform.local_y())));

    let transform = Transform::from_xyz(0.0, 0.0, 1.0); 
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::BLUE.into()),
            transform: transform.clone(),
            ..default()
        })
        .insert(Cube)
        .insert(FixedLookingAtCenter (transform.looking_at(Vec3::ZERO, transform.local_y())));
    let transform = Transform::from_xyz(1.0, 0.0, 1.0);
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::CYAN.into()),
            transform: transform.clone(),
            ..default()
        })
        .insert(Cube)
        .insert(FixedLookingAtCenter (transform.looking_at(Vec3::ZERO, transform.local_y())));
    let transform = Transform::from_xyz(-1.0, 0.0, 1.0);
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::FUCHSIA.into()),
            transform: transform.clone(),
            ..default()
        })
        .insert(Cube)
        .insert(FixedLookingAtCenter (transform.looking_at(Vec3::ZERO, transform.local_y())));

    let transform = Transform::from_xyz(0.0, 0.0, -1.0);
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::ALICE_BLUE.into()),
            transform: transform.clone(),
            ..default()
        })
        .insert(Cube)
        .insert(FixedLookingAtCenter (transform.looking_at(Vec3::ZERO, transform.local_y())));
    let transform = Transform::from_xyz(1.0, 0.0, -1.0);
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::ANTIQUE_WHITE.into()),
            transform: transform.clone(),
            ..default()
        })
        .insert(Cube)
        .insert(FixedLookingAtCenter (transform.looking_at(Vec3::ZERO, transform.local_y())));
    let transform = Transform::from_xyz(-1.0, 0.0, -1.0);
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::BEIGE.into()),
            transform: transform.clone(),
            ..default()
        })
        .insert(Cube)
        .insert(FixedLookingAtCenter (transform.looking_at(Vec3::ZERO, transform.local_y())));

    // camera
    commands.spawn(Camera3dBundle {
        // 从上往下俯视
        transform: Transform::from_xyz(0.0, 10.0, 0.0).looking_at(Vec3::ZERO, Vec3::Z),
        ..default()
    });
}

fn establish_hierarchy(mut commands: Commands, cubes: Query<Entity, With<Cube>>, center: Query<Entity, With<CenterCube>>) {
    // 组建父子关系
    for cube_entity in &cubes {
        commands.entity(center.single()).add_child(cube_entity);
    }
}

fn clear_hierarchy(mut commands: Commands, cubes: Query<Entity, With<Cube>>) {
    // 清除父子关系
    for cube_entity in &cubes {
        commands.entity(cube_entity).remove_parent();
    }
}

fn rotate_cube(mut center: Query<&mut Transform, With<CenterCube>>, time: Res<Time>) {
    // 旋转父物体
    center.single_mut().rotate_y(0.3 * TAU * time.delta_seconds());
}