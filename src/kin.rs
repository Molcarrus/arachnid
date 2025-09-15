use bevy::{color::palettes::css, prelude::*};

use crate::rotations;

const POINT_RADIUS: f32 = 0.3;
const POINT_COLOR: Color = Color::Srgba(css::PURPLE);
const SEGMENT_COLOR: Color = Color::Srgba(css::LIGHT_CYAN);

const DRAW_CHAIN_GIZMOS: bool = false;
const DRAW_ORIENTATION_GIZMOS: bool = false;

pub struct IkPlugin;

impl Plugin for IkPlugin {
    fn build(&self, app: &mut App) {
        
    }
}

#[derive(Component)]
pub struct IkChain {
    pub start: Vec3,
    points: Vec<Vec3>,
    lengths: Vec<f32>
}

impl IkChain {
    pub fn new(points: Vec<Vec3>) -> Self {
        if points.len() < 2 {
            panic!("Invalid vector! IK chain can't be made from {} points", points.len());
        }

        let lengths = calculate_chain_lengths(&points);

        Self {
            start: points[0],
            points,
            lengths
        }
    }

    pub fn get_segment(&self, idx: usize) -> ChainSegment {
        if idx >= self.points.len() - 1 {
            panic!("Invalid index! `get_segment` called with index: {}, but only {} points", idx, self.points.len());
        }

        ChainSegment {
            start: self.points[idx],
            end: self.points[idx + 1],
            length: self.lengths[idx]
        }
    }

    pub fn move_start(&mut self, delta: Vec3) {
        self.start += delta;
    }
}

pub struct ChainSegment {
    pub start: Vec3,
    pub end: Vec3,
    pub length: f32 
}

fn solve_chain_towards_target(
    chain: &mut IkChain,
    target: Vec3,
    iterations: i32,
    gizmos: &mut Gizmos 
) {
    for _ in 0..iterations {
        backward_fabric_pass(chain, target);
        forward_fabric_pass(chain);
        constrain_chain_orientation(chain, gizmos);
    }
}

fn forward_fabric_pass(chain: &mut IkChain) {
    let points_count = chain.points.len();

    chain.points[0] = chain.start;

    for i in 0..points_count - 1 {
        let segment = chain.get_segment(i);
        let direction = (segment.end - segment.start).normalize_or_zero();

        chain.points[i+1] = segment.start + direction * segment.length;
    }
}

fn backward_fabric_pass(
    chain: &mut IkChain, 
    target: Vec3
) {
    let points_count = chain.points.len();

    chain.points[points_count - 1] = target;

    for i in (0..points_count - 1).rev() {
        let segment = chain.get_segment(i);
        let direction = (segment.start - segment.end).normalize_or_zero();

        chain.points[i] = segment.end + direction * segment.length;
    }
}

fn constrain_chain_orientation(
    chain: &mut IkChain,
    gizmos: &mut Gizmos 
) {
    let first_point = chain.points[0];
    let last_point = chain.points[chain.points.len() - 1];
    let leg_orientation = rotations::looking_at(first_point, last_point, Vec3::Y);

    let middle_point = chain.points[1];
    let joint_orientation = rotations::looking_at(first_point, middle_point, Vec3::Y);

    let delta_orientation = leg_orientation.inverse() * joint_orientation;
    let delta_euler = delta_orientation.to_euler(EulerRot::XYZ);

    let x_adjustment = if delta_euler.0 < 0.01 {
        -delta_euler.0 + 0.01
    } else {
        0.0
    };

    let orientation_adjustment = Quat::from_euler(EulerRot::XYZ, x_adjustment, -delta_euler.1, 0.0);

    let adjusted_orientation = joint_orientation * orientation_adjustment;
    let segment = chain.get_segment(0);

    chain.points[1] = first_point + adjusted_orientation * (Vec3::NEG_Z * segment.length);

    if DRAW_ORIENTATION_GIZMOS {
        draw_orientation_gizmos(gizmos, first_point, leg_orientation);
        draw_orientation_gizmos(gizmos, first_point, joint_orientation);
    }
}

fn calculate_chain_lengths(points: &Vec<Vec3>) -> Vec<f32> {
    let mut lengths: Vec<f32> = Vec::new();

    for idx in 0..points.len() - 1 {
        let start = points[idx];
        let end = points[idx + 1];
        lengths.push(start.distance(end));
    }

    lengths
}

fn draw_ik_chain_gizmos(
    mut gizmos: Gizmos, 
    ik_chains: Query<&IkChain>
) {
    if DRAW_CHAIN_GIZMOS {
        for chain in ik_chains.iter() {
            for point in chain.points.iter() {
                gizmos.sphere(*point, POINT_RADIUS, POINT_COLOR);
            }

            for idx in 0..chain.points.len() - 1 {
                let segment = chain.get_segment(idx);
                gizmos.line(segment.start, segment.end, SEGMENT_COLOR);
            }
        }
    }
}

fn draw_orientation_gizmos(
    gizmos: &mut Gizmos, 
    position: Vec3,
    orientation: Quat 
) {
    gizmos.ray(position, orientation * Vec3::X, Color::Srgba(css::GREEN));
    gizmos.ray(position, orientation * Vec3::Y, Color::Srgba(css::RED));
    gizmos.ray(position, orientation * Vec3::Z, Color::Srgba(css::BLUE));
}