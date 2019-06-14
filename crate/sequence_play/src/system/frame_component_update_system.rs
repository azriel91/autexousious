use std::{marker::PhantomData, ops::Deref};

use amethyst::{
    assets::{Asset, AssetStorage, Handle},
    ecs::{Entity, Read, ReadStorage, System, SystemData, WriteStorage},
    shred::Resources,
    shrev::{EventChannel, ReaderId},
};
use derivative::Derivative;
use derive_new::new;
use sequence_model::play::SequenceUpdateEvent;
use sequence_model_spi::loaded::{ComponentFrames, ComponentSequenceExt};
use shred_derive::SystemData;
use typename_derive::TypeName;

/// Updates the frame component value based on the current component sequence handle.
#[derive(Debug, Default, TypeName, new)]
pub struct FrameComponentUpdateSystem<CS>
where
    CS: Asset
        + ComponentSequenceExt
        + Deref<Target = ComponentFrames<<CS as ComponentSequenceExt>::Component>>,
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
        + ComponentSequenceExt
        + Deref<Target = ComponentFrames<<CS as ComponentSequenceExt>::Component>>,
{
    /// Event channel for `SequenceUpdateEvent`s.
    #[derivative(Debug = "ignore")]
    pub sequence_update_ec: Read<'s, EventChannel<SequenceUpdateEvent>>,
    /// `Handle<CS>` component storage.
    #[derivative(Debug = "ignore")]
    pub component_sequence_handles: ReadStorage<'s, Handle<CS>>,
    /// `CS` assets.
    #[derivative(Debug = "ignore")]
    pub component_sequence_assets: Read<'s, AssetStorage<CS>>,
    /// Frame `Component` storages.
    #[derivative(Debug = "ignore")]
    pub components: WriteStorage<'s, <CS as ComponentSequenceExt>::Component>,
}

impl<CS> FrameComponentUpdateSystem<CS>
where
    CS: Asset
        + ComponentSequenceExt
        + Deref<Target = ComponentFrames<<CS as ComponentSequenceExt>::Component>>,
{
    fn update_frame_components(
        components: &mut WriteStorage<<CS as ComponentSequenceExt>::Component>,
        component_sequence: &CS,
        entity: Entity,
        frame_index: usize,
    ) {
        let component = CS::component_owned(&component_sequence[frame_index]);
        components
            .insert(entity, component)
            .expect("Failed to insert component.");
    }
}

impl<'s, CS> System<'s> for FrameComponentUpdateSystem<CS>
where
    CS: Asset
        + ComponentSequenceExt
        + Deref<Target = ComponentFrames<<CS as ComponentSequenceExt>::Component>>,
{
    type SystemData = FrameComponentUpdateSystemData<'s, CS>;

    fn run(
        &mut self,
        FrameComponentUpdateSystemData {
            sequence_update_ec,
            component_sequence_handles,
            component_sequence_assets,
            mut components,
        }: Self::SystemData,
    ) {
        sequence_update_ec
            .read(
                self.reader_id
                    .as_mut()
                    .expect("Expected reader ID to exist for FrameComponentUpdateSystem."),
            )
            .for_each(|ev| {
                let entity = ev.entity();
                let frame_index = ev.frame_index();

                let component_sequence_handle = component_sequence_handles
                    .get(entity)
                    .expect("Expected entity to have a `Handle<CS>` component.");
                let component_sequence = component_sequence_assets
                    .get(component_sequence_handle)
                    .expect("Expected component_sequence to be loaded.");

                Self::update_frame_components(
                    &mut components,
                    component_sequence,
                    entity,
                    frame_index,
                );
            });
    }

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        self.reader_id = Some(
            res.fetch_mut::<EventChannel<SequenceUpdateEvent>>()
                .register_reader(),
        );
    }
}

#[cfg(test)]
mod tests {
    use amethyst::{
        ecs::{Entities, Join, ReadStorage, World, WriteStorage},
        shrev::EventChannel,
        Error,
    };
    use application_test_support::{AutexousiousApplication, SequenceQueries};
    use assets_test::ASSETS_CHAR_BAT_SLUG;
    use character_model::config::CharacterSequenceId;
    use sequence_model::{
        config::Wait,
        loaded::{WaitSequence, WaitSequenceHandle},
        play::{FrameIndexClock, FrameWaitClock, SequenceUpdateEvent},
    };

    use super::FrameComponentUpdateSystem;

