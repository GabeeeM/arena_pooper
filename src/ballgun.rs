use bevy::{prelude::*, transform::commands};
use bevy_rapier3d::{
    dynamics::{RigidBody, Velocity},
    geometry::Collider,
};

use crate::player::ShotBall;

pub struct BallGunPlugin;

impl Plugin for BallGunPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, fondle_balls);
    }
}

#[derive(Component)]
pub struct Prop;

fn fondle_balls(
    mut event: EventReader<ShotBall>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for ShotBall(direction, position) in event.read() {
        let ball = (
            PbrBundle {
                mesh: meshes.add(Sphere { radius: 0.15 }.mesh()),
                material: materials.add(Color::BISQUE),
                transform: Transform::from_translation(*position),
                ..default()
            },
            Velocity {
                linvel: Vec3::from(*direction) * 15.0,
                ..default()
            },
            RigidBody::Dynamic,
            Collider::ball(0.15),
            Prop,
        );

        commands.spawn(ball);
    }
}
