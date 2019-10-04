#[cfg(test)]
mod test {
    use std::mem::discriminant;

    use amethyst::{
        ecs::{World, WorldExt},
        shrev::EventChannel,
        StateData, Trans,
    };
    use application_input::ApplicationEvent;

    use application_robot::{ApplicationEventIntercept, Intercept};

    fn setup() -> (ApplicationEventIntercept, World) {
        let mut world = World::new();
        world.insert(EventChannel::<ApplicationEvent>::with_capacity(10));

        (ApplicationEventIntercept::new(), world)
    }

    #[test]
    fn on_start_begin_initializes_application_event_channel_reader() {
        let (mut intercept, mut world) = setup();

        assert!(intercept.application_event_reader.is_none());

        <dyn Intercept<(), ()>>::on_start_begin(
            &mut intercept,
            &mut StateData::new(&mut world, &mut ()),
        );

        assert!(intercept.application_event_reader.is_some());
        let app_event_channel = world.read_resource::<EventChannel<ApplicationEvent>>();
        let mut reader_id = &mut intercept.application_event_reader.as_mut().unwrap();
        assert_eq!(None, app_event_channel.read(&mut reader_id).next());
    }

    #[test]
    fn fixed_update_begin_returns_trans_quit_on_application_event() {
        let (mut intercept, mut world) = setup();

        // register reader
        <dyn Intercept<(), ()>>::on_start_begin(
            &mut intercept,
            &mut StateData::new(&mut world, &mut ()),
        );

        {
            let mut app_event_channel = world.write_resource::<EventChannel<ApplicationEvent>>();
            app_event_channel.single_write(ApplicationEvent::Exit);
        } // kcov-ignore

        assert_eq!(
            discriminant(&Trans::Quit as &Trans<(), ()>),
            discriminant(
                &intercept
                    .fixed_update_begin(&mut StateData::new(&mut world, &mut ()))
                    .unwrap()
            )
        );
    }

    #[test]
    fn update_begin_returns_trans_quit_on_application_event() {
        let (mut intercept, mut world) = setup();

        // register reader
        <dyn Intercept<(), ()>>::on_start_begin(
            &mut intercept,
            &mut StateData::new(&mut world, &mut ()),
        );

        {
            let mut app_event_channel = world.write_resource::<EventChannel<ApplicationEvent>>();
            app_event_channel.single_write(ApplicationEvent::Exit);
        } // kcov-ignore

        assert_eq!(
            discriminant(&Trans::Quit as &Trans<(), ()>),
            discriminant(
                &intercept
                    .update_begin(&mut StateData::new(&mut world, &mut ()))
                    .unwrap()
            )
        );
    }

    #[test]
    fn intercept_is_transitive() {
        assert!(<dyn Intercept<(), ()>>::is_transitive(
            &ApplicationEventIntercept::new()
        ));
    }
}