    #[test]
    fn updates_all_frame_components_on_sequence_begin_event() -> Result<(), Error> {
        AutexousiousApplication::game_base()
            .with_system(FrameComponentUpdateSystem::<WaitSequence>::new(), "", &[])
            .with_setup(|world| {
                let component_sequence_handle = SequenceQueries::wait_sequence_handle(
                    world,
                    &ASSETS_CHAR_BAT_SLUG.clone(),
                    CharacterSequenceId::StandAttack0,
                );
                initial_values(
                    world,
                    0, // first frame in sequence
                    5,
                    0,
                    5,
                    component_sequence_handle,
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
    fn updates_all_frame_components_on_frame_begin_event() -> Result<(), Error> {
        AutexousiousApplication::game_base()
            .with_system(FrameComponentUpdateSystem::<WaitSequence>::new(), "", &[])
            .with_setup(|world| {
                let component_sequence_handle = SequenceQueries::wait_sequence_handle(
                    world,
                    &ASSETS_CHAR_BAT_SLUG.clone(),
                    CharacterSequenceId::StandAttack0,
                );
                initial_values(
                    world,
                    2, // third frame in the sequence
                    5,
                    0,
                    5,
                    component_sequence_handle,
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
        frame_index_clock_value: usize,
        frame_index_clock_limit: usize,
        frame_wait_clock_value: usize,
        frame_wait_clock_limit: usize,
        component_sequence_handle_initial: WaitSequenceHandle,
    ) {
        let (
            _entities,
            mut frame_index_clocks,
            mut frame_wait_clocks,
            mut component_sequence_handles,
            ..
        ) = world.system_data::<TestSystemData>();

        (
            &mut frame_index_clocks,
            &mut frame_wait_clocks,
            &mut component_sequence_handles,
        )
            .join()
            .for_each(
                |(frame_index_clock, frame_wait_clock, component_sequence_handle)| {
                    (*frame_index_clock).value = frame_index_clock_value;
                    (*frame_index_clock).limit = frame_index_clock_limit;

                    (*frame_wait_clock).value = frame_wait_clock_value;
                    (*frame_wait_clock).limit = frame_wait_clock_limit;

                    *component_sequence_handle = component_sequence_handle_initial.clone();
                },
            );
    }

    fn expect_component_values(world: &mut World, expected_wait: Wait) {
        let (waits, sequence_statuses) =
            world.system_data::<(WriteStorage<'_, Wait>, ReadStorage<'_, CharacterSequenceId>)>();

        (&waits, &sequence_statuses)
            .join()
            .for_each(|(wait, _sequence_status)| {
                assert_eq!(&expected_wait, wait);
            });
    } // kcov-ignore

    fn send_events(world: &mut World, events: Vec<SequenceUpdateEvent>) {
        let mut ec = world.write_resource::<EventChannel<SequenceUpdateEvent>>();
        ec.iter_write(events.into_iter())
    }

    fn sequence_begin_events(world: &mut World) -> Vec<SequenceUpdateEvent> {
        let (
            entities,
            frame_index_clocks,
            frame_wait_clocks,
            component_sequence_handles,
            _components,
        ) = world.system_data::<TestSystemData>();

        (
            &entities,
            &frame_index_clocks,
            &frame_wait_clocks,
            &component_sequence_handles,
        )
            .join()
            .map(|(entity, _, _, _)| SequenceUpdateEvent::SequenceBegin { entity })
            .collect::<Vec<_>>()
    }

    fn frame_begin_events(world: &mut World) -> Vec<SequenceUpdateEvent> {
        let (
            entities,
            frame_index_clocks,
            frame_wait_clocks,
            component_sequence_handles,
            _components,
        ) = world.system_data::<TestSystemData>();

        (
            &entities,
            &frame_index_clocks,
            &frame_wait_clocks,
            &component_sequence_handles,
        )
            .join()
            .map(|(entity, frame_index_clock, _, _)| {
                let frame_index = (*frame_index_clock).value;
                SequenceUpdateEvent::FrameBegin {
                    entity,
                    frame_index,
                }
            })
            .collect::<Vec<_>>()
    }

    type TestSystemData<'s> = (
        Entities<'s>,
        WriteStorage<'s, FrameIndexClock>,
        WriteStorage<'s, FrameWaitClock>,
        WriteStorage<'s, WaitSequenceHandle>,
        WriteStorage<'s, Wait>,
    );
}
