use bevy::prelude::*;
use serde::{
    de::{self, MapAccess, Visitor},
    Deserialize, Deserializer,
};
use std::{
    cell::RefCell,
    fmt,
    sync::{Arc, RwLock},
};

use super::body::Body;

pub fn deserialize_vec3<'de, D>(deserializer: D) -> Result<Vec3, D::Error>
where
    D: Deserializer<'de>,
{
    struct Vec3Visitor;

    impl<'de> Visitor<'de> for Vec3Visitor {
        type Value = Vec3;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a struct with x, y, and z fields")
        }

        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: MapAccess<'de>,
        {
            let mut x = None;
            let mut y = None;
            let mut z = None;

            while let Some(key) = map.next_key::<&str>()? {
                match key {
                    "x" => x = Some(map.next_value()?),
                    "y" => y = Some(map.next_value()?),
                    "z" => z = Some(map.next_value()?),
                    _ => return Err(de::Error::unknown_field(key, &["x", "y", "z"])),
                }
            }

            let x = x.ok_or_else(|| de::Error::missing_field("x"))?;
            let y = y.ok_or_else(|| de::Error::missing_field("y"))?;
            let z = z.ok_or_else(|| de::Error::missing_field("z"))?;

            Ok(Vec3::new(x, y, z))
        }
    }

    deserializer.deserialize_map(Vec3Visitor)
}

pub fn deserialize_color<'de, D>(deserializer: D) -> Result<Color, D::Error>
where
    D: Deserializer<'de>,
{
    let arr: Option<[f32; 4]> = Option::deserialize(deserializer)?;

    Ok(arr
        .map(|[r, g, b, a]| Color::srgba(r, g, b, a))
        .unwrap_or_else(|| super::body::BodyMetadata::default().color))
}

pub fn deserialize_satellites<'de, D>(
    deserializer: D,
) -> Result<Option<Vec<Arc<RwLock<Body>>>>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::{Error, SeqAccess, Visitor};
    use std::fmt;

    struct SatellitesVisitor;

    impl<'de> Visitor<'de> for SatellitesVisitor {
        type Value = Option<Vec<Arc<RwLock<Body>>>>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an optional array of satellites")
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(None)
        }

        fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_seq(self)
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let mut satellites = Vec::new();
            while let Some(satellite) = seq.next_element::<Body>()? {
                satellites.push(Arc::new(RwLock::new(satellite)));
            }

            Ok(Some(satellites))
        }
    }

    deserializer.deserialize_option(SatellitesVisitor)
}
