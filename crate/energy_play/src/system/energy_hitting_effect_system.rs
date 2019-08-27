use amethyst::{
    ecs::{Read, ReadStorage, System, World, WriteStorage},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use collision_model::{loaded::HittingTransition, play::HitEvent};
use derivative::Derivative;
use derive_new::new;
use sequence_model::loaded::SequenceId;
use typename_derive::TypeName;

/// Determines the next sequence for `Energy`s when they hit another object.
#[derive(Debug, Default, TypeName, new)]
pub struct EnergyHittingEffectSystem {
    /// Reader ID for the `HitEvent` event channel.
    #[new(default)]
    hit_event_rid: Option<ReaderId<HitEvent>>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct EnergyHittingEffectSystemData<'s> {
    /// `HitEvent` channel.
    #[derivative(Debug = "ignore")]
    pub hit_ec: Read<'s, EventChannel<HitEvent>>,
    /// `HittingTransition` components.
    #[derivative(Debug = "ignore")]
    pub hitting_transitions: ReadStorage<'s, HittingTransition>,
    /// `SequenceId` components.
    #[derivative(Debug = "ignore")]
    pub sequence_ids: WriteStorage<'s, SequenceId>,
}

impl<'s> System<'s> for EnergyHittingEffectSystem {
    type SystemData = EnergyHittingEffectSystemData<'s>;

    fn run(
        &mut self,
        EnergyHittingEffectSystemData {
            hit_ec,
            hitting_transitions,
            mut sequence_ids,
        }: Self::SystemData,
    ) {
        hit_ec
            .read(
                self.hit_event_rid
                    .as_mut()
                    .expect("Expected `hit_event_rid` to exist for `EnergyHittingEffectSystem`."),
            )
            .for_each(|ev| {
                let hitting_transition = hitting_transitions.get(ev.from).copied();

                if let Some(HittingTransition(sequence_id)) = hitting_transition {
                    sequence_ids
                        .insert(ev.from, sequence_id)
                        .expect("Failed to insert `SequenceId` component.");
                }
            });
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.hit_event_rid = Some(
            world
                .fetch_mut::<EventChannel<HitEvent>>()
                .register_reader(),
        );
    }
}

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

    use super::EnergyHittingEffectSystem;

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
            .with_system(EnergyHittingEffectSystem::new(), "", &[])
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
