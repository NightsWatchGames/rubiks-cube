use bevy::prelude::*;
use bevy::time::FixedTimestep;

const TIME_STEP: f32 = 1.0 / 10.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        // .add_system(move_cube)
        .add_system_set(SystemSet::new().with_run_criteria(FixedTimestep::step(TIME_STEP as f64)).with_system(move_cube))
        .run();
}

#[derive(Debug, Component)]
struct CenterCube;

#[derive(Debug, Component)]
struct Cube;

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
    }).insert(CenterCube);
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    //     material: materials.add(Color::GREEN.into()),
    //     transform: Transform::from_xyz(1.0, 0.0, 0.0),
    //     ..default()
    // });
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    //     material: materials.add(Color::YELLOW.into()),
    //     transform: Transform::from_xyz(-1.0, 0.0, 0.0),
    //     ..default()
    // });

    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    //     material: materials.add(Color::BLUE.into()),
    //     transform: Transform::from_xyz(0.0, 0.0, 1.0),
    //     ..default()
    // });
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::CYAN.into()),
        transform: Transform::from_xyz(1.0, 0.0, 1.0),
        ..default()
    }).insert(Cube);
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    //     material: materials.add(Color::FUCHSIA.into()),
    //     transform: Transform::from_xyz(-1.0, 0.0, 1.0),
    //     ..default()
    // });

    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    //     material: materials.add(Color::ALICE_BLUE.into()),
    //     transform: Transform::from_xyz(0.0, 0.0, -1.0),
    //     ..default()
    // });
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    //     material: materials.add(Color::ANTIQUE_WHITE.into()),
    //     transform: Transform::from_xyz(1.0, 0.0, -1.0),
    //     ..default()
    // });
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    //     material: materials.add(Color::BEIGE.into()),
    //     transform: Transform::from_xyz(-1.0, 0.0, -1.0),
    //     ..default()
    // }).insert(Cube);

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-4.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

// 绕圆点运动
fn move_cube(mut cubes: Query<&mut Transform, With<Cube>>, timer: Res<Time>) {
    for mut transform in &mut cubes {
        dbg!(transform.translation);
        // 计算圆半径
        let r = transform.translation.distance(Vec3::splat(0.0));
        dbg!(r);
        let old_sin = transform.translation.x / r;
        // 每帧绕圆点走一定的角度
        let (sin_delta, cos_delta) = (1.0 * TIME_STEP).sin_cos();
        dbg!(sin_delta);
        dbg!(cos_delta);
        // sin(a+b) = sin(a)cos(b) + cos(a)sin(b)
        let new_sin = (transform.translation.x / r) * cos_delta + (transform.translation.z / r) * sin_delta; 
        dbg!(new_sin);

        transform.translation.x = new_sin * r;
        transform.translation.y = 0.0;
        if new_sin >= 0.0 && new_sin > old_sin {
            // sin为正且递增
            println!("第一象限");
            transform.translation.z = (r.powi(2) - transform.translation.x.powi(2)).sqrt();
        } else if new_sin >= 0.0 && new_sin < old_sin  {
            // sin为正且递减
            println!("第二象限");
            transform.translation.z = -(r.powi(2) - transform.translation.x.powi(2)).sqrt();
        } else if new_sin < 0.0 && new_sin < old_sin {
            // sin为负且递减
            println!("第三象限");
            transform.translation.z = -(r.powi(2) - transform.translation.x.powi(2)).sqrt();
        } else if new_sin < 0.0 && new_sin > old_sin {
            // sin为负且递增
            println!("第四象限");
            transform.translation.z = (r.powi(2) - transform.translation.x.powi(2)).sqrt();
        }

    }
}

