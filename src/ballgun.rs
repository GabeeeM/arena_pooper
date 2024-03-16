use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::player::{DeleteBall, ShotBall};

pub struct BallGunPlugin;

impl Plugin for BallGunPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (fondle_balls, kill_balls));
    }
}

#[derive(Component)]
pub struct Prop;

#[derive(Resource)]
pub struct BallCount(pub i32);

fn fondle_balls(
    mut event: EventReader<ShotBall>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut ball_count: ResMut<BallCount>,
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
            Damping {
                linear_damping: 1.0,
                angular_damping: 1.0,
            },
        );

        commands.spawn(ball);
        ball_count.0 += 1;

        println!("{}", ball_count.0);
    }
}

fn kill_balls(
    mut event: EventReader<DeleteBall>,
    mut commands: Commands,
    mut ball_count: ResMut<BallCount>,
    ball_query: Query<Entity, With<Prop>>,
) {
    for DeleteBall(entity) in event.read() {
        if ball_query.get(*entity).is_ok() {
            commands.entity(*entity).despawn();
            ball_count.0 -= 1;
        }
    }
}
