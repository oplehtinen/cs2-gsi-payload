use serde::de::{self, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use std::f64::EPSILON;
use std::fmt;

#[derive(Serialize, Clone, Debug)]
pub struct Coordinates {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Coordinates {
    pub fn is_zero(&self) -> bool {
        self.x.abs() < EPSILON && self.y.abs() < EPSILON && self.z.abs() < EPSILON
    }
}

// Custom visitor implementation to handle different coordinate formats
struct CoordinatesVisitor;

impl<'de> Visitor<'de> for CoordinatesVisitor {
    type Value = Coordinates;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("coordinates as a string 'x,y,z' or as a map with x,y,z fields")
    }

    // Handle string format like "x,y,z"
    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let tokens: Vec<&str> = value.split(',').map(|token| token.trim()).collect();
        if tokens.len() != 3 {
            return Err(E::custom(
                "invalid coordinates: it should be a 3 dimensions vector",
            ));
        }

        let coordinates: Result<Vec<f64>, _> = tokens
            .into_iter()
            .map(|token| {
                token
                    .parse::<f64>()
                    .map_err(|_| E::custom("parse float error"))
            })
            .collect();

        match coordinates {
            Ok(coords) => Ok(Coordinates {
                x: coords[0],
                y: coords[1],
                z: coords[2],
            }),
            Err(e) => Err(e),
        }
    }

    // Handle map format like {"x": 1.0, "y": 2.0, "z": 3.0}
    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut x: Option<f64> = None;
        let mut y: Option<f64> = None;
        let mut z: Option<f64> = None;

        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "x" => {
                    if x.is_some() {
                        return Err(de::Error::duplicate_field("x"));
                    }
                    x = Some(map.next_value()?);
                }
                "y" => {
                    if y.is_some() {
                        return Err(de::Error::duplicate_field("y"));
                    }
                    y = Some(map.next_value()?);
                }
                "z" => {
                    if z.is_some() {
                        return Err(de::Error::duplicate_field("z"));
                    }
                    z = Some(map.next_value()?);
                }
                _ => {
                    // Ignore unknown fields
                    let _: serde::de::IgnoredAny = map.next_value()?;
                }
            }
        }

        let x = x.ok_or_else(|| de::Error::missing_field("x"))?;
        let y = y.ok_or_else(|| de::Error::missing_field("y"))?;
        let z = z.ok_or_else(|| de::Error::missing_field("z"))?;

        Ok(Coordinates { x, y, z })
    }
}

// Custom deserialize implementation that handles both string and map formats
impl<'de> Deserialize<'de> for Coordinates {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(CoordinatesVisitor)
    }
}
