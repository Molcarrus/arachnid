mod camera;
mod kin;
mod leg;
mod rotations;
mod spider;
mod terrain;
mod ui;
mod world;

use bevy::prelude::*;

use camera::CameraPlugin;
use kin::IkPlugin;
use spider::SpiderPlugin;
use terrain::TerrainPlugin;
use ui::UiPlugin;
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
            UiPlugin,
        ))
        .run();
}
