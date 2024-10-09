// TODO: Implement BrushFaceContainment, rename existing FaceContainment to FaceFaceContainment
//       Use brush hulls to check against each vertex of a face

pub mod brush;
pub mod entity;
pub mod face;
pub mod texture;
pub mod line;

mod convex_hull;
mod geo_map;
mod plane_3d;

pub use convex_hull::*;
pub use geo_map::*;
pub use plane_3d::*;

pub use shalrath;

use shalrath::repr::{Point, TexturePlane};

use crate::face::FacePlanes;

const EPSILON: f64 = 0.001;

pub type Vector2 = nalgebra::Vector2<f64>;
pub type Vector3 = nalgebra::Vector3<f64>;

pub fn vector3_from_point(point: Point) -> Vector3 {
    nalgebra::vector![point.x as f64, point.y as f64, point.z as f64]
}

pub fn vector3_from_texture_plane(plane: &TexturePlane) -> Vector3 {
    nalgebra::vector![plane.x as f64, plane.y as f64, plane.z as f64]
}
