use enumflags2::bitflags;

/// Boundary faces (individual and combinations) of a cuboid.
#[bitflags]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BoundaryFace {
    /// Left face (`x-`).
    Left = 0b00_0001,
    /// Right face (`x+`).
    Right = 0b00_0010,
    /// Bottom face (`y-`).
    Bottom = 0b00_0100,
    /// Top face (`y+`).
    Top = 0b00_1000,
    /// Back face (`z-`).
    Back = 0b01_0000,
    /// Front face (`z+`).
    Front = 0b10_0000,
}
