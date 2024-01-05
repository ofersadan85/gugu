mod camera;
mod player;
mod world;

use bevy::prelude::*;
use camera::CameraPlugin;
use player::PlayerPlugin;
use world::WorldPlugin;

fn main() {
    let startup_plugins = (DefaultPlugins, CameraPlugin, PlayerPlugin, WorldPlugin);
    App::new().add_plugins(startup_plugins).run();
}
