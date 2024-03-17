use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

mod ballgun;
mod crosshair;
mod player;
mod rocket;
mod world;

use ballgun::{BallCount, BallGunPlugin};
use crosshair::CrosshairPlugin;
use player::PlayerPlugin;
use rocket::RocketPlugin;
use world::WorldPlugin;

#[derive(Component)]
pub struct DynamicFart;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, init_resources)
        .add_plugins((
            PlayerPlugin,
            WorldPlugin,
            RocketPlugin,
            BallGunPlugin,
            CrosshairPlugin,
        ))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .run();
}

fn init_resources(mut commands: Commands) {
    commands.insert_resource(BallCount(0));
}
