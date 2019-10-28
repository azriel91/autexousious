#[cfg(test)]
mod tests {
    use amethyst::{
        core::{math::Vector3, transform::Transform},
        ecs::{Entity, SystemData, World, WorldExt},
        shrev::EventChannel,
        window::ScreenDimensions,
        Error,
    };
    use amethyst_test::{AmethystApplication, HIDPI, SCREEN_HEIGHT, SCREEN_WIDTH};
    use camera_model::play::{
        CAMERA_ZOOM_DEPTH_DEFAULT, CAMERA_ZOOM_HEIGHT_DEFAULT, CAMERA_ZOOM_WIDTH_DEFAULT,
    };
    use camera_play::CameraCreator;
    use state_registry::{StateId, StateIdUpdateEvent};

    use state_play::{StateCameraResetSystem, StateCameraResetSystemData};

    #[test]
    fn does_not_reset_transform_when_no_event_sent() -> Result<(), Error> {
        run_test(
            SetupParams {
                events: vec![],
                transform: Transform::from(Vector3::new(1., 2., 3.)),
            },
            ExpectedParams {
                transform: Transform::from(Vector3::new(1., 2., 3.)),
            },
        )
    }

    #[test]
    fn resets_transform_when_event_sent() -> Result<(), Error> {
        run_test(
            SetupParams {
                events: vec![StateIdUpdateEvent::new(StateId::CharacterSelection, None)],
                transform: Transform::from(Vector3::new(1., 2., 3.)),
            },
            ExpectedParams {
                transform: Transform::from(Vector3::new(
                    CAMERA_ZOOM_WIDTH_DEFAULT / 2.,
                    CAMERA_ZOOM_HEIGHT_DEFAULT / 2.,
                    CAMERA_ZOOM_DEPTH_DEFAULT / 2.,
                )),
            },
        )
    }

    fn run_test(
        SetupParams {
            events,
            transform: transform_setup,
        }: SetupParams,
        ExpectedParams {
            transform: transform_expected,
        }: ExpectedParams,
    ) -> Result<(), Error> {
        AmethystApplication::blank()
            .with_system(StateCameraResetSystem::new(), "", &[])
            .with_setup(StateCameraResetSystemData::setup)
            .with_resource(ScreenDimensions::new(SCREEN_WIDTH, SCREEN_HEIGHT, HIDPI))
            .with_effect(move |world| create_camera_with_transform(world, transform_setup))
            .with_effect(move |world| send_events(world, events))
            .with_assertion(move |world| {
                assert_camera_transform(world, transform_expected);
            })
            .run()
    }

    fn send_events(world: &mut World, mut events: Vec<StateIdUpdateEvent>) {
        let mut state_id_update_ec = world.write_resource::<EventChannel<StateIdUpdateEvent>>();
        state_id_update_ec.iter_write(events.drain(..));
    }

    fn create_camera_with_transform(world: &mut World, transform: Transform) {
        let entity = CameraCreator::create_in_world(world);

        {
            // Overwrite `Transform`.
            let mut transforms = world.write_storage::<Transform>();
            transforms
                .insert(entity, transform)
                .expect("Failed to insert `Transform` component.");
        }

        world.insert(entity);
    }

    fn assert_camera_transform(world: &mut World, transform_expected: Transform) {
        let entity = *world.read_resource::<Entity>();
        let transforms = world.read_storage::<Transform>();

        let transform_actual = transforms
            .get(entity)
            .cloned()
            .expect("Expected entity to have `Transform` component.");

        assert_eq!(transform_expected, transform_actual);
    }

    struct SetupParams {
        events: Vec<StateIdUpdateEvent>,
        transform: Transform,
    }

    struct ExpectedParams {
        transform: Transform,
    }
}
