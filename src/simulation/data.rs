use bevy::{color::palettes::tailwind, prelude::*};

use crate::simulation::body::{Body, BodyData, BodyMetadata, BodyType};

/// Initializes the celestial bodies with their initial positions and velocities.
/// Data source: https://ssd.jpl.nasa.gov/doc/de430_de431.html
pub fn bodies(asset_server: &Res<AssetServer>, sun: &Body) -> Vec<Body> {
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
            mass: 1.65956463e-7,
            radius: 1.6308e-5,
            temperature: 5778.0,
            ..default()
        },
        metadata: BodyMetadata {
            color: Color::from(tailwind::GRAY_500),
            name: Some("Mercury"),
            texture: None,
            body_type: BodyType::Planet,
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
            mass: 2.44699613e-6,
            radius: 4.04537843e-5,
            temperature: 5778.0,
            ..default()
        },
        metadata: BodyMetadata {
            color: Color::from(tailwind::AMBER_500),
            name: Some("Venus"),
            texture: None,
            body_type: BodyType::Planet,
        },
    };

    let earth = Body {
        data: BodyData {
            position: Vec3::new(
                0.12051741410138465477,
                -0.92583847476914859295,
                -0.40154022645315222236,
            ),
            velocity: Vec3::new(
                0.01681126830978379448,
                0.00174830923073434441,
                0.00075820289738312913,
            ),
            mass: 3.0024584e-6,
            radius: 4.26349651e-5,
            temperature: 5778.0,
            ..default()
        },
        metadata: BodyMetadata {
            color: Color::from(tailwind::BLUE_500),
            name: Some("Earth"),
            texture: Some(asset_server.load("earth.jpg")),
            body_type: BodyType::Planet,
        },
    };

    // 0.30880888469119793793-0.12920883432676397162-0.11456844475984127862-0.11456844475984127862

    let _a = -0.11456844475984127862 + 0.00450250878464055477;

    let mars = Body {
        data: BodyData {
            position: Vec3::new(
                -0.11018607714879824523,
                -1.32759945030298299295,
                -0.60588914048429142236,
            ),
            velocity: Vec3::new(
                0.01448165305704756448,
                0.00024246307683646861,
                -0.00028152072792433877,
            ),
            mass: 3.213e-7,
            radius: 2.2657408e-5,
            temperature: 5778.0,
            ..default()
        },
        metadata: BodyMetadata {
            color: Color::from(tailwind::RED_500),
            name: Some("Mars"),
            texture: None,
            body_type: BodyType::Planet,
        },
    };

    let jupiter = Body {
        data: BodyData {
            position: Vec3::new(
                -5.37970676855393644523,
                -0.83048132656339789295,
                -0.22482887442656542236,
            ),
            velocity: Vec3::new(
                0.00109201259423733748,
                -0.00651811661280738459,
                -0.00282078276229867897,
            ),
            mass: 0.000954588,
            radius: 0.000477895,
            temperature: 5778.0,
            ..default()
        },
        metadata: BodyMetadata {
            color: Color::from(tailwind::YELLOW_500),
            name: Some("Jupiter"),
            texture: None,
            body_type: BodyType::Planet,
        },
    };

    let saturn = Body {
        data: BodyData {
            position: Vec3::new(
                7.89439068290953155477,
                4.59647805517127300705,
                1.55869584283189997764,
            ),
            velocity: Vec3::new(
                -0.00321755651650091552,
                0.00433581034174662541,
                0.00192864631686015503,
            ),
            mass: 0.0002857,
            radius: 0.00038925688,
            temperature: 97.0,
            ..default()
        },
        metadata: BodyMetadata {
            color: Color::from(tailwind::YELLOW_300),
            name: Some("Saturn"),
            texture: None,
            body_type: BodyType::Planet,
        },
    };

    let uranus = Body {
        data: BodyData {
            position: Vec3::new(
                -18.26540225387235944523,
                -1.16195541867586999295,
                -0.25010605772133802236,
            ),
            velocity: Vec3::new(
                0.00022119039101561468,
                -0.00376247500810884459,
                -0.00165101502742994997,
            ),
            mass: 0.00004365,
            radius: 0.000169534499,
            temperature: 53.0,
            ..default()
        },
        metadata: BodyMetadata {
            color: Color::from(tailwind::TEAL_500),
            name: Some("Uranus"),
            texture: None,
            body_type: BodyType::Planet,
        },
    };

    let neptune = Body {
        data: BodyData {
            position: Vec3::new(
                -16.05503578023336944523,
                -23.94219155985470899295,
                -9.40015796880239402236,
            ),
            velocity: Vec3::new(
                0.00264276984798005548,
                -0.00149831255054097759,
                -0.00067904196080291327,
            ),
            mass: 0.00005149,
            radius: 0.000164587904,
            temperature: 72.0,
            ..default()
        },
        metadata: BodyMetadata {
            color: Color::from(tailwind::BLUE_700),
            name: Some("Neptune"),
            texture: None,
            body_type: BodyType::Planet,
        },
    };

    // NOTE: although Pluto is a dwarf planet, we include it here because our data is in world space,
    // while the rest of the dwarf planets are relative to the sun.
    let pluto = Body {
        data: BodyData {
            position: Vec3::new(
                -30.48331376718383944523,
                -0.8724055568410499929585470899295,
                8.91157617249954997764,
            ),
            velocity: Vec3::new(
                0.00032220737349778078,
                -0.00314357639364532859,
                -0.00107794975959731297,
            ),
            mass: 6.58086572e-9,
            radius: 7.9432949e-6,
            temperature: 44.0,
            ..default()
        },
        metadata: BodyMetadata {
            color: Color::from(tailwind::GRAY_200),
            name: Some("Pluto"),
            texture: None,
            body_type: BodyType::DwarfPlanet,
        },
    };

    // Moon is relative to Earth
    let moon = Body {
        data: BodyData {
            position: Vec3::new(
                // our data is relative to Earth
                0.12051741410138465477 + -0.00080817735147818490,
                -0.92583847476914859295 + -0.00199462998549701300,
                -0.40154022645315222236 + -0.00108726268307068900,
            ),
            velocity: Vec3::new(
                0.01681126830978379448 + 0.00060108481561422370,
                0.00174830923073434441 + -0.00016744546915764980,
                0.00075820289738312913 + -0.00008556214140094871,
            ),
            mass: 3.69432e-8,
            radius: 1.16138017e-5,
            temperature: 250.0,
            ..default()
        },
        metadata: BodyMetadata {
            color: Color::from(tailwind::GRAY_500),
            name: Some("Moon"),
            texture: None,
            body_type: BodyType::Moon,
        },
    };

    let mut all_bodies = vec![
        mercury, venus, earth, moon, mars, jupiter, saturn, uranus, neptune, pluto,
    ];

    all_bodies.extend(bodies_with_data_relative_to_sun(asset_server, sun));

    all_bodies
}

