use std::{fmt::Debug, marker::PhantomData, ops::Deref};

use amethyst::ecs::WorldExt; use amethyst::{
    assets::{Asset, AssetStorage, Handle},
    ecs::{Entity, Read, ReadStorage, System, World, WriteStorage},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use derivative::Derivative;
use derive_new::new;
use log::error;
use sequence_model::play::SequenceUpdateEvent;
use sequence_model_spi::loaded::{ComponentDataExt, FrameComponentData};
use typename_derive::TypeName;

/// Updates the frame component value based on the current frame component data handle.
#[derive(Debug, Default, TypeName, new)]
pub struct FrameComponentUpdateSystem<CS>
where
    CS: Asset
        + ComponentDataExt
        + Debug
        + Deref<Target = FrameComponentData<<CS as ComponentDataExt>::Component>>,
{
    /// Reader ID for the `SequenceUpdateEvent` event channel.
    #[new(default)]
    reader_id: Option<ReaderId<SequenceUpdateEvent>>,
    /// Marker.
    phantom_data: PhantomData<CS>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct FrameComponentUpdateSystemData<'s, CS>
where
    CS: Asset
        + ComponentDataExt
        + Debug
        + Deref<Target = FrameComponentData<<CS as ComponentDataExt>::Component>>,
{
    /// Event channel for `SequenceUpdateEvent`s.
    #[derivative(Debug = "ignore")]
    pub sequence_update_ec: Read<'s, EventChannel<SequenceUpdateEvent>>,
    /// `Handle<CS>` component storage.
    #[derivative(Debug = "ignore")]
    pub frame_component_data_handles: ReadStorage<'s, Handle<CS>>,
    /// `CS` assets.
    #[derivative(Debug = "ignore")]
    pub frame_component_data_assets: Read<'s, AssetStorage<CS>>,
    /// Frame `Component` storages.
    #[derivative(Debug = "ignore")]
    pub components: WriteStorage<'s, <CS as ComponentDataExt>::Component>,
}

impl<CS> FrameComponentUpdateSystem<CS>
where
    CS: Asset
        + ComponentDataExt
        + Debug
        + Deref<Target = FrameComponentData<<CS as ComponentDataExt>::Component>>,
{
    fn update_frame_components(
        components: &mut WriteStorage<<CS as ComponentDataExt>::Component>,
        frame_component_data: &CS,
        entity: Entity,
        frame_index: usize,
    ) {
        if frame_index < frame_component_data.len() {
            let component = CS::to_owned(&frame_component_data[frame_index]);
            components
                .insert(entity, component)
                .expect("Failed to insert frame component.");
        } else {
            error!(
                "Attempted to access index `{}` for frame component data: `{:?}`",
                frame_index, frame_component_data
            );
        }
    }
}

impl<'s, CS> System<'s> for FrameComponentUpdateSystem<CS>
where
    CS: Asset
        + ComponentDataExt
        + Debug
        + Deref<Target = FrameComponentData<<CS as ComponentDataExt>::Component>>,
{
    type SystemData = FrameComponentUpdateSystemData<'s, CS>;

    fn run(
        &mut self,
        FrameComponentUpdateSystemData {
            sequence_update_ec,
            frame_component_data_handles,
            frame_component_data_assets,
            mut components,
        }: Self::SystemData,
    ) {
        sequence_update_ec
            .read(
                self.reader_id
                    .as_mut()
                    .expect("Expected reader ID to exist for FrameComponentUpdateSystem."),
            )
            .filter(|ev| {
                if let SequenceUpdateEvent::SequenceBegin { .. }
                | SequenceUpdateEvent::FrameBegin { .. } = ev
                {
                    true
                } else {
                    false
                }
            })
            .for_each(|ev| {
                let entity = ev.entity();
                let frame_index = ev.frame_index();

                let frame_component_data_handle = frame_component_data_handles.get(entity);

                // Some entities will have sequence update events, but not this particular sequence
                // component.
                if let Some(frame_component_data_handle) = frame_component_data_handle {
                    let frame_component_data = frame_component_data_assets
                        .get(frame_component_data_handle)
                        .expect("Expected frame_component_data to be loaded.");

                    Self::update_frame_components(
                        &mut components,
                        frame_component_data,
                        entity,
                        frame_index,
                    );
                }
            });
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.reader_id = Some(
            world
                .fetch_mut::<EventChannel<SequenceUpdateEvent>>()
                .register_reader(),
        );
    }
}

#[cfg(test)]
mod tests {
    use amethyst::ecs::WorldExt; use amethyst::{
        ecs::{Builder, Entity, World},
        shrev::EventChannel,
        Error,
    };
    use application_test_support::{AutexousiousApplication, SequenceQueries};
    use assets_test::CHAR_BAT_SLUG;
    use character_model::config::CharacterSequenceId;
    use sequence_model::{
        config::Wait,
        loaded::{WaitSequence, WaitSequenceHandle},
        play::{FrameIndexClock, FrameWaitClock, SequenceUpdateEvent},
    };

    use super::FrameComponentUpdateSystem;

    #[test]
    fn updates_frame_component_on_sequence_begin_event() -> Result<(), Error> {
        AutexousiousApplication::game_base()
            .with_system(FrameComponentUpdateSystem::<WaitSequence>::new(), "", &[])
            .with_setup(|world| {
                let frame_component_data_handle = SequenceQueries::wait_sequence_handle(
                    world,
                    &CHAR_BAT_SLUG.clone(),
                    CharacterSequenceId::StandAttack0,
                );
                initial_values(
                    world,
                    // First frame in sequence.
                    FrameIndexClock::new_with_value(5, 0),
                    FrameWaitClock::new_with_value(5, 0),
                    Some(frame_component_data_handle),
                )
            })
            .with_setup(|world| {
                let events = sequence_begin_events(world);
                send_events(world, events);
            })
            .with_assertion(|world| {
                // See bat/object.toml for values.
                expect_component_values(world, Wait::new(1))
            })
            .run_isolated()
    }

    #[test]
    fn updates_frame_component_on_frame_begin_event() -> Result<(), Error> {
        AutexousiousApplication::game_base()
            .with_system(FrameComponentUpdateSystem::<WaitSequence>::new(), "", &[])
            .with_setup(|world| {
                let frame_component_data_handle = SequenceQueries::wait_sequence_handle(
                    world,
                    &CHAR_BAT_SLUG.clone(),
                    CharacterSequenceId::StandAttack0,
                );
                initial_values(
                    world,
                    // Third frame in the sequence.
                    FrameIndexClock::new_with_value(5, 2),
                    FrameWaitClock::new_with_value(5, 0),
                    Some(frame_component_data_handle),
                )
            })
            .with_setup(|world| {
                let events = frame_begin_events(world);
                send_events(world, events);
            })
            .with_assertion(|world| {
                // See bat/object.toml for values.
                expect_component_values(world, Wait::new(2))
            })
            .run_isolated()
    }

    #[test]
    fn does_not_panic_when_entity_does_not_have_frame_component_data_handle() -> Result<(), Error> {
        AutexousiousApplication::game_base()
            .with_system(FrameComponentUpdateSystem::<WaitSequence>::new(), "", &[])
            .with_setup(|world| {
                initial_values(
                    world,
                    // Third frame in the sequence.
                    FrameIndexClock::new_with_value(5, 2),
                    FrameWaitClock::new_with_value(5, 0),
                    None,
                )
            })
            .with_setup(|world| {
                let events = frame_begin_events(world);
                send_events(world, events);
            })
            .with_assertion(|world| {
                // See bat/object.toml for values.
                expect_component_values(world, Wait::new(2))
            })
            .run_isolated()
    }

    fn initial_values(
        world: &mut World,
        frame_index_clock: FrameIndexClock,
        frame_wait_clock: FrameWaitClock,
        frame_component_data_handle_initial: Option<WaitSequenceHandle>,
    ) {
        let entity = {
            let mut entity_builder = world
                .create_entity()
                .with(frame_index_clock)
                .with(frame_wait_clock)
                .with(Wait::new(2));

            if let Some(frame_component_data_handle_initial) = frame_component_data_handle_initial {
                entity_builder = entity_builder.with(frame_component_data_handle_initial);
            }

            entity_builder.build()
        };

        world.insert(entity);
    }

    fn expect_component_values(world: &mut World, expected_wait: Wait) {
        let entity = *world.read_resource::<Entity>();
        let waits = world.read_storage::<Wait>();

        let wait = waits
            .get(entity)
            .expect("Expected entity to have `Wait` component.");
        assert_eq!(&expected_wait, wait);
    }

    fn send_events(world: &mut World, events: Vec<SequenceUpdateEvent>) {
        let mut ec = world.write_resource::<EventChannel<SequenceUpdateEvent>>();
        ec.iter_write(events.into_iter())
    }

    fn sequence_begin_events(world: &mut World) -> Vec<SequenceUpdateEvent> {
        let entity = *world.read_resource::<Entity>();
        vec![SequenceUpdateEvent::SequenceBegin { entity }]
    }

    fn frame_begin_events(world: &mut World) -> Vec<SequenceUpdateEvent> {
        let entity = *world.read_resource::<Entity>();
        let frame_index = {
            let frame_index_clocks = world.read_storage::<FrameIndexClock>();
            let frame_index_clock = frame_index_clocks
                .get(entity)
                .expect("Expected entity to have `FrameIndexClock` component.");
            (*frame_index_clock).value
        };

        vec![SequenceUpdateEvent::FrameBegin {
            entity,
            frame_index,
        }]
    }
}
