use derive_new::new;

/// Position of a layer on a map.
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Eq, new)]
#[serde(default)]
pub struct Position {
    /// X coordinate of the image on the map.
    pub x: i32,
    /// Y coordinate of the image on the map.
    pub y: i32,
    /// Z coordinate of the image on the map.
    pub z: i32,
}
