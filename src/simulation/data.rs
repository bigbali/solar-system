use std::{
    fs::File,
    io::Read,
    path::Path,
    sync::{Arc, RwLock},
};

use bevy::{color::palettes::tailwind, prelude::*, utils::HashMap};

use crate::simulation::body::{Body, BodyType};

use super::body::{BodyMetadata, MetaLoader};

/// Initialize the celestial bodies with their initial data.
/// We get the data using the JPL Horizons API from our pretty little Python script.
pub fn initialize_bodies(asset_server: &Res<AssetServer>) -> Option<Vec<Body>> {
    let metadata_map: HashMap<&'static str, MetaLoader> = HashMap::from([
        ("Mercury", MetaLoader::new(None, BodyType::Planet)),
        ("Venus", MetaLoader::new(None, BodyType::Planet)),
        ("Earth", MetaLoader::new(None, BodyType::Planet)),
        ("Mars", MetaLoader::new(None, BodyType::Planet)),
        ("Jupiter", MetaLoader::new(None, BodyType::Planet)),
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
                                    .map(|satellite| {
                                        let satellite = satellite.read().unwrap();

                                        let binding = MetaLoader::default();
                                        let meta = metadata_map
                                            .get(b.metadata.name.as_ref().unwrap().as_str())
                                            .unwrap_or(&binding);

                                        Arc::new(RwLock::new(Body {
                                            metadata: BodyMetadata {
                                                texture: meta.texture.clone(),
                                                body_type: meta.body_type.clone(),
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

            info!("Successfully loaded data for {} bodies", data.len());

            Some(data)
        }
        Err(e) => {
            error!("Error opening file: {}", e);

            None
        }
    }
}
