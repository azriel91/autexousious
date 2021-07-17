use kinematic_model::config::Position;
use map_model::loaded::Margins;

use crate::Comparative;

/// Functions to check if a position is within map bounds.
#[derive(Debug)]
pub struct MapBoundsChecks;

impl MapBoundsChecks {
    /// Returns whether the position is within the map margins.
    pub fn is_within_bounds(
        within_x: Comparative,
        within_y: Comparative,
        within_z: Comparative,
    ) -> bool {
        within_x == Comparative::Within
            && within_y == Comparative::Within
            && within_z == Comparative::Within
    }

    /// Returns a 3-tuple of `Comparative`s whether the position is within
    /// margins on each axis.
    pub fn position_comparative(
        map_margins: &Margins,
        position: Position<f32>,
    ) -> (Comparative, Comparative, Comparative) {
        let within_x = Self::value_comparative(map_margins.left, map_margins.right, position[0]);
        let within_y = Self::value_comparative(map_margins.bottom, map_margins.top, position[1]);
        let within_z = Self::value_comparative(map_margins.back, map_margins.front, position[2]);

        (within_x, within_y, within_z)
    }

    /// Returns whether the value is between the lower and upper limits
    /// (inclusive at both ends).
    pub fn value_comparative(lower: f32, upper: f32, value: f32) -> Comparative {
        if value >= lower {
            if value <= upper {
                Comparative::Within
            } else {
                Comparative::Above
            }
        } else {
            Comparative::Below
        }
    }
}
