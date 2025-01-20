use bevy::{color::palettes::tailwind, prelude::*};

use crate::simulation::body::{Body, BodyData, BodyMetadata};

pub fn planets(asset_server: &Res<AssetServer>) -> Vec<Body> {
    let mercury = Body {
        data: BodyData {
            position: Vec3::new(
                0.36176271656028195477,
                -0.09078197215676599295,
                0.08571497256275117236,
            ),
            velocity: Vec3::new(
                0.00336749397200575848,
                0.02489452055768343341,
                0.01294630040970409203,
            ),
            acceleration: Vec3::new(0.0, 0.0, 0.0),
            rotation: Vec3::new(0.0, 0.0, 0.0),
            axial_tilt: Vec3::new(0.0, 0.0, 0.0),
            mass: 1.65956463e-7,
            radius: 1.6308e-5,
            temperature: 5778.0,
        },
        metadata: BodyMetadata {
            color: Color::from(tailwind::GRAY_500),
            name: Some("Mercury"),
            texture: None,
        },
    };

    let venus = Body {
        data: BodyData {
            position: Vec3::new(
                0.61275194083507215477,
                -0.34836536903362219295,
                -0.19527828667594382236,
            ),
            velocity: Vec3::new(
                0.01095206842352823448,
                0.01561768426786768341,
                0.00633110570297786403,
            ),
            acceleration: Vec3::new(0.0, 0.0, 0.0),
            rotation: Vec3::new(0.0, 0.0, 0.0),
            axial_tilt: Vec3::new(0.0, 0.0, 0.0),
            mass: 2.44699613e-6,
            radius: 4.04537843e-5,
            temperature: 5778.0,
        },
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
            mass: 3.0024584e-6,
            radius: 4.26349651e-5,
            temperature: 5778.0,
        },
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
        },
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
        },
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
        },
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
        },
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
        },
        metadata: BodyMetadata {
            color: Color::from(tailwind::BLUE_700),
            name: Some("Neptune"),
            texture: None,
        },
    };

    vec![
        mercury, venus, earth, /* mars, jupiter, saturn, uranus, neptune ,*/
    ]
}
