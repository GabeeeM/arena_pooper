use bevy::{
    input::mouse::MouseMotion,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, control_player);
    }
}

#[derive(Component)]
pub struct PlayerCam;

#[derive(Component)]
struct Paused(bool);

#[derive(Component)]
struct Player;

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
        Player,
    );

    let camera = (
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.5, 0.0).looking_at(Vec3::X, Vec3::Y),
            ..default()
        },
        PlayerCam,
        Paused(true),
    );

    commands.spawn(player).with_children(|parent| {
        parent.spawn(camera);
    });
}

fn control_player(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut pause_q: Query<&mut Paused>,
    mut player_q: Query<&mut Transform, (With<Player>, Without<PlayerCam>)>,
    mut camera_q: Query<&mut Transform, With<PlayerCam>>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut pause_now = pause_q.get_single_mut().unwrap();
    if keys.just_pressed(KeyCode::Escape) {
        pause_now.0 = !pause_now.0;
    }

    let mut camera_transform = camera_q
        .get_single_mut()
        .expect("Couldnt get camera for mouse control thing");

    let mut primary_window = q_windows
        .get_single_mut()
        .expect("Could not grab primary window");

    if pause_now.0 {
        primary_window.cursor.grab_mode = CursorGrabMode::None;
        primary_window.cursor.visible = true;
    } else {
        primary_window.cursor.grab_mode = CursorGrabMode::Locked;
        primary_window.cursor.visible = false;

        let (mut yaw, mut pitch, _) = camera_transform.rotation.to_euler(EulerRot::YXZ);

        for ev in mouse_motion.read() {
            pitch -= (ev.delta.y * 0.05).to_radians();
            yaw -= (ev.delta.x * 0.05).to_radians();

            camera_transform.rotation =
                Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
        }
    }

    let mut player_transform = player_q.get_single_mut().unwrap();
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

    player_transform.translation += movement.normalize_or_zero() * time.delta_seconds();
}
