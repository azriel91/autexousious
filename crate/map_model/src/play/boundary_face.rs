use enumflags2_derive::EnumFlags;

/// Boundary faces (individual and combinations) of a cuboid.
#[derive(Clone, Copy, Debug, EnumFlags, PartialEq)]
pub enum BoundaryFace {
    /// Left face (`x-`).
    Left = 0b000001,
    /// Right face (`x+`).
    Right = 0b000010,
    /// Bottom face (`y-`).
    Bottom = 0b000100,
    /// Top face (`y+`).
    Top = 0b001000,
    /// Back face (`z-`).
    Back = 0b010000,
    /// Front face (`z+`).
    Front = 0b100000,
}
