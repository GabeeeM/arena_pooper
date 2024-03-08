use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

mod player;
mod world;

use player::PlayerPlugin;
use world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((PlayerPlugin, WorldPlugin))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .run();
}
