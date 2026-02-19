use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn);
    }
}

fn spawn(mut commands: Commands) {
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            illuminance: 10_000.0,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 20.0, 20.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::PI / 4.0),
            ..default()
        },
    ));
}
