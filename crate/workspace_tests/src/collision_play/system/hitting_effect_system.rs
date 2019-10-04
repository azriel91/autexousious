#[cfg(test)]
mod tests {
    use amethyst::{
        ecs::{Builder, Entity, WorldExt},
        shrev::EventChannel,
        Error,
    };
    use amethyst_test::AmethystApplication;
    use collision_model::{
        config::{Hit, Interaction, InteractionKind},
        loaded::HittingTransition,
        play::HitEvent,
    };
    use sequence_model::loaded::SequenceId;
    use shape_model::Volume;

    use collision_play::HittingEffectSystem;

    #[test]
    fn sets_next_sequence_id_to_hitting_when_hitting_while_hover() -> Result<(), Error> {
        run_test(
            Some(SequenceId::new(0)),
            SetupVariant::WithHittingTransition,
            true,
            Some(SequenceId::new(2)),
        )
    }

    #[test]
    fn sets_next_sequence_id_to_hitting_when_hitting_while_hit() -> Result<(), Error> {
        run_test(
            Some(SequenceId::new(1)),
            SetupVariant::WithHittingTransition,
            true,
            Some(SequenceId::new(2)),
        )
    }

    #[test]
    fn does_nothing_when_hitting_while_hitting() -> Result<(), Error> {
        run_test(
            Some(SequenceId::new(2)),
            SetupVariant::WithHittingTransition,
            true,
            Some(SequenceId::new(2)),
        )
    }

    #[test]
    fn does_nothing_when_no_hit_event() -> Result<(), Error> {
        run_test(
            Some(SequenceId::new(0)),
            SetupVariant::WithHittingTransition,
            false,
            Some(SequenceId::new(0)),
        )
    }

    #[test]
    fn does_nothing_when_not_energy() -> Result<(), Error> {
        run_test(None, SetupVariant::WithoutHittingTransition, true, None)
    }

    fn run_test(
        sequence_id_setup: Option<SequenceId>,
        setup_variant: SetupVariant,
        send_event: bool,
        energy_sequence_name_expected: Option<SequenceId>,
    ) -> Result<(), Error> {
        AmethystApplication::blank()
            .with_system(HittingEffectSystem::new(), "", &[])
            .with_effect(move |world| {
                let entity_from = {
                    let mut entity_builder = world.create_entity();

                    if let Some(sequence_id_setup) = sequence_id_setup {
                        entity_builder = entity_builder.with(sequence_id_setup);
                    }

                    match setup_variant {
                        SetupVariant::WithHittingTransition => {
                            let hitting_transition = HittingTransition::new(SequenceId::new(2));
                            entity_builder = entity_builder.with(hitting_transition);
                        }
                        SetupVariant::WithoutHittingTransition => {}
                    }

                    entity_builder.build()
                };
                let entity_to = world.create_entity().build();
                world.insert(entity_from);

                if send_event {
                    let event = HitEvent::new(entity_from, entity_to, interaction(), body());
                    let mut ec = world.write_resource::<EventChannel<HitEvent>>();
                    ec.single_write(event);
                }
            })
            .with_assertion(move |world| {
                let entity_from = *world.read_resource::<Entity>();
                let sequence_ids = world.read_storage::<SequenceId>();
                let energy_sequence_name_actual = sequence_ids.get(entity_from).copied();
                assert_eq!(energy_sequence_name_expected, energy_sequence_name_actual);
            })
            .run()
    }

    fn interaction() -> Interaction {
        Interaction::new(InteractionKind::Hit(Hit::default()), vec![], true)
    }

    fn body() -> Volume {
        Volume::Box {
            x: 0,
            y: 0,
            z: 0,
            w: 1,
            h: 1,
            d: 1,
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    enum SetupVariant {
        WithHittingTransition,
        WithoutHittingTransition,
    }
}
