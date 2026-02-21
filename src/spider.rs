use bevy::{color::palettes::css, prelude::*};

use crate::{kin::IkChain, leg::AnimatedLeg, rotations, terrain::TerrainColliderCache};

const SPAWN_POSITION: Vec3 = Vec3::new(-2.0, 1.0, 2.0);
const MOVE_SPEED: f32 = 6.0;

const LEG_TARGET_OFFSET: Vec3 = Vec3::new(4.0, -0.5, 0.0);
const LEG_ERROR_THRESHOLD: f32 = 12.0;

const BODY_COLOR: Color = Color::Srgba(css::BLACK);
const LEGS_COLOR: Color = Color::Srgba(css::DARK_GRAY);

const BODY_HEIGHT_OFFSET: f32 = 1.5;
const BODY_HEIGHT_LERP: f32 = 0.1;

pub struct SpiderPlugin;

impl Plugin for SpiderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_spider).add_systems(
            Update,
            (
                move_from_input,
                adjust_body_height,
                update_leg_error,
                retarget_if_threshold_reached,
                retarget_legs_to_terrain,
                position_leg_pieces_on_chain,
                apply_appearance_changes,
            ),
        );
    }
}

#[derive(Component)]
pub struct Spider {
    combined_leg_position_error: f32,
    movement_group: u8,
}

impl Spider {
    fn switch_movement_group(&mut self) {
        self.movement_group = match self.movement_group {
            1 => 2,
            _ => 1,
        };
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpiderShapeType {
    Box,
    Sphere,
    Flat,
    Long,
}

#[derive(Clone)]
pub struct SpiderColorConfig {
    pub body_color: Color,
    pub leg_color: Color,
}

impl Default for SpiderColorConfig {
    fn default() -> Self {
        Self {
            body_color: Color::BLACK,
            leg_color: Color::srgb(0.25, 0.25, 0.25),
        }
    }
}

#[derive(Component)]
pub struct SpiderAppearance {
    pub shape: SpiderShapeType,
    pub color_config: SpiderColorConfig,
    pub dirty: bool,
}

impl Default for SpiderAppearance {
    fn default() -> Self {
        Self {
            shape: SpiderShapeType::Box,
            color_config: SpiderColorConfig::default(),
            dirty: false,
        }
    }
}

#[derive(Component)]
pub struct SpiderBody;

#[derive(Component)]
struct SpiderLeg {
    movement_group: u8,
}

struct LegSpawnInfo {
    position_offset: Vec3,
    angle_offset: f32,
    movement_group: u8,
}

impl LegSpawnInfo {
    fn new(pos: Vec3, angle: f32, group: u8) -> Self {
        LegSpawnInfo {
            position_offset: pos,
            angle_offset: angle,
            movement_group: group,
        }
    }
}

#[derive(Component)]
struct LegPiece {
    index_in_chains: u8,
}

impl LegPiece {
    fn new(position_in_chain: u8) -> Self {
        Self {
            index_in_chains: position_in_chain,
        }
    }
}

fn get_body_mesh(shape: SpiderShapeType) -> Mesh {
    match shape {
        SpiderShapeType::Box => Cuboid::new(1.4, 0.8, 1.8).into(),
        SpiderShapeType::Sphere => Sphere::new(0.9).into(),
        SpiderShapeType::Flat => Cuboid::new(2.0, 0.4, 2.2).into(),
        SpiderShapeType::Long => Cuboid::new(1.0, 0.6, 2.8).into(),
    }
}

fn spawn_spider(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let appearance = SpiderAppearance::default();

    let mesh = meshes.add(get_body_mesh(appearance.shape));
    let material = materials.add(StandardMaterial {
        base_color: appearance.color_config.body_color,
        perceptual_roughness: 1.0,
        ..default()
    });

    commands
        .spawn((
            Spider {
                combined_leg_position_error: 0.0,
                movement_group: 2,
            },
            Transform::from_translation(SPAWN_POSITION),
            Mesh3d(mesh),
            MeshMaterial3d(material),
            SpiderBody,
        ))
        .with_children(|spider| spawn_spider_legs(spider, &mut meshes, &mut materials));
}

fn spawn_spider_legs(
    spider: &mut ChildSpawnerCommands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Cuboid::new(0.2, 0.2, 3.2));

    let material = materials.add(StandardMaterial {
        base_color: LEGS_COLOR,
        perceptual_roughness: 1.0,
        ..default()
    });

    let base_points = vec![
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 3.0, 0.0),
        Vec3::new(2.0, 0.0, 0.0),
    ];

