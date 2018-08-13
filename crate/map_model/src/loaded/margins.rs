use config::MapBounds;

/// Coordinates of the limits of the playable area.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, new)]
pub struct Margins {
    /// X coordinate of the map's left boundary.
    pub left: f32,
    /// X coordinate of the map's right boundary.
    pub right: f32,
    /// Y coordinate of the map's bottom boundary.
    pub bottom: f32,
    /// Y coordinate of the map's top boundary.
    pub top: f32,
    /// Z coordinate of the map's back boundary.
    pub back: f32,
    /// Z coordinate of the map's front boundary.
    pub front: f32,
}

impl From<MapBounds> for Margins {
    fn from(map_bounds: MapBounds) -> Self {
        // We add the depth to the bottom and top so that it visually shifts the origin of the z
        // axis upwards on screen.
        Margins {
            left: map_bounds.x as f32,
            right: (map_bounds.x + map_bounds.width) as f32,
            bottom: (map_bounds.y + map_bounds.depth) as f32,
            top: (map_bounds.y + map_bounds.depth + map_bounds.height) as f32,
            back: map_bounds.z as f32,
            front: (map_bounds.z + map_bounds.depth) as f32,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Margins;
    use config::MapBounds;

    #[test]
    fn from_map_bounds() {
        let map_bounds = MapBounds::new(1, 2, 3, 10, 20, 30);
        assert_eq!(
            Margins {
                left: 1.,
                right: 11.,
                bottom: 32.,
                top: 52.,
                back: 3.,
                front: 33.,
            },
            map_bounds.into()
        );
    }
}
