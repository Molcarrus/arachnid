use bevy::prelude::*;

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentTerrain>()
            .add_event::<TerrainChangeEvent>()
            .add_systems(Startup, setup_terrain)
            .add_systems(
                Update,
                (handle_terrain_change, update_terrain_collider_cache),
            );
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum TerrainType {
    #[default]
    Flat,
    Hills,
    Steps,
    Ramps,
    Rocky,
}

impl TerrainType {
    pub fn name(&self) -> &str {
        match self {
            TerrainType::Flat => "Flat",
            TerrainType::Hills => "Hills",
            TerrainType::Steps => "Steps",
            TerrainType::Ramps => "Ramps",
            TerrainType::Rocky => "Rocky",
        }
    }

    pub fn all() -> &'static [TerrainType] {
        &[
            TerrainType::Flat,
            TerrainType::Hills,
            TerrainType::Steps,
            TerrainType::Ramps,
            TerrainType::Rocky,
        ]
    }
}

#[derive(Resource)]
pub struct CurrentTerrain {
    pub terrain_type: TerrainType,
}

impl Default for CurrentTerrain {
    fn default() -> Self {
        Self {
            terrain_type: TerrainType::Flat,
        }
    }
}

#[derive(Event)]
pub struct TerrainChangeEvent {
    pub new_terrain: TerrainType,
}

#[derive(Component)]
pub struct TerrainPiece;

#[derive(Component, Clone)]
pub struct TerrainCollider {
    pub min: Vec3,
    pub max: Vec3,
}

impl TerrainCollider {
    pub fn from_cuboid(translation: Vec3, half_extents: Vec3) -> Self {
        Self {
            min: translation - half_extents,
            max: translation + half_extents,
        }
    }

    pub fn height_at(&self, x: f32, z: f32) -> Option<f32> {
        if x >= self.min.x && x <= self.max.x && z >= self.min.z && z <= self.max.z {
            Some(self.max.y)
        } else {
            None
        }
    }
}

#[derive(Resource, Default)]
pub struct TerrainColliderCache {
    pub colliders: Vec<TerrainCollider>,
}

impl TerrainColliderCache {
    pub fn sample_height(&self, x: f32, z: f32) -> f32 {
        let mut max_height = 0.0_f32;
        for collider in &self.colliders {
            if let Some(h) = collider.height_at(x, z) {
                max_height = max_height.max(h);
            }
        }

        max_height
    }

    pub fn sample_height_at_vec3(&self, pos: Vec3) -> f32 {
        self.sample_height(pos.x, pos.z)
    }
}

fn update_terrain_collider_cache(
    colliders: Query<&TerrainCollider, With<TerrainPiece>>,
    mut cache: ResMut<TerrainColliderCache>,
) {
    cache.colliders.clear();
    for collider in colliders.iter() {
        cache.colliders.push(collider.clone());
    }
}

fn setup_terrain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.init_resource::<TerrainColliderCache>();
    spawn_terrain(
        TerrainType::Flat,
        &mut commands,
        &mut meshes,
        &mut materials,
    );
}

fn handle_terrain_change(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut events: EventReader<TerrainChangeEvent>,
    mut current: ResMut<CurrentTerrain>,
    existing: Query<Entity, With<TerrainPiece>>,
) {
    for event in events.read() {
        if current.terrain_type == event.new_terrain {
            continue;
        }
        current.terrain_type = event.new_terrain;

        for entity in existing.iter() {
            commands.entity(entity).despawn();
        }

        spawn_terrain(
            event.new_terrain,
            &mut commands,
            &mut meshes,
            &mut materials,
        );
    }
}

