use bevy::prelude::*;
use serde::{
    de::{self, MapAccess, Visitor},
    Deserialize, Deserializer,
};
use std::fmt;

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
    let arr: [f32; 4] = Deserialize::deserialize(deserializer)?;
    Ok(Color::rgba(arr[0], arr[1], arr[2], arr[3]))
}
