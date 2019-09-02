use std::{convert::AsRef, fmt::Debug, marker::PhantomData, ops::Deref};

use amethyst::{
    assets::{Asset, AssetStorage, Handle},
    ecs::{Entity, Read, ReadStorage, System, World, WriteStorage},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use derivative::Derivative;
use derive_new::new;
use sequence_model::{loaded::SequenceId, play::SequenceUpdateEvent};
use sequence_model_spi::loaded::{ComponentDataExt, SequenceComponentData};
use typename_derive::TypeName;

/// Updates the sequence component based on the current sequence ID.
///
/// # Type Parameters
///
/// * `SCDA`: Asset type that the sequence component data is stored in, e.g. `Object`.
/// * `SCD`: Type of sequence component data, e.g. `SequenceEndTransitions`.
#[derive(Debug, Default, TypeName, new)]
pub struct SequenceComponentUpdateSystem<SCDA, SCD>
where
    SCDA: Asset + AsRef<SCD>,
    SCD: ComponentDataExt
        + Debug
        + Deref<Target = SequenceComponentData<<SCD as ComponentDataExt>::Component>>,
{
    /// Reader ID for the `SequenceUpdateEvent` event channel.
    #[new(default)]
    reader_id: Option<ReaderId<SequenceUpdateEvent>>,
    /// Marker.
    phantom_data: PhantomData<(SCDA, SCD)>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct SequenceComponentUpdateSystemData<'s, SCDA, SCD>
where
    SCDA: Asset + AsRef<SCD>,
    SCD: ComponentDataExt
        + Debug
        + Deref<Target = SequenceComponentData<<SCD as ComponentDataExt>::Component>>,
{
    /// Event channel for `SequenceUpdateEvent`s.
    #[derivative(Debug = "ignore")]
    pub sequence_update_ec: Read<'s, EventChannel<SequenceUpdateEvent>>,
    /// `Handle<SCDA>` component storage.
    #[derivative(Debug = "ignore")]
    pub scda_handles: ReadStorage<'s, Handle<SCDA>>,
    /// `SCDA` assets.
    #[derivative(Debug = "ignore")]
    pub scda_assets: Read<'s, AssetStorage<SCDA>>,
    /// Frame `Component` storages.
    #[derivative(Debug = "ignore")]
    pub sequence_components: WriteStorage<'s, <SCD as ComponentDataExt>::Component>,
}

impl<SCDA, SCD> SequenceComponentUpdateSystem<SCDA, SCD>
where
    SCDA: Asset + AsRef<SCD>,
    SCD: ComponentDataExt
        + Debug
        + Deref<Target = SequenceComponentData<<SCD as ComponentDataExt>::Component>>,
{
    fn update_component(
        sequence_components: &mut WriteStorage<<SCD as ComponentDataExt>::Component>,
        scda: &SCDA,
        entity: Entity,
        sequence_id: SequenceId,
    ) {
        let sequence_component_data = AsRef::<SCD>::as_ref(scda);
        let component = sequence_component_data
            .get(*sequence_id)
            .map(SCD::to_owned)
            .unwrap_or_else(|| {
                panic!(
                    "Expected sequence component to exist for sequence ID: `{}`",
                    sequence_id
                )
            });
        sequence_components
            .insert(entity, component)
            .expect("Failed to insert sequence component.");
    }
}

impl<'s, SCDA, SCD> System<'s> for SequenceComponentUpdateSystem<SCDA, SCD>
where
    SCDA: Asset + AsRef<SCD>,
    SCD: ComponentDataExt
        + Debug
        + Deref<Target = SequenceComponentData<<SCD as ComponentDataExt>::Component>>,
{
    type SystemData = SequenceComponentUpdateSystemData<'s, SCDA, SCD>;

    fn run(
        &mut self,
        SequenceComponentUpdateSystemData {
            sequence_update_ec,
            scda_handles,
            scda_assets,
            mut sequence_components,
        }: Self::SystemData,
    ) {
        sequence_update_ec
            .read(
                self.reader_id
                    .as_mut()
                    .expect("Expected reader ID to exist for SequenceComponentUpdateSystem."),
            )
            .filter_map(|ev| {
                if let SequenceUpdateEvent::SequenceBegin {
                    entity,
                    sequence_id,
                } = ev
                {
                    Some((*entity, *sequence_id))
                } else {
                    None
                }
            })
            .for_each(|(entity, sequence_id)| {
                let scda_handle = scda_handles.get(entity);

                // Some entities will have sequence update events, but not this particular sequence
                // component data asset.
                if let Some(scda_handle) = scda_handle {
                    let scda = scda_assets
                        .get(scda_handle)
                        .expect("Expected `SCDA` to be loaded.");

                    Self::update_component(&mut sequence_components, scda, entity, sequence_id);
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
    use amethyst::{
        ecs::{Builder, Entity, World, WorldExt},
        shrev::EventChannel,
        Error,
    };
    use application_test_support::{AutexousiousApplication, ObjectQueries, SequenceQueries};
    use assets_test::CHAR_BAT_SLUG;
    use character_model::loaded::CharacterObjectWrapper;
    use sequence_model::{
        loaded::{SequenceId, WaitSequenceHandle, WaitSequenceHandles},
        play::SequenceUpdateEvent,
    };

    use super::SequenceComponentUpdateSystem;

    const SEQUENCE_ID_PREV: SequenceId = SequenceId(1);
    const SEQUENCE_ID_CURRENT: SequenceId = SequenceId(2);

    #[test]
    fn updates_sequence_component_on_sequence_begin_event() -> Result<(), Error> {
        run_test(sequence_begin_events, true, SEQUENCE_ID_CURRENT)
    }

    #[test]
    fn does_not_update_sequence_component_on_frame_begin_event() -> Result<(), Error> {
        run_test(frame_begin_events, true, SEQUENCE_ID_PREV)
    }

    #[test]
    fn does_not_panic_when_entity_does_not_have_scda_handle() -> Result<(), Error> {
        run_test(sequence_begin_events, false, SEQUENCE_ID_PREV)
    }

    fn run_test(
        sequence_update_events_fn: fn(&mut World) -> Vec<SequenceUpdateEvent>,
        with_scda_handle: bool,
        sequence_id_expected: SequenceId,
    ) -> Result<(), Error> {
        AutexousiousApplication::game_base()
            .with_system(
                SequenceComponentUpdateSystem::<CharacterObjectWrapper, WaitSequenceHandles>::new(),
                "",
                &[],
            )
            .with_effect(move |world| initial_values(world, with_scda_handle))
            .with_effect(move |world| {
                let events = sequence_update_events_fn(world);
                send_events(world, events);
            })
            .with_assertion(move |world| {
                let wait_sequence_handle_expected = SequenceQueries::wait_sequence_handle(
                    world,
                    &CHAR_BAT_SLUG.clone(),
                    sequence_id_expected,
                );
                expect_component_values(world, wait_sequence_handle_expected)
            })
            .run_isolated()
    }

    fn initial_values(world: &mut World, with_scda_handle: bool) {
        let entity = {
            let wait_sequence_handle = SequenceQueries::wait_sequence_handle(
                world,
                &CHAR_BAT_SLUG.clone(),
                SEQUENCE_ID_PREV,
            );
            let scda_handle = if with_scda_handle {
                Some(ObjectQueries::object_wrapper_handle(
                    world,
                    &CHAR_BAT_SLUG.clone(),
                ))
            } else {
                None
            };

            let mut entity_builder = world
                .create_entity()
                .with(SEQUENCE_ID_CURRENT)
                .with(wait_sequence_handle);

            if let Some(scda_handle) = scda_handle {
                entity_builder = entity_builder.with(scda_handle);
            }

            entity_builder.build()
        };

        world.insert(entity);
    }

    fn expect_component_values(
        world: &mut World,
        wait_sequence_handle_expected: WaitSequenceHandle,
    ) {
        let entity = *world.read_resource::<Entity>();
        let wait_sequence_handles = world.read_storage::<WaitSequenceHandle>();

        let wait_sequence_handle_actual = wait_sequence_handles
            .get(entity)
            .expect("Expected entity to have `WaitSequenceHandle` component.");
        assert_eq!(&wait_sequence_handle_expected, wait_sequence_handle_actual);
    }

    fn send_events(world: &mut World, events: Vec<SequenceUpdateEvent>) {
        let mut ec = world.write_resource::<EventChannel<SequenceUpdateEvent>>();
        ec.iter_write(events.into_iter())
    }

    fn sequence_begin_events(world: &mut World) -> Vec<SequenceUpdateEvent> {
        let entity = *world.read_resource::<Entity>();
        vec![SequenceUpdateEvent::SequenceBegin {
            entity,
            sequence_id: SEQUENCE_ID_CURRENT,
        }]
    }

    fn frame_begin_events(world: &mut World) -> Vec<SequenceUpdateEvent> {
        let entity = *world.read_resource::<Entity>();
        vec![SequenceUpdateEvent::FrameBegin {
            entity,
            frame_index: 0,
        }]
    }
}
