pub use glm::{identity, mat3, mat4, vec2, vec3, Mat3, Mat4, Vec2, Vec3};
pub use nalgebra_glm as glm;
pub use std::f32::consts::PI;

//TODO Replace all the vecs and mats with the `vek` crate? (SIMD = performace)

mod geometry;
pub use geometry::*;

#[derive(Debug, Clone)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

pub fn rect(x: f32, y: f32, width: f32, height: f32) -> Rect {
    Rect {
        x,
        y,
        width,
        height,
    }
}

pub fn eq_float(a: f32, b: f32) -> bool {
    (a - b).abs() < std::f32::EPSILON
}