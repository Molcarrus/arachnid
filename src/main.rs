mod rotations;
mod world;
mod kin;
mod leg;
mod spider;
mod camera;

use bevy::{prelude::*, window};

use camera::CameraPlugin;
use kin::IkPlugin;
use spider::SpiderPlugin;
use world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, CameraPlugin, WorldPlugin, IkPlugin, SpiderPlugin))
        .run();
}
