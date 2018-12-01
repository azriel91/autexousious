use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use amethyst::{
    core::nalgebra::Vector3,
    ecs::{storage::DenseVecStorage, Component},
};
use specs_derive::Component;

/// Position of the entity in game.
#[derive(Clone, Component, Copy, Debug, PartialEq, Eq)]
pub struct Position<S>(pub Vector3<S>)
where
    S: Clone + Copy + Debug + PartialEq + Send + Sync + 'static;

impl<S> Position<S>
where
    S: Clone + Copy + Debug + PartialEq + Send + Sync + 'static,
{
    /// Returns a new `Position` vector.
    ///
    /// # Parameters
    ///
    /// * `x`: X axis position component.
    /// * `y`: Y axis position component.
    /// * `z`: Z axis position component.
    pub fn new(x: S, y: S, z: S) -> Self {
        Position(Vector3::new(x, y, z))
    }
}

impl<S> Default for Position<S>
where
    S: Clone + Copy + Debug + Default + PartialEq + Send + Sync + 'static,
{
    fn default() -> Self {
        Position(Vector3::new(S::default(), S::default(), S::default()))
    }
}

impl<S> From<Vector3<S>> for Position<S>
where
    S: Clone + Copy + Debug + PartialEq + Send + Sync + 'static,
{
    fn from(p: Vector3<S>) -> Self {
        Position(p)
    }
}

impl<'a, S> From<&'a Vector3<S>> for Position<S>
where
    S: Clone + Copy + Debug + PartialEq + Send + Sync + 'static,
{
    fn from(p: &'a Vector3<S>) -> Self {
        Position(*p)
    }
}

impl<'a, S> From<&'a mut Vector3<S>> for Position<S>
where
    S: Clone + Copy + Debug + PartialEq + Send + Sync + 'static,
{
    fn from(p: &'a mut Vector3<S>) -> Self {
        Position(*p)
    }
}

impl<S> From<(S, S, S)> for Position<S>
where
    S: Clone + Copy + Debug + PartialEq + Send + Sync + 'static,
{
    fn from(p: (S, S, S)) -> Self {
        Position::new(p.0, p.1, p.2)
    }
}

impl<'a, S> From<&'a (S, S, S)> for Position<S>
where
    S: Clone + Copy + Debug + PartialEq + Send + Sync + 'static,
{
    fn from(p: &'a (S, S, S)) -> Self {
        Position::new(p.0, p.1, p.2)
    }
}

impl<'a, S> From<&'a mut (S, S, S)> for Position<S>
where
    S: Clone + Copy + Debug + PartialEq + Send + Sync + 'static,
{
    fn from(p: &'a mut (S, S, S)) -> Self {
        Position::new(p.0, p.1, p.2)
    }
}

impl<S> From<[S; 3]> for Position<S>
where
    S: Clone + Copy + Debug + PartialEq + Send + Sync + 'static,
{
    fn from(p: [S; 3]) -> Self {
        Position::new(p[0], p[1], p[2])
    }
}

impl<'a, S> From<&'a [S; 3]> for Position<S>
where
    S: Clone + Copy + Debug + PartialEq + Send + Sync + 'static,
{
    fn from(p: &'a [S; 3]) -> Self {
        Position::new(p[0], p[1], p[2])
    }
}

impl<'a, S> From<&'a mut [S; 3]> for Position<S>
where
    S: Clone + Copy + Debug + PartialEq + Send + Sync + 'static,
{
    fn from(p: &'a mut [S; 3]) -> Self {
        Position::new(p[0], p[1], p[2])
    }
}

impl<S> Deref for Position<S>
where
    S: Clone + Copy + Debug + PartialEq + Send + Sync + 'static,
{
    type Target = Vector3<S>;

    fn deref(&self) -> &Vector3<S> {
        &self.0
    }
}

impl<S> DerefMut for Position<S>
where
    S: Clone + Copy + Debug + PartialEq + Send + Sync + 'static,
{
    fn deref_mut(&mut self) -> &mut Vector3<S> {
        &mut self.0
    }
}

#[cfg(test)]
mod test {
    use amethyst::core::nalgebra::Vector3;

    use super::Position;

    #[test]
    fn from_vector3() {
        assert_eq!(
            Position::new(1., 2.1, 1.5),
            Vector3::new(1., 2.1, 1.5).into()
        );
        assert_eq!(
            Position::new(1., 2.1, 1.5),
            (&Vector3::new(1., 2.1, 1.5)).into()
        );
        assert_eq!(
            Position::new(1., 2.1, 1.5),
            (&mut Vector3::new(1., 2.1, 1.5)).into()
        );
    }

    #[test]
    fn from_tuple() {
        assert_eq!(Position::new(1., 2.1, 1.5), (1., 2.1, 1.5).into());
        assert_eq!(Position::new(1., 2.1, 1.5), (&(1., 2.1, 1.5)).into());
        assert_eq!(Position::new(1., 2.1, 1.5), (&mut (1., 2.1, 1.5)).into());
    }

    #[test]
    fn from_slice() {
        assert_eq!(Position::new(1., 2.1, 1.5), [1., 2.1, 1.5].into());
        assert_eq!(Position::new(1., 2.1, 1.5), (&[1., 2.1, 1.5]).into());
        assert_eq!(Position::new(1., 2.1, 1.5), (&mut [1., 2.1, 1.5]).into());
    }

    #[test]
    fn deref() {
        assert_eq!(Vector3::new(1., 2., 3.), *Position::new(1., 2., 3.));
    }

    #[test]
    fn deref_mut() {
        let mut position = Position::default();
        *position += Vector3::new(1., 2., 3.);
        assert_eq!(Vector3::new(1., 2., 3.), *position);
    }
}
