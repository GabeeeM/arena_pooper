use bevy::{
    input::mouse::MouseMotion,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

use bevy_rapier3d::{dynamics::RigidBody, prelude::*};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, control_player)
            .add_event::<ShotRocket>()
            .add_event::<ShotBall>();
    }
}

#[derive(Component)]
pub struct PlayerCam;

#[derive(Component)]
pub struct Player {
    paused: bool,
}

#[derive(Component)]
pub struct Grounded(pub bool);

#[derive(Event)]
pub struct ShotRocket(pub Vec3);

#[derive(Event)]
pub struct ShotBall(pub Direction3d, pub Vec3);

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player = (
        PbrBundle {
            mesh: meshes.add(Sphere { radius: 0.5 }.mesh()),
            material: materials.add(Color::BISQUE),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        Player { paused: true },
        RigidBody::Dynamic,
        Collider::ball(0.5),
        Velocity::default(),
        LockedAxes::ROTATION_LOCKED,
        Ccd::enabled(),
        // Damping {
        //     linear_damping: 5.0,
        //     angular_damping: 0.0,
        // },
        Friction {
            coefficient: 0.0,
            combine_rule: CoefficientCombineRule::Min,
        },
        Grounded(true),
    );

    let camera = (
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.25, 0.0).looking_at(Vec3::X, Vec3::Y),
            ..default()
        },
        PlayerCam,
    );

    commands.spawn(player).with_children(|parent| {
        parent.spawn(camera);
    });
}

fn control_player(
    keys: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    rapier_context: Res<RapierContext>,
    time: Res<Time>,
    mut shot_rocket: EventWriter<ShotRocket>,
    mut shot_ball: EventWriter<ShotBall>,
    mut player_q: Query<
        (&mut Velocity, &mut Player, &Transform, &mut Grounded),
        (With<Player>, Without<PlayerCam>),
    >,
    mut camera_q: Query<&mut Transform, With<PlayerCam>>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut camera_transform = camera_q.get_single_mut().unwrap();
    let (mut player_velocity, mut player_stuff, player_transform, mut is_grounded) =
        player_q.get_single_mut().unwrap();

    if keys.just_pressed(KeyCode::Escape) {
        player_stuff.paused = !player_stuff.paused;
    }

    let mut primary_window = q_windows
        .get_single_mut()
        .expect("Could not grab primary window");

    is_grounded.0 = rapier_context
        .cast_ray(
            player_transform.translation,
            -Vec3::Y,
            0.5,
            true,
            QueryFilter::only_fixed(),
        )
        .is_some();

    if player_stuff.paused {
        primary_window.cursor.grab_mode = CursorGrabMode::None;
        primary_window.cursor.visible = true;
    } else {
        primary_window.cursor.grab_mode = CursorGrabMode::Locked;
        primary_window.cursor.visible = false;

        let (mut yaw, mut pitch, _) = camera_transform.rotation.to_euler(EulerRot::YXZ);

        for ev in mouse_motion.read() {
            pitch -= (ev.delta.y * 0.05).to_radians();
            yaw -= (ev.delta.x * 0.05).to_radians();

            pitch = pitch.clamp(-std::f32::consts::FRAC_PI_2, std::f32::consts::FRAC_PI_2);

            camera_transform.rotation =
                Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
        }
    }

    let mut movement = Vec3::ZERO;

    // Forward
    if keys.pressed(KeyCode::KeyW) {
        movement.x += camera_transform.forward().x;
        movement.z += camera_transform.forward().z;
    }

    // Backward
    if keys.pressed(KeyCode::KeyS) {
        movement.x += camera_transform.back().x;
        movement.z += camera_transform.back().z;
    }

    // Left
    if keys.pressed(KeyCode::KeyA) {
        movement.x += camera_transform.left().x;
        movement.z += camera_transform.left().z;
    }

    // Right
    if keys.pressed(KeyCode::KeyD) {
        movement.x += camera_transform.right().x;
        movement.z += camera_transform.right().z;
    }

    // Jump
    if keys.pressed(KeyCode::Space) && is_grounded.0 {
        player_velocity.linvel.y = 10.0;
    }

    // Right Click Rocket Mode
    if mouse_buttons.just_pressed(MouseButton::Right) {
        if let Some((_entity, point)) = rapier_context.cast_ray_and_get_normal(
            player_transform.translation
                + Vec3 {
                    x: 0.0,
                    y: 0.25,
                    z: 0.0,
                },
            Vec3::from(camera_transform.forward()),
            1000.0,
            true,
            QueryFilter::only_fixed(),
        ) {
            shot_rocket.send(ShotRocket(point.point));
        }
    }

    // Left Click Shoot Ball
    if mouse_buttons.pressed(MouseButton::Left) {
        shot_ball.send(ShotBall(
            camera_transform.forward(),
            player_transform.translation + (1.0 * Vec3::from(camera_transform.forward())),
        ));
    }

    let normalized_movement = movement.normalize_or_zero() * time.delta_seconds();
    if is_grounded.0 {
        player_velocity.linvel.x = normalized_movement.x * 150.0;
        player_velocity.linvel.z = normalized_movement.z * 150.0;
    } else {
        player_velocity.linvel.x += normalized_movement.x * 15.0;
        player_velocity.linvel.z += normalized_movement.z * 15.0;
    }
}
