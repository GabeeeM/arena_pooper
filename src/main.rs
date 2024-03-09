use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

mod player;
mod rocket;
mod world;

use player::PlayerPlugin;
use rocket::RocketPlugin;
use world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((PlayerPlugin, WorldPlugin, RocketPlugin))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .run();
}