fn spawn_terrain(
    terrain_type: TerrainType,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let ground_color = Color::Srgba(Srgba::rgb_u8(137, 137, 137));
    let accent_color = Color::Srgba(Srgba::rgb_u8(100, 120, 100));
    let rock_color = Color::Srgba(Srgba::rgb_u8(90, 85, 80));

    let ground_mat = materials.add(StandardMaterial {
        base_color: ground_color,
        perceptual_roughness: 0.9,
        ..default()
    });

    let accent_mat = materials.add(StandardMaterial {
        base_color: accent_color,
        perceptual_roughness: 0.9,
        ..default()
    });

    let rock_mat = materials.add(StandardMaterial {
        base_color: rock_color,
        perceptual_roughness: 1.0,
        ..default()
    });

    match terrain_type {
        TerrainType::Flat => {
            spawn_box(
                commands,
                meshes,
                &ground_mat,
                Vec3::new(0.0, -0.1, 0.0),
                Vec3::new(200.0, 0.2, 200.0),
            );
        }

        TerrainType::Hills => {
            spawn_box(
                commands,
                meshes,
                &ground_mat,
                Vec3::new(0.0, -0.1, 0.0),
                Vec3::new(200.0, 0.2, 200.0),
            );

            let hill_positions = [
                (Vec3::new(8.0, 0.0, 5.0), 6.0, 1.5),
                (Vec3::new(-6.0, 0.0, -8.0), 5.0, 1.0),
                (Vec3::new(15.0, 0.0, -3.0), 4.0, 0.8),
                (Vec3::new(-12.0, 0.0, 10.0), 7.0, 2.0),
                (Vec3::new(0.0, 0.0, -15.0), 5.5, 1.2),
            ];

            for (center, radius, max_height) in &hill_positions {
                let layers = 5;
                for i in 0..layers {
                    let fraction = (layers - i) as f32 / layers as f32;
                    let layer_radius = radius * fraction;
                    let layer_height = max_height * (1.0 - fraction) + 0.1;
                    let y = layer_height / 2.0;

                    spawn_box(
                        commands,
                        meshes,
                        &accent_mat,
                        Vec3::new(center.x, y, center.z),
                        Vec3::new(layer_radius * 2.0, layer_height, layer_radius * 2.0),
                    );
                }
            }
        }

        TerrainType::Steps => {
            spawn_box(
                commands,
                meshes,
                &ground_mat,
                Vec3::new(0.0, -0.1, 0.0),
                Vec3::new(200.0, 0.2, 200.0),
            );

            let step_count = 8;
            let step_width = 3.0;
            let step_depth = 2.0;
            let step_height = 0.4;

            for i in 0..step_count {
                let height = step_height * (i + 1) as f32;
                let x = step_depth * i as f32;

                spawn_box(
                    commands,
                    meshes,
                    &accent_mat,
                    Vec3::new(x, height / 2.0, 0.0),
                    Vec3::new(step_depth, height, step_depth * 2.0),
                );
            }

            for i in 0..step_count {
                let height = step_height * (i + 1) as f32;
                let z = -step_depth * i as f32 - 8.0;

                spawn_box(
                    commands,
                    meshes,
                    &accent_mat,
                    Vec3::new(-5.0, height / 2.0, z),
                    Vec3::new(step_width * 2.0, height, step_depth),
                );
            }
        }

        TerrainType::Ramps => {
            spawn_box(
                commands,
                meshes,
                &ground_mat,
                Vec3::new(0.0, -0.1, 0.0),
                Vec3::new(200.0, 0.2, 200.0),
            );

            let ramp_slices = 20;
            let ramp_length = 12.0;
            let ramp_height = 3.0;
            let ramp_width = 4.0;
            let slice_length = ramp_length / ramp_slices as f32;

            for i in 0..ramp_slices {
                let fraction = (i + 1) as f32 / ramp_slices as f32;
                let h = ramp_height * fraction;
                let x = slice_length * i as f32 + slice_length / 2.0;

                spawn_box(
                    commands,
                    meshes,
                    &accent_mat,
                    Vec3::new(x, h / 2.0, 6.0),
                    Vec3::new(slice_length, h, ramp_width),
                );
            }

            spawn_box(
                commands,
                meshes,
                &accent_mat,
                Vec3::new(ramp_length + 2.0, ramp_height / 2.0, 6.0),
                Vec3::new(4.0, ramp_height, ramp_width),
            );

            for i in 0..ramp_slices {
                let fraction = (i + 1) as f32 / ramp_slices as f32;
                let h = ramp_height * fraction * 0.7;
                let z = -(slice_length * i as f32 + slice_length / 2.0) - 5.0;

                spawn_box(
                    commands,
                    meshes,
                    &accent_mat,
                    Vec3::new(-6.0, h / 2.0, z),
                    Vec3::new(ramp_width, h, slice_length),
                );
            }
        }

        TerrainType::Rocky => {
            spawn_box(
                commands,
                meshes,
                &ground_mat,
                Vec3::new(0.0, -0.1, 0.0),
                Vec3::new(200.0, 0.2, 200.0),
            );

            let rocks = [
                (Vec3::new(3.0, 0.0, 2.0), Vec3::new(2.0, 0.6, 1.5)),
                (Vec3::new(-4.0, 0.0, 3.0), Vec3::new(1.5, 0.4, 2.0)),
                (Vec3::new(7.0, 0.0, -2.0), Vec3::new(3.0, 0.8, 2.5)),
                (Vec3::new(-2.0, 0.0, -5.0), Vec3::new(1.0, 0.3, 1.0)),
                (Vec3::new(5.0, 0.0, 6.0), Vec3::new(2.5, 1.0, 1.8)),
                (Vec3::new(-8.0, 0.0, -1.0), Vec3::new(1.8, 0.5, 1.2)),
                (Vec3::new(1.0, 0.0, -8.0), Vec3::new(2.2, 0.7, 2.0)),
                (Vec3::new(10.0, 0.0, 0.0), Vec3::new(1.5, 0.9, 1.5)),
                (Vec3::new(-6.0, 0.0, 7.0), Vec3::new(3.0, 0.6, 2.0)),
                (Vec3::new(0.0, 0.0, 10.0), Vec3::new(2.0, 0.5, 3.0)),
                (Vec3::new(12.0, 0.0, 5.0), Vec3::new(1.2, 0.4, 1.8)),
                (Vec3::new(-10.0, 0.0, -6.0), Vec3::new(2.5, 1.2, 2.5)),
                (Vec3::new(4.0, 0.0, -10.0), Vec3::new(1.8, 0.6, 1.5)),
                (Vec3::new(-3.0, 0.0, 12.0), Vec3::new(2.0, 0.8, 1.0)),
                (Vec3::new(8.0, 0.0, -8.0), Vec3::new(1.5, 0.3, 2.2)),
            ];

            for (pos, size) in &rocks {
                let y = size.y / 2.0;
                spawn_box(
                    commands,
                    meshes,
                    &rock_mat,
                    Vec3::new(pos.x, y, pos.z),
                    *size,
                );
            }
        }
    }
}

fn spawn_box(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    material: &Handle<StandardMaterial>,
    position: Vec3,
    size: Vec3,
) {
    let half = size / 2.0;
    commands.spawn((
        TerrainPiece,
        TerrainCollider::from_cuboid(position, half),
        Mesh3d(meshes.add(Cuboid::new(size.x, size.y, size.z))),
        MeshMaterial3d(material.clone()),
        Transform::from_translation(position),
    ));
}
