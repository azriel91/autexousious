use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use amethyst::{
    core::math::Vector3,
    ecs::{storage::DenseVecStorage, Component},
};
use serde::{Deserialize, Serialize};
use specs_derive::Component;

/// Velocity of the entity in game.
#[derive(Clone, Component, Copy, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub struct Velocity<S>(pub Vector3<S>)
where
    S: Clone + Copy + Debug + PartialEq + Send + Sync + 'static;

impl<S> Velocity<S>
where
    S: Clone + Copy + Debug + PartialEq + Send + Sync + 'static,
{
    /// Returns a new `Velocity` vector.
    ///
    /// # Parameters
    ///
    /// * `x`: X axis velocity component.
    /// * `y`: Y axis velocity component.
    /// * `z`: Z axis velocity component.
    pub fn new(x: S, y: S, z: S) -> Self {
        Velocity(Vector3::new(x, y, z))
    }
}

impl<S> Default for Velocity<S>
where
    S: Clone + Copy + Debug + Default + PartialEq + Send + Sync + 'static,
{
    fn default() -> Self {
        Velocity(Vector3::new(S::default(), S::default(), S::default()))
    }
}

impl<S> From<Vector3<S>> for Velocity<S>
where
    S: Clone + Copy + Debug + PartialEq + Send + Sync + 'static,
{
    fn from(v: Vector3<S>) -> Self {
        Velocity(v)
    }
}

impl<'a, S> From<&'a Vector3<S>> for Velocity<S>
where
    S: Clone + Copy + Debug + PartialEq + Send + Sync + 'static,
{
    fn from(v: &'a Vector3<S>) -> Self {
        Velocity(*v)
    }
}

impl<'a, S> From<&'a mut Vector3<S>> for Velocity<S>
where
    S: Clone + Copy + Debug + PartialEq + Send + Sync + 'static,
{
    fn from(v: &'a mut Vector3<S>) -> Self {
        Velocity(*v)
    }
}

impl<S> From<(S, S, S)> for Velocity<S>
where
    S: Clone + Copy + Debug + PartialEq + Send + Sync + 'static,
{
    fn from(v: (S, S, S)) -> Self {
        Velocity::new(v.0, v.1, v.2)
    }
}

impl<'a, S> From<&'a (S, S, S)> for Velocity<S>
where
    S: Clone + Copy + Debug + PartialEq + Send + Sync + 'static,
{
    fn from(v: &'a (S, S, S)) -> Self {
        Velocity::new(v.0, v.1, v.2)
    }
}

impl<'a, S> From<&'a mut (S, S, S)> for Velocity<S>
where
    S: Clone + Copy + Debug + PartialEq + Send + Sync + 'static,
{
    fn from(v: &'a mut (S, S, S)) -> Self {
        Velocity::new(v.0, v.1, v.2)
    }
}

impl<S> From<[S; 3]> for Velocity<S>
where
    S: Clone + Copy + Debug + PartialEq + Send + Sync + 'static,
{
    fn from(v: [S; 3]) -> Self {
        Velocity::new(v[0], v[1], v[2])
    }
}

impl<'a, S> From<&'a [S; 3]> for Velocity<S>
where
    S: Clone + Copy + Debug + PartialEq + Send + Sync + 'static,
{
    fn from(v: &'a [S; 3]) -> Self {
        Velocity::new(v[0], v[1], v[2])
    }
}

impl<'a, S> From<&'a mut [S; 3]> for Velocity<S>
where
    S: Clone + Copy + Debug + PartialEq + Send + Sync + 'static,
{
    fn from(v: &'a mut [S; 3]) -> Self {
        Velocity::new(v[0], v[1], v[2])
    }
}

impl<S> Deref for Velocity<S>
where
    S: Clone + Copy + Debug + PartialEq + Send + Sync + 'static,
{
    type Target = Vector3<S>;

    fn deref(&self) -> &Vector3<S> {
        &self.0
    }
}

impl<S> DerefMut for Velocity<S>
where
    S: Clone + Copy + Debug + PartialEq + Send + Sync + 'static,
{
    fn deref_mut(&mut self) -> &mut Vector3<S> {
        &mut self.0
    }
}

#[cfg(test)]
mod test {
    use amethyst::core::math::Vector3;

    use super::Velocity;

    #[test]
    fn from_vector3() {
        assert_eq!(
            Velocity::new(1., 2.1, 1.5),
            Vector3::new(1., 2.1, 1.5).into()
        );
        assert_eq!(
            Velocity::new(1., 2.1, 1.5),
            (&Vector3::new(1., 2.1, 1.5)).into()
        );
        assert_eq!(
            Velocity::new(1., 2.1, 1.5),
            (&mut Vector3::new(1., 2.1, 1.5)).into()
        );
    }

    #[test]
    fn from_tuple() {
        assert_eq!(Velocity::new(1., 2.1, 1.5), (1., 2.1, 1.5).into());
        assert_eq!(Velocity::new(1., 2.1, 1.5), (&(1., 2.1, 1.5)).into());
        assert_eq!(Velocity::new(1., 2.1, 1.5), (&mut (1., 2.1, 1.5)).into());
    }

    #[test]
    fn from_slice() {
        assert_eq!(Velocity::new(1., 2.1, 1.5), [1., 2.1, 1.5].into());
        assert_eq!(Velocity::new(1., 2.1, 1.5), (&[1., 2.1, 1.5]).into());
        assert_eq!(Velocity::new(1., 2.1, 1.5), (&mut [1., 2.1, 1.5]).into());
    }

    #[test]
    fn deref() {
        assert_eq!(Vector3::new(1., 2., 3.), *Velocity::new(1., 2., 3.));
    }

    #[test]
    fn deref_mut() {
        let mut velocity = Velocity::default();
        *velocity += Vector3::new(1., 2., 3.);
        assert_eq!(Vector3::new(1., 2., 3.), *velocity);
    }
}
