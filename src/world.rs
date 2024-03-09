use bevy::{prelude::*, render::mesh::shape::Cube};
use bevy_rapier3d::{dynamics::RigidBody, geometry::Collider};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_world);
    }
}

fn spawn_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor = (
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(70.0, 70.0)),
            material: materials.add(Color::RED),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(35.0, 0.0, 35.0),
    );

    let cube = (
        PbrBundle {
            mesh: meshes.add(
                Cuboid {
                    half_size: Vec3 {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                    },
                }
                .mesh(),
            ),
            material: materials.add(Color::BLACK),
            transform: Transform::from_xyz(5.0, 1.0, 5.0),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(1.0, 1.0, 1.0),
    );

    let light = PointLightBundle {
        transform: Transform::from_xyz(0.0, 5.0, 0.0),
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        ..default()
    };

    commands.spawn(floor);
    commands.spawn(light);
    commands.spawn(cube);
}
