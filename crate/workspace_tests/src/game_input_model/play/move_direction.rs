#[cfg(test)]
mod tests {
    use amethyst::ecs::{Builder, Entity, World, WorldExt};
    use game_input_model::{play::MoveDirection, Axis, AxisMoveEventData};

    macro_rules! test {
        ($test_name:ident, $axis:expr, $value:expr, $expected:expr) => {
            #[test]
            fn $test_name() {
                let entity = entity();
                let move_direction = MoveDirection::from(AxisMoveEventData {
                    entity,
                    axis: $axis,
                    value: $value,
                });

                assert_eq!($expected, move_direction);
            }
        };
    }

    test!(
        move_direction_from_axis_x_positive_is_right,
        Axis::X,
        0.5,
        MoveDirection::Right
    );
    test!(
        move_direction_from_axis_x_negative_is_left,
        Axis::X,
        -0.5,
        MoveDirection::Left
    );
    test!(
        move_direction_from_axis_x_zero_is_none,
        Axis::X,
        0.,
        MoveDirection::None
    );
    test!(
        move_direction_from_axis_z_positive_is_up,
        Axis::Z,
        0.5,
        MoveDirection::Up
    );
    test!(
        move_direction_from_axis_z_negative_is_down,
        Axis::Z,
        -0.5,
        MoveDirection::Down
    );
    test!(
        move_direction_from_axis_z_zero_is_none,
        Axis::Z,
        0.,
        MoveDirection::None
    );

    fn entity() -> Entity {
        World::new().create_entity().build()
    }
}