    let legs_data = [
        LegSpawnInfo::new(vec3(0.5, 0.0, -0.8), 40.0, 1),
        LegSpawnInfo::new(vec3(0.5, 0.0, -0.4), 10.0, 2),
        LegSpawnInfo::new(vec3(0.5, 0.0, 0.4), -10.0, 1),
        LegSpawnInfo::new(vec3(0.5, 0.0, 0.8), -40.0, 2),
        LegSpawnInfo::new(vec3(-0.5, 0.0, -0.8), 140.0, 2),
        LegSpawnInfo::new(vec3(-0.5, 0.0, -0.4), 170.0, 1),
        LegSpawnInfo::new(vec3(-0.5, 0.0, 0.4), 190.0, 2),
        LegSpawnInfo::new(vec3(-0.5, 0.0, 0.8), 220.0, 1),
    ];

    for data in legs_data.iter() {
        let rotation = Quat::from_axis_angle(Vec3::Y, data.angle_offset.to_radians());
        let points_of_current_leg = base_points
            .iter()
            .map(|point| SPAWN_POSITION + data.position_offset + (rotation * *point))
            .collect::<Vec<Vec3>>();

        let start = base_points[0];
        let target = start + (rotation * LEG_TARGET_OFFSET);

        spider
            .spawn((
                IkChain::new(points_of_current_leg),
                AnimatedLeg::new(rotation * LEG_TARGET_OFFSET, target),
                SpiderLeg {
                    movement_group: data.movement_group,
                },
                Transform::default(),
                Visibility::default(),
            ))
            .with_children(|chain| {
                chain.spawn((
                    Transform::from_translation(SPAWN_POSITION),
                    Mesh3d(mesh.clone()),
                    MeshMaterial3d(material.clone()),
                    LegPiece::new(0),
                ));

                chain.spawn((
                    Transform::from_translation(SPAWN_POSITION),
                    Mesh3d(mesh.clone()),
                    MeshMaterial3d(material.clone()),
                    LegPiece::new(1),
                ));
            });
    }
}

fn move_from_input(
    mut spider: Query<(&mut Transform, &Children), With<Spider>>,
    mut spider_legs: Query<&mut IkChain, With<SpiderLeg>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let (mut transform, children) = spider.single_mut().unwrap();

    let move_input = get_wasd_input_as_vec(&input);
    let delta_position = move_input * time.delta_secs() * MOVE_SPEED;

    transform.translation += delta_position;

    for child_id in children.iter() {
        if let Ok(mut leg) = spider_legs.get_mut(child_id) {
            leg.move_start(delta_position);
        }
    }
}

fn adjust_body_height(
    mut spider: Query<&mut Transform, With<Spider>>,
    terrain_cache: Option<Res<TerrainColliderCache>>,
) {
    let Some(cache) = terrain_cache else { return };
    let mut transform = spider.single_mut().unwrap();

    let terrain_height = cache.sample_height(transform.translation.x, transform.translation.z);
    let target_y = terrain_height + BODY_HEIGHT_OFFSET;

    transform.translation.y =
        transform.translation.y + (target_y - transform.translation.y) * BODY_HEIGHT_LERP;
}

fn get_wasd_input_as_vec(input: &Res<ButtonInput<KeyCode>>) -> Vec3 {
    let mut result = Vec3::ZERO;

    if input.pressed(KeyCode::KeyW) {
        result.z -= 1.0;
    }

    if input.pressed(KeyCode::KeyA) {
        result.x -= 1.0;
    }

    if input.pressed(KeyCode::KeyS) {
        result.z += 1.0;
    }

    if input.pressed(KeyCode::KeyD) {
        result.x += 1.0;
    }

    result.normalize_or_zero()
}

