#[cfg(test)]
mod tests {
    use amethyst::{
        ecs::{Entity, SystemData, World, WorldExt},
        shrev::EventChannel,
        window::ScreenDimensions,
        Error,
    };
    use amethyst_test::{AmethystApplication, HIDPI, SCREEN_HEIGHT, SCREEN_WIDTH};
    use camera_play::CameraCreator;
    use kinematic_model::config::Position;
    use state_registry::{StateId, StateIdUpdateEvent};

    use state_play::{StateCameraResetSystem, StateCameraResetSystemData};

    #[test]
    fn does_not_reset_position_when_no_event_sent() -> Result<(), Error> {
        run_test(
            SetupParams {
                events: vec![],
                position: Position::new(1., 2., 3.),
            },
            ExpectedParams {
                position: Position::new(1., 2., 3.),
            },
        )
    }

    #[test]
    fn spawns_layer_entities_when_event_sent() -> Result<(), Error> {
        run_test(
            SetupParams {
                events: vec![StateIdUpdateEvent::new(StateId::CharacterSelection, None)],
                position: Position::new(1., 2., 3.),
            },
            ExpectedParams {
                position: Position::new(0., 0., 0.),
            },
        )
    }

    fn run_test(
        SetupParams {
            events,
            position: position_setup,
        }: SetupParams,
        ExpectedParams {
            position: position_expected,
        }: ExpectedParams,
    ) -> Result<(), Error> {
        AmethystApplication::blank()
            .with_system(StateCameraResetSystem::new(), "", &[])
            .with_setup(StateCameraResetSystemData::setup)
            .with_resource(ScreenDimensions::new(SCREEN_WIDTH, SCREEN_HEIGHT, HIDPI))
            .with_effect(move |world| create_camera_with_position(world, position_setup))
            .with_effect(move |world| send_events(world, events))
            .with_assertion(move |world| {
                assert_camera_position(world, position_expected);
            })
            .run()
    }

    fn send_events(world: &mut World, mut events: Vec<StateIdUpdateEvent>) {
        let mut state_id_update_ec = world.write_resource::<EventChannel<StateIdUpdateEvent>>();
        state_id_update_ec.iter_write(events.drain(..));
    }

    fn create_camera_with_position(world: &mut World, position: Position<f32>) {
        let entity = CameraCreator::create_in_world(world);

        {
            // Overwrite `Position<f32>`.
            let mut positions = world.write_storage::<Position<f32>>();
            positions
                .insert(entity, position)
                .expect("Failed to insert `Position<f32>` component.");
        }

        world.insert(entity);
    }

    fn assert_camera_position(world: &mut World, position_expected: Position<f32>) {
        let entity = *world.read_resource::<Entity>();
        let positions = world.read_storage::<Position<f32>>();

        let position_actual = positions
            .get(entity)
            .copied()
            .expect("Expected entity to have `Position<f32>` component.");

        assert_eq!(position_expected, position_actual);
    }

    struct SetupParams {
        events: Vec<StateIdUpdateEvent>,
        position: Position<f32>,
    }

    struct ExpectedParams {
        position: Position<f32>,
    }
}
