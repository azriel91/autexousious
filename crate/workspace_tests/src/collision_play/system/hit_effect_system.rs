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
        loaded::HitTransition,
        play::HitEvent,
    };
    use sequence_model::loaded::SequenceId;
    use shape_model::Volume;

    use collision_play::HitEffectSystem;

    #[test]
    fn sets_next_sequence_id_to_hit_when_hit_while_hover() -> Result<(), Error> {
        run_test(
            Some(SequenceId::new(0)),
            SetupVariant::WithHitTransition,
            true,
            Some(SequenceId::new(1)),
        )
    }

    #[test]
    fn does_nothing_when_hit_while_hit() -> Result<(), Error> {
        run_test(
            Some(SequenceId::new(1)),
            SetupVariant::WithHitTransition,
            true,
            Some(SequenceId::new(1)),
        )
    }

    #[test]
    fn sets_next_sequence_id_to_hit_when_hit_while_hitting() -> Result<(), Error> {
        run_test(
            Some(SequenceId::new(2)),
            SetupVariant::WithHitTransition,
            true,
            Some(SequenceId::new(1)),
        )
    }

    #[test]
    fn does_nothing_when_no_hit_event() -> Result<(), Error> {
        run_test(
            Some(SequenceId::new(0)),
            SetupVariant::WithHitTransition,
            false,
            Some(SequenceId::new(0)),
        )
    }

    #[test]
    fn does_nothing_when_no_hit_transition() -> Result<(), Error> {
        run_test(None, SetupVariant::WithoutHitTransition, true, None)
    }

    fn run_test(
        sequence_id_setup: Option<SequenceId>,
        setup_variant: SetupVariant,
        send_event: bool,
        sequence_id_expected: Option<SequenceId>,
    ) -> Result<(), Error> {
        AmethystApplication::blank()
            .with_system(HitEffectSystem::new(), "", &[])
            .with_effect(move |world| {
                let entity_from = world.create_entity().build();
                let entity_to = {
                    let mut entity_builder = world.create_entity();

                    if let Some(sequence_id_setup) = sequence_id_setup {
                        entity_builder = entity_builder.with(sequence_id_setup);
                    }

                    match setup_variant {
                        SetupVariant::WithHitTransition => {
                            let hit_transition = HitTransition::new(SequenceId::new(1));
                            entity_builder = entity_builder.with(hit_transition);
                        }
                        SetupVariant::WithoutHitTransition => {}
                    }

                    entity_builder.build()
                };
                world.insert(entity_to);

                if send_event {
                    let event = HitEvent::new(entity_from, entity_to, interaction(), body());
                    let mut ec = world.write_resource::<EventChannel<HitEvent>>();
                    ec.single_write(event);
                }
            })
            .with_assertion(move |world| {
                let entity_to = *world.read_resource::<Entity>();
                let sequence_ids = world.read_storage::<SequenceId>();
                let sequence_id_actual = sequence_ids.get(entity_to).copied();
                assert_eq!(sequence_id_expected, sequence_id_actual);
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
        WithHitTransition,
        WithoutHitTransition,
    }
}
