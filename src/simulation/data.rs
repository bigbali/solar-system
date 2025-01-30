use std::{fs::File, io::Read, path::Path};

use bevy::{color::palettes::tailwind, prelude::*, utils::HashMap};

use crate::simulation::body::{Body, BodyType};

use super::body::{BodyMetadata, MetaLoader as MetaMap};

/// Initialize the celestial bodies with their initial data.
/// We get the data using the JPL Horizons API from our pretty little Python script.
pub fn initialize_bodies(asset_server: &Res<AssetServer>) -> Option<Vec<Body>> {
    let metadata_map: HashMap<&'static str, MetaMap> = HashMap::from([
        ("Mercury", MetaMap::new(None, Some(BodyType::Planet))),
        ("Venus", MetaMap::new(None, Some(BodyType::Planet))),
        ("Earth", MetaMap::new(None, Some(BodyType::Planet))),
        ("Mars", MetaMap::new(None, Some(BodyType::Planet))),
        ("Jupiter", MetaMap::new(None, Some(BodyType::Planet))),
        ("Saturn", MetaMap::new(None, Some(BodyType::Planet))),
        ("Uranus", MetaMap::new(None, Some(BodyType::Planet))),
        ("Neptune", MetaMap::new(None, Some(BodyType::Planet))),
        ("Pluto", MetaMap::new(None, Some(BodyType::DwarfPlanet))),
        ("Ceres", MetaMap::new(None, Some(BodyType::DwarfPlanet))),
        ("Eris", MetaMap::new(None, Some(BodyType::DwarfPlanet))),
        ("Haumea", MetaMap::new(None, Some(BodyType::DwarfPlanet))),
        ("Makemake", MetaMap::new(None, Some(BodyType::DwarfPlanet))),
    ]);

    let bodies = load_data();

    return match bodies {
        Some(bodies) => Some(
            bodies
                .iter()
                .map(|b| {
                    let meta = metadata_map
                        .get(b.metadata.name.as_ref().unwrap().as_str())
                        .unwrap();

                    Body {
                        metadata: BodyMetadata {
                            texture: meta.texture.clone(),
                            body_type: meta.body_type.clone().unwrap_or_default(),
                            ..b.metadata.clone()
                        },
                        data: b.data,
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
