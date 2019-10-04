#[cfg(test)]
mod test {
    use std::sync::mpsc::{self, Sender};

    use amethyst::{
        ecs::prelude::RunNow,
        shred::{SystemData, World},
        shrev::{EventChannel, ReaderId},
    };
    use application_event::AppEventVariant;
    use application_input::ApplicationEvent;
    use state_registry::StateId;
    use stdio_command_model::StdinCommandBarrier;
    use stdio_spi::VariantAndTokens;

    use stdio_input::{StdinSystem, StdinSystemData};

    fn setup() -> (
        StdinSystem,
        Sender<String>,
        World,
        ReaderId<ApplicationEvent>,
        ReaderId<VariantAndTokens>,
    ) {
        setup_with_barrier(None)
    }

    fn setup_with_barrier(
        with_barrier: Option<bool>,
    ) -> (
        StdinSystem,
        Sender<String>,
        World,
        ReaderId<ApplicationEvent>,
        ReaderId<VariantAndTokens>,
    ) {
        let mut world = World::empty();
        world.insert(StateId::CharacterSelection);
        let barrier_state_id = with_barrier.map(|barrier_matches| {
            if barrier_matches {
                StateId::CharacterSelection
            } else {
                StateId::Loading
            }
        });
        let stdin_command_barrier = StdinCommandBarrier::new(barrier_state_id);
        world.insert(stdin_command_barrier);
        world.insert(EventChannel::<ApplicationEvent>::with_capacity(10));
        world.insert(EventChannel::<VariantAndTokens>::with_capacity(10));

        let (tx, rx) = mpsc::channel();
        let stdin_system = StdinSystem::internal_new(rx, || {});

        let (application_ev_id, variant_and_tokens_id) = {
            let StdinSystemData {
                mut application_ec,
                mut variant_and_tokens_ec,
                ..
            } = StdinSystemData::fetch(&world);
            (
                application_ec.register_reader(),
                variant_and_tokens_ec.register_reader(),
            ) // kcov-ignore
        }; // kcov-ignore

        (
            stdin_system,
            tx,
            world,
            application_ev_id,
            variant_and_tokens_id,
        )
    }

    #[test]
    fn sends_exit_event_when_input_is_exit() {
        let (mut stdin_system, tx, world, mut application_ev_id, _) = setup();

        tx.send("exit".to_string()).unwrap();
        stdin_system.run_now(&world);

        let StdinSystemData { application_ec, .. } = StdinSystemData::fetch(&world);

        expect_event(
            &application_ec,
            &mut application_ev_id,
            Some(&ApplicationEvent::Exit),
        );
    } // kcov-ignore

    #[test]
    fn does_not_send_exit_event_when_input_is_not_exit() {
        let (mut stdin_system, tx, world, mut application_ev_id, _) = setup();

        tx.send("abc".to_string()).unwrap();
        stdin_system.run_now(&world);

        let StdinSystemData { application_ec, .. } = StdinSystemData::fetch(&world);
        expect_event(&application_ec, &mut application_ev_id, None);
    } // kcov-ignore

    #[test]
    fn does_nothing_when_input_is_empty() {
        let (mut stdin_system, _tx, world, mut application_ev_id, _) = setup();

        // we don't call tx.send(..)
        stdin_system.run_now(&world);

        let StdinSystemData { application_ec, .. } = StdinSystemData::fetch(&world);
        expect_event(&application_ec, &mut application_ev_id, None);
    } // kcov-ignore

    #[test]
    fn does_not_panic_when_application_channel_is_disconnected() {
        let (mut stdin_system, tx, world, mut application_ev_id, _) = setup();

        drop(tx); // ensure channel is disconnected
        stdin_system.run_now(&world);

        let StdinSystemData { application_ec, .. } = StdinSystemData::fetch(&world);
        expect_event(&application_ec, &mut application_ev_id, None);
    } // kcov-ignore

    #[test]
    fn sends_vat_event_when_input_is_app_event() {
        let (mut stdin_system, tx, world, _, mut vat_ev_id) = setup();

        tx.send("character_selection confirm".to_string()).unwrap();
        stdin_system.run_now(&world);

        let StdinSystemData {
            variant_and_tokens_ec,
            ..
        } = StdinSystemData::fetch(&world);

        expect_vat_event(
            &variant_and_tokens_ec,
            &mut vat_ev_id,
            Some(&(
                AppEventVariant::CharacterSelection,
                vec!["character_selection".to_string(), "confirm".to_string()],
            )),
        ); // kcov-ignore
    }

    #[test]
    fn does_not_send_exit_event_when_barrier_does_not_match() {
        let (mut stdin_system, tx, world, mut application_ev_id, _) =
            setup_with_barrier(Some(false));

        tx.send("exit".to_string()).unwrap();
        stdin_system.run_now(&world);

        let StdinSystemData { application_ec, .. } = StdinSystemData::fetch(&world);

        expect_event(&application_ec, &mut application_ev_id, None);
    }

    #[test]
    fn sends_exit_event_when_barrier_matches() {
        let (mut stdin_system, tx, world, mut application_ev_id, _) =
            setup_with_barrier(Some(true));

        tx.send("exit".to_string()).unwrap();
        stdin_system.run_now(&world);

        let StdinSystemData { application_ec, .. } = StdinSystemData::fetch(&world);

        expect_event(
            &application_ec,
            &mut application_ev_id,
            Some(&ApplicationEvent::Exit),
        );
    }

    #[test]
    fn does_not_send_vat_event_when_barrier_does_not_match() {
        let (mut stdin_system, tx, world, _, mut vat_ev_id) = setup_with_barrier(Some(false));

        tx.send("character_selection confirm".to_string()).unwrap();
        stdin_system.run_now(&world);

        let StdinSystemData {
            variant_and_tokens_ec,
            ..
        } = StdinSystemData::fetch(&world);

        expect_vat_event(&variant_and_tokens_ec, &mut vat_ev_id, None); // kcov-ignore
    }

    #[test]
    fn sends_vat_event_when_barrier_matches() {
        let (mut stdin_system, tx, world, _, mut vat_ev_id) = setup_with_barrier(Some(true));

        tx.send("character_selection confirm".to_string()).unwrap();
        stdin_system.run_now(&world);

        let StdinSystemData {
            variant_and_tokens_ec,
            ..
        } = StdinSystemData::fetch(&world);

        expect_vat_event(
            &variant_and_tokens_ec,
            &mut vat_ev_id,
            Some(&(
                AppEventVariant::CharacterSelection,
                vec!["character_selection".to_string(), "confirm".to_string()],
            )),
        ); // kcov-ignore
    }

    fn expect_event(
        application_ec: &EventChannel<ApplicationEvent>,
        mut application_ev_id: &mut ReaderId<ApplicationEvent>,
        expected_event: Option<&ApplicationEvent>,
    ) {
        let mut event_it = application_ec.read(&mut application_ev_id);
        assert_eq!(expected_event, event_it.next());
    }

    fn expect_vat_event(
        variant_and_tokens_ec: &EventChannel<VariantAndTokens>,
        mut vat_ev_id: &mut ReaderId<VariantAndTokens>,
        expected_event: Option<&VariantAndTokens>,
    ) {
        let mut event_it = variant_and_tokens_ec.read(&mut vat_ev_id);
        assert_eq!(expected_event, event_it.next());
    }
}
