use bevy:: prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn);
    }
}

fn spawn(
    mut commands: Commands,  
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(200.0, 0.2, 200.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::Srgba(Srgba::rgb_u8(137, 137, 137)),
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::from_length(2.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::Srgba(Srgba::rgb_u8(255, 165, 0)), 
            perceptual_roughness: 1.0,        
            ..default()
        })),
        Transform::from_xyz(0.0, 1.0, 0.0)
    ));

    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            illuminance: 10_000.0,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 20.0, 20.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::PI/4.0),
            ..default()
        }
    ));
}