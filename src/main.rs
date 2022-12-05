use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(move_cube)
        .add_system(rotate_cube)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::RED.into()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::GREEN.into()),
        transform: Transform::from_xyz(1.0, 0.0, 0.0),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::YELLOW.into()),
        transform: Transform::from_xyz(-1.0, 0.0, 0.0),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::BLUE.into()),
        transform: Transform::from_xyz(0.0, 0.0, 1.0),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::CYAN.into()),
        transform: Transform::from_xyz(1.0, 0.0, 1.0),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::FUCHSIA.into()),
        transform: Transform::from_xyz(-1.0, 0.0, 1.0),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::ALICE_BLUE.into()),
        transform: Transform::from_xyz(0.0, 0.0, -1.0),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::ANTIQUE_WHITE.into()),
        transform: Transform::from_xyz(1.0, 0.0, -1.0),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::BEIGE.into()),
        transform: Transform::from_xyz(-1.0, 0.0, -1.0),
        ..default()
    });

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-4.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn move_cube(mut cubes: Query<&mut Transform>, timer: Res<Time>) {
    for mut transform in &mut cubes {
        // Move the cube forward smoothly at a given move_speed.
        let forward = transform.forward();
        dbg!(forward);
        transform.translation += forward * 2.0 * timer.delta_seconds();
    }
}

fn rotate_cube(
    mut cubes: Query<&mut Transform>,
    timer: Res<Time>,
) {
    // Calculate the point to circle around. (The position of the center_sphere)
    let mut center: Vec3 = Vec3::ZERO;
    // Update the rotation of the cube(s).
    for mut transform in &mut cubes {
        // Calculate the rotation of the cube if it would be looking at the sphere in the center.
        let look_at_sphere = transform.looking_at(center, transform.local_y());
        // Interpolate between the current rotation and the fully turned rotation
        // when looking a the sphere,  with a given turn speed to get a smooth motion.
        // With higher speed the curvature of the orbit would be smaller.
        let incremental_turn_weight = 0.2 * timer.delta_seconds();
        let old_rotation = transform.rotation;
        transform.rotation = old_rotation.lerp(look_at_sphere.rotation, incremental_turn_weight);
    }
}

