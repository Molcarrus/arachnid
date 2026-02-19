mod camera;
mod kin;
mod leg;
mod rotations;
mod spider;
mod terrain;
mod world;
mod ui;

use bevy::prelude::*;

use camera::CameraPlugin;
use kin::IkPlugin;
use spider::SpiderPlugin;
use terrain::TerrainPlugin;
use world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            CameraPlugin,
            WorldPlugin,
            IkPlugin,
            SpiderPlugin,
            TerrainPlugin,
        ))
        .run();
}
