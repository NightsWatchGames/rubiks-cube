use crate::cube::*;
use crate::moving::*;
use bevy::prelude::*;
use bevy_mod_picking::PickingCameraBundle;
use bevy_mod_raycast::RaycastSource;


pub fn setup_camera(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    }).insert(PickingCameraBundle::default())
    .insert(RaycastSource::<MyRaycastSet>::new());
}