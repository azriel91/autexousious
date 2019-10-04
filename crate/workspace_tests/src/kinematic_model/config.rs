#[cfg(test)]
mod tests {
    use amethyst::core::math;

    use kinematic_model::config::Position;

    #[test]
    fn from_vector3() {
        assert_eq!(
            Position::new(1., 2.1, 1.5),
            math::Vector3::new(1., 2.1, 1.5).into()
        );
        assert_eq!(
            Position::new(1., 2.1, 1.5),
            (&math::Vector3::new(1., 2.1, 1.5)).into()
        );
        assert_eq!(
            Position::new(1., 2.1, 1.5),
            (&mut math::Vector3::new(1., 2.1, 1.5)).into()
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
        assert_eq!(math::Vector3::new(1., 2., 3.), *Position::new(1., 2., 3.));
    }

    #[test]
    fn deref_mut() {
        let mut velocity = Position::default();
        *velocity += math::Vector3::new(1., 2., 3.);
        assert_eq!(math::Vector3::new(1., 2., 3.), *velocity);
    }
}
