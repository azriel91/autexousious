use amethyst::{
    ecs::{Read, ReadStorage, System, World, WriteStorage},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use collision_model::{loaded::HitTransition, play::HitEvent};
use derivative::Derivative;
use derive_new::new;
use sequence_model::loaded::SequenceId;
use typename_derive::TypeName;

/// Determines the next sequence for `Energy`s when they are hit.
#[derive(Debug, Default, TypeName, new)]
pub struct EnergyHitEffectSystem {
    /// Reader ID for the `HitEvent` event channel.
    #[new(default)]
    hit_event_rid: Option<ReaderId<HitEvent>>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct EnergyHitEffectSystemData<'s> {
    /// `HitEvent` channel.
    #[derivative(Debug = "ignore")]
    pub hit_ec: Read<'s, EventChannel<HitEvent>>,
    /// `HitTransition` components.
    #[derivative(Debug = "ignore")]
    pub hit_transitions: ReadStorage<'s, HitTransition>,
    /// `SequenceId` components.
    #[derivative(Debug = "ignore")]
    pub sequence_ids: WriteStorage<'s, SequenceId>,
}

impl<'s> System<'s> for EnergyHitEffectSystem {
    type SystemData = EnergyHitEffectSystemData<'s>;

    fn run(
        &mut self,
        EnergyHitEffectSystemData {
            hit_ec,
            hit_transitions,
            mut sequence_ids,
        }: Self::SystemData,
    ) {
        hit_ec
            .read(
                self.hit_event_rid
                    .as_mut()
                    .expect("Expected `hit_event_rid` to exist for `EnergyHitEffectSystem`."),
            )
            .for_each(|ev| {
                let hit_transition = hit_transitions.get(ev.to).copied();

                if let Some(HitTransition(sequence_id)) = hit_transition {
                    sequence_ids
                        .insert(ev.to, sequence_id)
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
        loaded::HitTransition,
        play::HitEvent,
    };
    use sequence_model::loaded::SequenceId;
    use shape_model::Volume;

    use super::EnergyHitEffectSystem;

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
    fn does_nothing_when_hit_while_hitting() -> Result<(), Error> {
        run_test(
            Some(SequenceId::new(2)),
            SetupVariant::WithHitTransition,
            true,
            Some(SequenceId::new(2)),
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
            .with_system(EnergyHitEffectSystem::new(), "", &[])
            .with_effect(move |world| {
                let entity_from = {
                    let mut entity_builder = world.create_entity();

                    if let Some(sequence_id_setup) = sequence_id_setup {
                        entity_builder = entity_builder.with(sequence_id_setup);
                    }

                    entity_builder.build()
                };
                let entity_to = world.create_entity().build();
                match setup_variant {
                    SetupVariant::WithHitTransition => {
                        let hit_transition = HitTransition::new(SequenceId::new(1));
                        let mut hit_transitions = world.write_storage::<HitTransition>();
                        hit_transitions
                            .insert(entity_to, hit_transition)
                            .expect("Failed to insert `HitTransition` component.");
                    }
                    SetupVariant::WithoutHitTransition => {}
                }
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
