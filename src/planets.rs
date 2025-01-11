use bevy::{
    asset::AssetServer,
    color::{palettes::tailwind, Color},
    math::Vec3,
    prelude::Res,
};

use crate::body::{Body, BodyData, BodyMetadata};

pub fn planets(asset_server: &Res<AssetServer>) -> Vec<Body> {
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

    let earth = Body {
        data: BodyData {
            position: Vec3::new(152_097_597.0, 0.0, 0.0),
            velocity: Vec3::new(0.0, 0.0, 29.7827),
            acceleration: Vec3::new(0.0, 0.0, 0.0),
            rotation: Vec3::new(0.0, 0.0, 0.0),
            axial_tilt: Vec3::new(0.0, 0.0, 0.0),
            mass: 5.97237e24,
            radius: 6_371.0,
            temperature: 5778.0,
        }
        .downscaled(),
        metadata: BodyMetadata {
            color: Color::from(tailwind::BLUE_500),
            name: Some("Earth"),
            texture: Some(asset_server.load("earth.jpg")),
        },
    };

    let mars = Body {
        data: BodyData {
            position: Vec3::new(249_261_000.0, 0.0, 0.0),
            velocity: Vec3::new(0.0, 0.0, 24.07),
            acceleration: Vec3::new(0.0, 0.0, 0.0),
            rotation: Vec3::new(0.0, 0.0, 0.0),
            axial_tilt: Vec3::new(0.0, 0.0, 0.0),
            mass: 6.4171e23,
            radius: 3_389.5,
            temperature: 5778.0,
        }
        .downscaled(),
        metadata: BodyMetadata {
            color: Color::from(tailwind::RED_500),
            name: Some("Mars"),
            texture: None,
        },
    };

    let jupiter = Body {
        data: BodyData {
            position: Vec3::new(816_081_455.0, 0.0, 0.0),
            velocity: Vec3::new(0.0, 0.0, 13.07),
            acceleration: Vec3::new(0.0, 0.0, 0.0),
            rotation: Vec3::new(0.0, 0.0, 0.0),
            axial_tilt: Vec3::new(0.0, 0.0, 0.0),
            mass: 1.8982E27,
            radius: 142_984.0,
            temperature: 5778.0,
        }
        .downscaled(),
        metadata: BodyMetadata {
            color: Color::from(tailwind::YELLOW_500),
            name: Some("Jupiter"),
            texture: None,
        },
    };

    let saturn = Body {
        data: BodyData {
            position: Vec3::new(1_514_500_000.0, 0.0, 0.0),
            velocity: Vec3::new(0.0, 0.0, 30.01),
            acceleration: Vec3::new(0.0, 0.0, 0.0),
            rotation: Vec3::new(0.0, 0.0, 0.0),
            axial_tilt: Vec3::new(0.0, 0.0, 0.0),
            mass: 5.6834e26,
            radius: 58_232.0,
            temperature: 5778.0,
        }
        .downscaled(),
        metadata: BodyMetadata {
            color: Color::from(tailwind::YELLOW_300),
            name: Some("Saturn"),
            texture: None,
        },
    };

    let uranus = Body {
        data: BodyData {
            position: Vec3::new(3_006_390_000.0, 0.0, 0.0),
            velocity: Vec3::new(0.0, 0.0, 50.0),
            acceleration: Vec3::new(0.0, 0.0, 0.0),
            rotation: Vec3::new(0.0, 0.0, 0.0),
            axial_tilt: Vec3::new(0.0, 0.0, 0.0),
            mass: 8.6810e25,
            radius: 25_362.0,
            temperature: 5778.0,
        }
        .downscaled(),
        metadata: BodyMetadata {
            color: Color::from(tailwind::TEAL_500),
            name: Some("Uranus"),
            texture: None,
        },
    };

    let neptune = Body {
        data: BodyData {
            position: Vec3::new(4_540_000_000.0, 0.0, 0.0),
            velocity: Vec3::new(0.0, 0.0, 80.0),
            acceleration: Vec3::new(0.0, 0.0, 0.0),
            rotation: Vec3::new(0.0, 0.0, 0.0),
            axial_tilt: Vec3::new(0.0, 0.0, 0.0),
            mass: 1.02413e26,
            radius: 24_622.0,
            temperature: 5778.0,
        }
        .downscaled(),
        metadata: BodyMetadata {
            color: Color::from(tailwind::BLUE_700),
            name: Some("Neptune"),
            texture: None,
        },
    };

    vec![
        mercury, venus, earth, mars, jupiter, saturn, uranus, neptune,
    ]
}