fn update_leg_error(
    mut spider: Query<(&mut Spider, &Children)>,
    spider_legs: Query<(&IkChain, &AnimatedLeg), With<SpiderLeg>>,
) {
    let (mut spider, children) = spider.single_mut().unwrap();

    let combined_error = children
        .iter()
        .filter_map(|child| spider_legs.get(child).ok())
        .fold(0.0, |combined, (chain, leg)| {
            combined + (chain.start + leg.reposition_target_offset).distance(leg.current_target)
        });

    spider.combined_leg_position_error = combined_error;
}

fn retarget_if_threshold_reached(
    mut spider: Query<(&mut Spider, &Children)>,
    mut spider_legs: Query<(&IkChain, &mut AnimatedLeg, &SpiderLeg)>,
) {
    let (mut spider, children) = spider.single_mut().unwrap();

    if spider.combined_leg_position_error > LEG_ERROR_THRESHOLD {
        spider.switch_movement_group();

        for child_id in children.iter() {
            if let Ok((chain, mut leg, spider_leg)) = spider_legs.get_mut(child_id) {
                if spider_leg.movement_group == spider.movement_group {
                    let target = chain.start + leg.reposition_target_offset;
                    leg.set_new_target(target);
                }
            }
        }
    }
}

fn retarget_legs_to_terrain(
    spider: Query<&Children, With<Spider>>,
    mut spider_legs: Query<(&IkChain, &mut AnimatedLeg, &SpiderLeg)>,
    terrain_cache: Option<Res<TerrainColliderCache>>,
) {
    let Some(cache) = terrain_cache else { return };
    let children = spider.single().unwrap();

    for child_id in children.iter() {
        if let Ok((_chain, mut leg, _spider_leg)) = spider_legs.get_mut(child_id) {
            if leg.lerp_fraction >= 1.0 {
                let terrain_h = cache.sample_height(leg.current_target.x, leg.current_target.z);
                leg.current_target.y = terrain_h;
            }
        }
    }
}

fn position_leg_pieces_on_chain(
    spider_legs: Query<(&IkChain, &GlobalTransform, &Children), With<SpiderLeg>>,
    mut leg_pieces: Query<(&LegPiece, &mut Transform)>,
) {
    for (chain, global_transform, children) in spider_legs.iter() {
        for child_id in children.iter() {
            if let Ok((leg, mut transform)) = leg_pieces.get_mut(child_id) {
                let segment = chain.get_segment(leg.index_in_chains as usize);

                let segment_direction = (segment.end - segment.start).normalize_or_zero();
                let segment_orientation = rotations::looking_towards(segment_direction, Vec3::Y);
                let segment_middle = segment.start + segment_direction * segment.length / 2.0;

                let local_position = segment_middle - global_transform.translation();

                transform.translation = local_position;
                transform.rotation = segment_orientation;
            }
        }
    }
}

fn apply_appearance_changes(
    mut spider: Query<
        (
            &mut SpiderAppearance,
            &mut Mesh3d,
            &mut MeshMaterial3d<StandardMaterial>,
            &Children,
        ),
        With<Spider>,
    >,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    children_query: Query<&Children, With<SpiderLeg>>,
    mut all_leg_materials: Query<
        &mut MeshMaterial3d<StandardMaterial>,
        (With<LegPiece>, Without<Spider>),
    >,
) {
    let Ok((mut appearance, mut mesh, mut material, spider_children)) = spider.single_mut() else {
        return;
    };

    if !appearance.dirty {
        return;
    }

    appearance.dirty = false;

    *mesh = Mesh3d(meshes.add(get_body_mesh(appearance.shape)));

    *material = MeshMaterial3d(materials.add(StandardMaterial {
        base_color: appearance.color_config.body_color,
        perceptual_roughness: 1.0,
        ..default()
    }));

    let leg_mat = materials.add(StandardMaterial {
        base_color: appearance.color_config.leg_color,
        perceptual_roughness: 1.0,
        ..default()
    });

    for spider_child in spider_children.iter() {
        if let Ok(leg_children) = children_query.get(spider_child) {
            for leg_child in leg_children.iter() {
                if let Ok(mut leg_material) = all_leg_materials.get_mut(leg_child) {
                    *leg_material = MeshMaterial3d(leg_mat.clone());
                }
            }
        }
    }
}
