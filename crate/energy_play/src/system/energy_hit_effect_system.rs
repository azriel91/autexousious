use amethyst::{
    ecs::{Read, System, World, WriteStorage},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use collision_model::play::HitEvent;
use derivative::Derivative;
use derive_new::new;
use energy_model::config::EnergySequenceId;
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
    /// `EnergySequenceId` components.
    #[derivative(Debug = "ignore")]
    pub energy_sequence_ids: WriteStorage<'s, EnergySequenceId>,
}

impl<'s> System<'s> for EnergyHitEffectSystem {
    type SystemData = EnergyHitEffectSystemData<'s>;

    fn run(
        &mut self,
        EnergyHitEffectSystemData {
            hit_ec,
            mut energy_sequence_ids,
        }: Self::SystemData,
    ) {
        hit_ec
            .read(
                self.hit_event_rid
                    .as_mut()
                    .expect("Expected `hit_event_rid` to exist for `EnergyHitEffectSystem`."),
            )
            .for_each(|ev| {
                let energy_sequence_id = energy_sequence_ids.get(ev.to).copied();

                if let Some(EnergySequenceId::Hover) = energy_sequence_id {
                    energy_sequence_ids
                        .insert(ev.to, EnergySequenceId::Hit)
                        .expect("Failed to insert `EnergySequenceId` component.");
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
        ecs::{Builder, Entity},
        shrev::EventChannel,
        Error,
    };
    use amethyst_test::AmethystApplication;
    use collision_model::{
        config::{Hit, Interaction, InteractionKind},
        play::HitEvent,
    };
    use energy_model::config::EnergySequenceId;
    use shape_model::Volume;

    use super::EnergyHitEffectSystem;

    #[test]
    fn sets_next_sequence_id_to_hit_when_hit_while_hover() -> Result<(), Error> {
        run_test(
            SetupVariant::WithSequenceId(EnergySequenceId::Hover),
            true,
            Some(EnergySequenceId::Hit),
        )
    }

    #[test]
    fn does_nothing_when_hit_while_hit() -> Result<(), Error> {
        run_test(
            SetupVariant::WithSequenceId(EnergySequenceId::Hit),
            true,
            Some(EnergySequenceId::Hit),
        )
    }

    #[test]
    fn does_nothing_when_hit_while_hitting() -> Result<(), Error> {
        run_test(
            SetupVariant::WithSequenceId(EnergySequenceId::Hitting),
            true,
            Some(EnergySequenceId::Hitting),
        )
    }

    #[test]
    fn does_nothing_when_no_hit_event() -> Result<(), Error> {
        run_test(
            SetupVariant::WithSequenceId(EnergySequenceId::Hover),
            false,
            Some(EnergySequenceId::Hover),
        )
    }

    #[test]
    fn does_nothing_when_not_energy() -> Result<(), Error> {
        run_test(SetupVariant::WithoutSequenceId, true, None)
    }

    fn run_test(
        setup_variant: SetupVariant,
        send_event: bool,
        energy_sequence_id_expected: Option<EnergySequenceId>,
    ) -> Result<(), Error> {
        AmethystApplication::blank()
            .with_system(EnergyHitEffectSystem::new(), "", &[])
            .with_effect(move |world| {
                let entity_from = world.create_entity().build();
                let entity_to = world.create_entity().build();
                match setup_variant {
                    SetupVariant::WithSequenceId(sequence_id) => {
                        let mut energy_sequence_ids = world.write_storage::<EnergySequenceId>();
                        energy_sequence_ids
                            .insert(entity_to, sequence_id)
                            .expect("Failed to insert `EnergySequenceId` component.");
                    }
                    SetupVariant::WithoutSequenceId => {}
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
                let energy_sequence_ids = world.read_storage::<EnergySequenceId>();
                let energy_sequence_id_actual = energy_sequence_ids.get(entity_to).copied();
                assert_eq!(energy_sequence_id_expected, energy_sequence_id_actual);
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
        WithSequenceId(EnergySequenceId),
        WithoutSequenceId,
    }
}