/// All the celestial bodies that have their initial positions and velocities relative to the Sun.
pub fn bodies_with_data_relative_to_sun(asset_server: &Res<AssetServer>, sun: &Body) -> Vec<Body> {
    let ceres = Body {
        data: BodyData {
            position: Vec3::new(
                1.438681809676469747,
                -2.204373633189407045,
                -1.326397853361325874,
            ),
            velocity: Vec3::new(
                0.008465406136316316,
                0.004684247977335608,
                0.000466157738595739,
            ),
            mass: 5.21e-10,
            radius: 3.1618e-6,
            temperature: 172.5,
            ..default()
        },
        metadata: BodyMetadata {
            color: Color::from(tailwind::GRAY_500),
            name: Some("Ceres"),
            texture: None,
            body_type: BodyType::DwarfPlanet,
        },
    };

    // let phobos = Body {
    //     data: BodyData {
    //         position: Vec3::new(
    //             -0.00108268701234565477,
    //             -0.00017163589889500705,
    //             -0.00026605791776697764,
    //         ),
    //         velocity: Vec3::new(
    //             0.00022119039101561468,
    //             -0.00376247500810884459,
    //             -0.00165101502742994997,
    //         ),
    //         mass: 5.4300826548e-15,
    //         radius: 1.48398e-7,
    //         temperature: 233.0,
    //         ..default()
    //     },
    //     metadata: BodyMetadata {
    //         color: Color::from(tailwind::GRAY_500),
    //         name: Some("Phobos"),
    //         texture: None,
    //         body_type: BodyType::DwarfPlanet,
    //     },
    // };

    let x = sun.data.position.x;
    let y = sun.data.position.y;
    let z = sun.data.position.z;
    let vx = sun.data.velocity.x;
    let vy = sun.data.velocity.y;
    let vz = sun.data.velocity.z;

    vec![ceres]
        .iter()
        .map(|m| Body {
            data: BodyData {
                position: Vec3::new(
                    x + m.data.position.x,
                    y + m.data.position.y,
                    z + m.data.position.z,
                ),
                velocity: Vec3::new(
                    vx + m.data.velocity.x,
                    vy + m.data.velocity.y,
                    vz + m.data.velocity.z,
                ),
                ..m.data
            },
            metadata: m.metadata.clone(),
        })
        .collect::<Vec<Body>>()
}
