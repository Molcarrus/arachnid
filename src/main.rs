mod rotations;
mod world;
mod kin;
mod leg;
mod spider;
mod camera;
mod terrain;

use bevy::prelude::*;

use camera::CameraPlugin;
use kin::IkPlugin;
use spider::SpiderPlugin;
use world::WorldPlugin;
use terrain::TerrainPlugin;

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
