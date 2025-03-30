use std::{
    fs::File,
    io::Read,
    path::Path,
    sync::{Arc, RwLock},
};

use bevy::{prelude::*, utils::HashMap};

use crate::simulation::body::{Body, BodyType};

use super::body::{BodyMetadata, MetaLoader};

/// Initialize the celestial bodies with their initial data.
/// We get the data using the JPL Horizons API from our pretty little Python script.
pub fn initialize_bodies(asset_server: &Res<AssetServer>) -> Option<Vec<Body>> {
    let metadata_map: HashMap<&'static str, MetaLoader> = HashMap::from([
        (
            "Sun",
            MetaLoader::new(Some(asset_server.load("sun.jpg")), BodyType::Star),
        ),
        ("Mercury", MetaLoader::new(None, BodyType::Planet)),
        ("Venus", MetaLoader::new(None, BodyType::Planet)),
        ("Earth", MetaLoader::new(None, BodyType::Planet)),
        /**/ ("Moon", MetaLoader::new(None, BodyType::Moon)),
        ("Mars", MetaLoader::new(None, BodyType::Planet)),
        /**/ ("Phobos", MetaLoader::new(None, BodyType::Moon)),
        /**/ ("Deimos", MetaLoader::new(None, BodyType::Moon)),
        ("Jupiter", MetaLoader::new(None, BodyType::Planet)),
        /**/ ("Io", MetaLoader::new(None, BodyType::Moon)),
        /**/ ("Europa", MetaLoader::new(None, BodyType::Moon)),
        /**/ ("Ganymede", MetaLoader::new(None, BodyType::Moon)),
        /**/ ("Callisto", MetaLoader::new(None, BodyType::Moon)),
        /**/ ("Amalthea", MetaLoader::new(None, BodyType::Moon)),
        /**/ ("Thebe", MetaLoader::new(None, BodyType::Moon)),
        /**/ ("Adrastea", MetaLoader::new(None, BodyType::Moon)),
        /**/ ("Metis", MetaLoader::new(None, BodyType::Moon)),
        ("Saturn", MetaLoader::new(None, BodyType::Planet)),
        ("Uranus", MetaLoader::new(None, BodyType::Planet)),
        ("Neptune", MetaLoader::new(None, BodyType::Planet)),
        ("Pluto", MetaLoader::new(None, BodyType::DwarfPlanet)),
        ("Ceres", MetaLoader::new(None, BodyType::DwarfPlanet)),
        ("Eris", MetaLoader::new(None, BodyType::DwarfPlanet)),
        ("Haumea", MetaLoader::new(None, BodyType::DwarfPlanet)),
        ("Makemake", MetaLoader::new(None, BodyType::DwarfPlanet)),
    ]);

    let bodies = load_data();

    return match bodies {
        Some(bodies) => Some(
            bodies
                .iter()
                .map(|b| {
                    let binding = MetaLoader::default();
                    let meta = metadata_map
                        .get(b.metadata.name.as_ref().unwrap().as_str())
                        .unwrap_or(&binding);

                    Body {
                        metadata: BodyMetadata {
                            texture: meta.texture.clone(),
                            body_type: meta.body_type.clone(),
                            ..b.metadata.clone()
                        },
                        data: b.data,
                        satellites: match &b.satellites {
                            Some(satellites) => Some(
                                satellites
                                    .iter()
                                    .map(|s| {
                                        let satellite = s.read().unwrap();

                                        let s_binding = MetaLoader::default();
                                        let s_meta = metadata_map
                                            .get(satellite.metadata.name.as_ref().unwrap().as_str())
                                            .unwrap_or(&s_binding);

                                        Arc::new(RwLock::new(Body {
                                            metadata: BodyMetadata {
                                                texture: s_meta.texture.clone(),
                                                body_type: s_meta.body_type.clone(),
                                                ..satellite.metadata.clone()
                                            },
                                            data: satellite.data,
                                            satellites: None,
                                        }))
                                    })
                                    .collect(),
                            ),
                            None => None,
                        },
                    }
                })
                .collect(),
        ),
        None => None,
    };
}

pub fn load_data() -> Option<Vec<Body>> {
    let dir = std::env::current_dir().unwrap();
    let path = dir.join(Path::new("data\\compiled_data.json"));
    let data_file = File::open(path.clone());

    info!("Reading data from {}", path.to_str().unwrap());

    match data_file {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();

            let data: Vec<Body> = serde_json::from_str(&contents).unwrap();

            info!(
                "Successfully loaded data for {} bodies:",
                data.len()
                    + data.iter().fold(0, |acc, b| {
                        acc + b
                            .satellites
                            .as_ref()
                            .and_then(|s| Some(s.len()))
                            .unwrap_or(0)
                    })
            );

            for body in &data {
                let name = body
                    .metadata
                    .name
                    .clone()
                    .unwrap_or(format!("<unknown {}>", body.metadata.body_type));

                info!("    {}", name);

                for satellite in body.satellites.as_ref().unwrap() {
                    info!(
                        "        {}",
                        satellite
                            .read()
                            .unwrap()
                            .metadata
                            .name
                            .clone()
                            .unwrap_or(format!(
                                "<unknown {} (satellite of {})>",
                                body.metadata.body_type, name
                            ))
                    );
                }
            }

            Some(data)
        }
        Err(e) => {
            error!("Error opening file: {}", e);

            None
        }
    }
}
