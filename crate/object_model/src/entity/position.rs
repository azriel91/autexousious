use amethyst::ecs::prelude::*;

/// Position of the entity in game.
#[derive(Constructor, Clone, Copy, Debug, Default, PartialEq)]
pub struct Position {
    /// X coordinate.
    pub x: f32,
    /// Y coordinate.
    pub y: f32,
    /// Z coordinate.
    pub z: f32,
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}

impl From<(f32, f32, f32)> for Position {
    fn from(pos: (f32, f32, f32)) -> Self {
        Position::new(pos.0, pos.1, pos.2)
    }
}

impl<'a> From<&'a (f32, f32, f32)> for Position {
    fn from(pos: &'a (f32, f32, f32)) -> Self {
        Position::new(pos.0, pos.1, pos.2)
    }
}

impl From<[f32; 3]> for Position {
    fn from(pos: [f32; 3]) -> Self {
        Position::new(pos[0], pos[1], pos[2])
    }
}

impl<'a> From<&'a [f32; 3]> for Position {
    fn from(pos: &'a [f32; 3]) -> Self {
        Position::new(pos[0], pos[1], pos[2])
    }
}

#[cfg(test)]
mod test {
    use super::Position;

    #[test]
    fn from_f32_tuple() {
        assert_eq!(Position::new(1., 2.1, 1.5), (1., 2.1, 1.5).into());
        assert_eq!(Position::new(1., 2.1, 1.5), (&(1., 2.1, 1.5)).into());
    }

    #[test]
    fn from_f32_slice() {
        assert_eq!(Position::new(1., 2.1, 1.5), [1., 2.1, 1.5].into());
        assert_eq!(Position::new(1., 2.1, 1.5), (&[1., 2.1, 1.5]).into());
    }
}
