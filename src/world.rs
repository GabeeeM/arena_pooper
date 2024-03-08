use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_floor);
    }
}

fn spawn_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor = PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(100.0, 100.0)),
        material: materials.add(Color::RED),
        ..default()
    };

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
}
