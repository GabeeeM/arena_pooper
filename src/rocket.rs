use bevy::prelude::*;
use bevy_rapier3d::{
    dynamics::{RigidBody, Velocity},
    geometry::{Collider, Sensor},
    plugin::RapierContext,
};

use crate::{
    ballgun::Prop,
    player::{Grounded, Player, ShotRocket},
};
pub struct RocketPlugin;

impl Plugin for RocketPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (do_rocket, handle_rockets));
    }
}

#[derive(Component)]
struct BlastDuration(Timer);

fn do_rocket(
    mut event_listener: EventReader<ShotRocket>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for ShotRocket(position) in event_listener.read() {
        let blast = (
            PbrBundle {
                mesh: meshes.add(Sphere { radius: 0.2 }.mesh()),
                material: materials.add(Color::BLUE),
                transform: Transform::from_translation(*position),
                ..default()
            },
            BlastDuration(Timer::from_seconds(0.25, TimerMode::Once)),
            RigidBody::Fixed,
            Collider::ball(1.0),
            Sensor,
        );

        commands.spawn(blast);
    }
}

fn handle_rockets(
    mut rockets_q: Query<(&mut BlastDuration, &Transform, Entity), With<BlastDuration>>,
    mut player_q: Query<(&mut Velocity, &Transform, Entity, &mut Grounded), With<Player>>,
    mut prop_q: Query<(&mut Velocity, &Transform, Entity), (With<Prop>, Without<Player>)>,
    mut commands: Commands,
    time: Res<Time>,
    rapier_context: Res<RapierContext>,
) {
    let (mut player_velocity, player_transform, player, mut is_grounded) =
        player_q.get_single_mut().unwrap();
    for (mut timer, blast_transform, blast) in rockets_q.iter_mut() {
        timer.0.tick(time.delta());

        if timer.0.finished() {
            commands.entity(blast).despawn();
        }

        if rapier_context.intersection_pair(blast, player) == Some(true) {
            is_grounded.0 = false;
            let direction = player_transform.translation
                - (blast_transform.translation
                    + Vec3 {
                        x: 0.0,
                        y: 0.25,
                        z: 0.0,
                    });
            player_velocity.linvel += direction * time.delta_seconds() * 1000.0;
        }

        for (mut thing_velocity, thing_transform, thing) in prop_q.iter_mut() {
            if rapier_context.intersection_pair(blast, thing) == Some(true) {
                let direction = thing_transform.translation
                    - (blast_transform.translation
                        + Vec3 {
                            x: 0.0,
                            y: 0.25,
                            z: 0.0,
                        });
                thing_velocity.linvel += direction * time.delta_seconds() * 1000.0;
            }
        }
    }
}
