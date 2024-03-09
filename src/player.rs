use bevy::{
    input::mouse::MouseMotion,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

use bevy_rapier3d::{dynamics::RigidBody, na::ComplexField, parry::query::point, prelude::*};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, control_player)
            .add_event::<ShotRocket>();
    }
}

#[derive(Component)]
pub struct PlayerCam;

#[derive(Component)]
pub struct Player {
    paused: bool,
    moving: bool,
}

#[derive(Event)]
pub struct ShotRocket(pub Vec3);

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
        Player {
            paused: true,
            moving: false,
        },
        RigidBody::Dynamic,
        Collider::ball(0.5),
        Velocity::default(),
        LockedAxes::ROTATION_LOCKED,
        Ccd::enabled(),
        Damping {
            linear_damping: 5.0,
            angular_damping: 0.0,
        },
        Friction {
            coefficient: 0.0,
            combine_rule: CoefficientCombineRule::Min,
        },
    );

    let camera = (
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.5, 0.0).looking_at(Vec3::X, Vec3::Y),
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
    mut player_q: Query<
        (&mut Velocity, &mut Player, &Transform, &mut Damping),
        (With<Player>, Without<PlayerCam>),
    >,
    mut camera_q: Query<(&mut Transform), With<PlayerCam>>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let (mut camera_transform) = camera_q.get_single_mut().unwrap();
    let (mut player_velocity, mut player_stuff, player_transform, mut player_damping) =
        player_q.get_single_mut().unwrap();

    if keys.just_pressed(KeyCode::Escape) {
        player_stuff.paused = !player_stuff.paused;
    }

    let mut primary_window = q_windows
        .get_single_mut()
        .expect("Could not grab primary window");

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
    player_stuff.moving = false;

    // Forward
    if keys.pressed(KeyCode::KeyW) {
        movement.x += camera_transform.forward().x;
        movement.z += camera_transform.forward().z;
        player_stuff.moving = true;
    }

    // Backward
    if keys.pressed(KeyCode::KeyS) {
        movement.x += camera_transform.back().x;
        movement.z += camera_transform.back().z;
        player_stuff.moving = true;
    }

    // Left
    if keys.pressed(KeyCode::KeyA) {
        movement.x += camera_transform.left().x;
        movement.z += camera_transform.left().z;
        player_stuff.moving = true;
    }

    // Right
    if keys.pressed(KeyCode::KeyD) {
        movement.x += camera_transform.right().x;
        movement.z += camera_transform.right().z;
        player_stuff.moving = true;
    }

    // Jump
    if keys.just_pressed(KeyCode::Space) {
        player_velocity.linvel.y += 10.0;
    }

    // Damping I hate This
    if player_velocity.linvel.y.abs().round() != 0.0 {
        player_damping.linear_damping = 0.0;
    } else {
        player_damping.linear_damping = 5.0;
    }

    // Right Click Rocket Mode
    if mouse_buttons.just_pressed(MouseButton::Right) {
        if let Some((_entity, point)) = rapier_context.cast_ray_and_get_normal(
            player_transform.translation
                + Vec3 {
                    x: 0.0,
                    y: 0.5,
                    z: 0.0,
                },
            Vec3::from(camera_transform.forward()),
            10.0,
            true,
            QueryFilter::only_fixed(),
        ) {
            shot_rocket.send(ShotRocket(point.point));
        }
    }

    if (player_stuff.moving) {
        let poo = movement.normalize_or_zero() * 1000.0 * time.delta_seconds();
        player_velocity.linvel.x = poo.x;
        player_velocity.linvel.z = poo.z;
    }
}
