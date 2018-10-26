use Axis;

/// Represents a volume
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Volume {
    /// Useful for box shaped volumes.
    Box {
        /// Origin X coordinate.
        x: i32,
        /// Origin Y coordinate.
        y: i32,
        /// Origin Z coordinate, defaults to 0.
        #[serde(default)]
        z: i32,
        /// Width: Distance along the X axis.
        w: u32,
        /// Height: Distance along the Y axis.
        h: u32,
        /// Depth: Distance along the Z axis. Defaults to 25.
        #[serde(default = "Volume::box_default_depth")]
        d: u32,
    },
    /// Useful for tube shaped volumes.
    Cylinder {
        /// Axis the cylinder is aligned with.
        axis: Axis,
        /// Center coordinate of the cylinder.
        ///
        /// For the cylinder to be aligned with the center of the sprite:
        ///
        /// * X axis: Use the X offset pixel coordinate.
        /// * Y axis: Use the Y offset pixel coordinate.
        /// * Z axis: Use 0.
        center: i32,
        /// Radius of the cylinder.
        r: u32,
        /// Length of the cylinder.
        l: u32,
    },
    /// Useful for ball shaped volumes.
    Sphere {
        /// Origin X coordinate.
        x: i32,
        /// Origin Y coordinate.
        y: i32,
        /// Origin Z coordinate, defaults to 0.
        #[serde(default)]
        z: i32,
        /// Radius of the sphere.
        r: u32,
    },
}

impl Volume {
    /// Default depth for `Box` volumes.
    fn box_default_depth() -> u32 {
        26
    }
}
