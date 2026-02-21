use bevy::{input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel}, prelude::*};

use crate::{rotations::{self, looking_at}, spider::Spider};

const FOLLOW_DISTANCE: f32 = 10.0;
const SPAWN_POSITION: Vec3 = Vec3::new(0.0, 6.0, 10.0);

const MOVE_LERP_VALUE: f32 = 0.05;
const ROTATE_LERP_VALUE: f32 = 0.1;
const HEIGHT_OFFSET: f32 = 4.0;

const MIN_DISTANCE: f32 = 4.0;
const MAX_DISTANCE: f32 = 30.0;
const DEFAULT_DISTANCE: f32 = 10.0;

const MIN_PITCH: f32 = -0.3;
const MAX_PITCH: f32 = 1.4;

const ORBIT_SENSITIVITY: f32 = 0.005;
const ZOOM_SENSITIVITY: f32 = 1.0;
const ZOOM_LETP: f32 = 0.1;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera).add_systems(
            Update,
            (
                handle_camera_input,
                update_target_transform,
                apply_camera_lerp,
            )
                .chain(),
        );
    }
}

#[derive(Component)]
struct SpiderCamera {
    yaw: f32,
    pitch: f32,
    distance: f32,
    target_distance: f32,
    target_position: Vec3,
    target_rotation: Quat,
}

impl SpiderCamera {
    fn new() -> Self {
        SpiderCamera {
            yaw: 0.0,
            pitch: 0.5,
            distance: DEFAULT_DISTANCE,
            target_distance: DEFAULT_DISTANCE,
            target_position: SPAWN_POSITION,
            target_rotation: Quat::IDENTITY,
        }
    }
}

fn spawn_camera(mut commands: Commands) {
    let spawn_rotation = rotations::looking_at(SPAWN_POSITION, Vec3::ZERO, Vec3::Y);

    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(SPAWN_POSITION).with_rotation(spawn_rotation),
        SpiderCamera::new(),
    ));
}

fn handle_camera_input(
    mut camera: Query<&mut SpiderCamera>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut scroll_events: EventReader<MouseWheel>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut cam = camera.single_mut().unwrap();
    
    if mouse_button.pressed(MouseButton::Right) {
        for event in mouse_motion.read() {
            cam.yaw -= event.delta.x * ORBIT_SENSITIVITY;
            cam.pitch += event.delta.y * ORBIT_SENSITIVITY;
            cam.pitch = cam.pitch.clamp(MIN_PITCH, MAX_PITCH);
        }
    } else {
        mouse_motion.read();
    }
    
    let keyboard_orbit_speed = 2.0;
    if keyboard.pressed(KeyCode::KeyQ) {
        cam.yaw += keyboard_orbit_speed * time.delta_secs();
    }
    if keyboard.pressed(KeyCode::KeyE) {
        cam.yaw -= keyboard_orbit_speed * time.delta_secs();
    }
    
    if keyboard.pressed(KeyCode::KeyR) {
        cam.pitch = (cam.pitch - keyboard_orbit_speed * time.delta_secs()).clamp(MIN_PITCH, MAX_PITCH);
    }
    if keyboard.pressed(KeyCode::KeyF) {
        cam.pitch = (cam.pitch + keyboard_orbit_speed * time.delta_secs()).clamp(MIN_PITCH, MAX_PITCH);
    }
    
    for event in scroll_events.read() {
        let scroll_amount = match event.unit {
            MouseScrollUnit::Line => event.y * ZOOM_SENSITIVITY,
            MouseScrollUnit::Pixel => event.y * ZOOM_SENSITIVITY * 0.01,
        };
        cam.target_distance = (cam.target_distance - scroll_amount).clamp(MIN_DISTANCE, MAX_DISTANCE);
    }
    
    cam.distance = cam.distance + (cam.target_distance - cam.distance) * ZOOM_LETP;
}

fn update_target_transform(
    mut camera: Query<&mut SpiderCamera>,
    spider: Query<&Transform, (With<Spider>, Without<SpiderCamera>)>
) {
    let mut cam = camera.single_mut().unwrap();
    let spider_transform = spider.single().unwrap();
    let spider_pos = spider_transform.translation;
    
    let offset = Vec3::new(
        cam.yaw.cos() * cam.pitch.cos() * cam.distance, 
        cam.pitch.sin() * cam.distance, 
        cam.yaw.sin() * cam.pitch.cos() * cam.distance
    );
    
    cam.target_position = spider_pos + offset;
    cam.target_rotation = looking_at(cam.target_position, spider_pos, Vec3::Y);
}

fn apply_camera_lerp(
    mut camera: Query<(&SpiderCamera, &mut Transform)>
) {
    let (cam, mut transform) = camera.single_mut().unwrap();
    
    transform.translation = transform.translation.lerp(cam.target_position, MOVE_LERP_VALUE);
    transform.rotation = transform.rotation.slerp(cam.target_rotation, ROTATE_LERP_VALUE);
}