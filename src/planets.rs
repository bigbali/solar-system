use bevy::{
    color::{palettes::tailwind, Color},
    math::Vec3,
};

use crate::body::{Body, BodyData, BodyMetadata};

pub fn planets() -> Vec<Body> {
    let mercury = Body {
        data: BodyData {
            position: Vec3::new(69_820_000.0, 0.0, 0.0),
            velocity: Vec3::new(0.0, 0.0, 47.36),
            acceleration: Vec3::new(0.0, 0.0, 0.0),
            rotation: Vec3::new(0.0, 0.0, 0.0),
            axial_tilt: Vec3::new(0.0, 0.0, 0.0),
            mass: 3.3011e23,
            radius: 2_439.7,
            temperature: 5778.0,
        }
        .downscaled(),
        metadata: BodyMetadata {
            color: Color::from(tailwind::GRAY_500),
            name: Some("Mercury"),
            texture: None,
        },
    };

    let venus = Body {
        data: BodyData {
            position: Vec3::new(108_940_000.0, 0.0, 0.0),
            velocity: Vec3::new(0.0, 0.0, 35.02),
            acceleration: Vec3::new(0.0, 0.0, 0.0),
            rotation: Vec3::new(0.0, 0.0, 0.0),
            axial_tilt: Vec3::new(0.0, 0.0, 0.0),
            mass: 4.8675e24,
            radius: 6_051.8,
            temperature: 5778.0,
        }
        .downscaled(),
        metadata: BodyMetadata {
            color: Color::from(tailwind::AMBER_500),
            name: Some("Venus"),
            texture: None,
        },
    };

    vec![mercury, venus]
}
