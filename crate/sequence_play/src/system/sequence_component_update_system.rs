use std::{
    fmt::Debug,
    marker::PhantomData,
    ops::{Deref, Index},
};

use amethyst::{
    ecs::{Entity, Read, ReadStorage, System, World, WriteStorage},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use asset_model::loaded::AssetId;
use derivative::Derivative;
use derive_new::new;
use named_type::NamedType;
use named_type_derive::NamedType;
use sequence_model::{loaded::SequenceId, play::SequenceUpdateEvent};
use sequence_model_spi::loaded::{ComponentDataExt, SequenceComponentData};
use slotmap::{SecondaryMap, SparseSecondaryMap};

/// Extensions to allow slotmap inner types to be accessed.
pub trait AssetScdExt<V> {
    /// Returns the component if it exists.
    fn get(&self, asset_id: AssetId) -> Option<&V>;
}

impl<V> AssetScdExt<V> for SecondaryMap<AssetId, V> {
    fn get(&self, asset_id: AssetId) -> Option<&V> {
        self.get(asset_id)
    }
}

impl<V> AssetScdExt<V> for SparseSecondaryMap<AssetId, V> {
    fn get(&self, asset_id: AssetId) -> Option<&V> {
        self.get(asset_id)
    }
}

/// Updates the sequence component based on the current sequence ID.
///
/// # Type Parameters
///
/// * `AssetScd`: Resource type that the sequence component data is stored in, e.g. `AssetMargins`.
/// * `SCD`: Type of sequence component data, e.g. `SequenceEndTransitions`.
#[derive(Debug, Default, NamedType, new)]
pub struct SequenceComponentUpdateSystem<AssetScd, SCD>
where
    AssetScd: AssetScdExt<SCD> + Index<AssetId, Output = SCD>,
    SCD: ComponentDataExt
        + Debug
        + Deref<Target = SequenceComponentData<<SCD as ComponentDataExt>::Component>>,
{
    /// Reader ID for the `SequenceUpdateEvent` event channel.
    #[new(default)]
    reader_id: Option<ReaderId<SequenceUpdateEvent>>,
    /// Marker.
    phantom_data: PhantomData<(AssetScd, SCD)>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct SequenceComponentUpdateSystemData<'s, AssetScd, SCD>
where
    AssetScd: AssetScdExt<SCD> + Default + Index<AssetId, Output = SCD> + Send + Sync + 'static,
    SCD: ComponentDataExt
        + Debug
        + Deref<Target = SequenceComponentData<<SCD as ComponentDataExt>::Component>>,
{
    /// Event channel for `SequenceUpdateEvent`s.
    #[derivative(Debug = "ignore")]
    pub sequence_update_ec: Read<'s, EventChannel<SequenceUpdateEvent>>,
    /// `AssetId` components.
    #[derivative(Debug = "ignore")]
    pub asset_ids: ReadStorage<'s, AssetId>,
    /// `AssetScd` resource.
    #[derivative(Debug = "ignore")]
    pub asset_scd: Read<'s, AssetScd>,
    /// Frame `Component` storages.
    #[derivative(Debug = "ignore")]
    pub sequence_components: WriteStorage<'s, <SCD as ComponentDataExt>::Component>,
}

impl<AssetScd, SCD> SequenceComponentUpdateSystem<AssetScd, SCD>
where
    AssetScd: AssetScdExt<SCD> + Index<AssetId, Output = SCD>,
    SCD: ComponentDataExt
        + Debug
        + Deref<Target = SequenceComponentData<<SCD as ComponentDataExt>::Component>>,
{
    fn update_component(
        sequence_components: &mut WriteStorage<<SCD as ComponentDataExt>::Component>,
        sequence_component_data: &SCD,
        entity: Entity,
        sequence_id: SequenceId,
    ) {
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

impl<'s, AssetScd, SCD> System<'s> for SequenceComponentUpdateSystem<AssetScd, SCD>
where
    AssetScd: AssetScdExt<SCD> + Default + Index<AssetId, Output = SCD> + Send + Sync + 'static,
    SCD: ComponentDataExt
        + Debug
        + Deref<Target = SequenceComponentData<<SCD as ComponentDataExt>::Component>>,
{
    type SystemData = SequenceComponentUpdateSystemData<'s, AssetScd, SCD>;

    fn run(
        &mut self,
        SequenceComponentUpdateSystemData {
            sequence_update_ec,
            asset_ids,
            asset_scd,
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
                let sequence_component_data = asset_ids
                    .get(entity)
                    .and_then(|asset_id| asset_scd.get(*asset_id));

                // Some entities will have sequence update events, but not this particular sequence
                // component data asset.
                if let Some(sequence_component_data) = sequence_component_data {
                    Self::update_component(
                        &mut sequence_components,
                        sequence_component_data,
                        entity,
                        sequence_id,
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
    use amethyst::{
        ecs::{Builder, Entity, World, WorldExt},
        shrev::EventChannel,
        Error,
    };
    use application_test_support::{AssetQueries, AutexousiousApplication, SequenceQueries};
    use assets_test::CHAR_BAT_SLUG;
    use sequence_model::{
        loaded::{AssetWaitSequenceHandles, SequenceId, WaitSequenceHandle, WaitSequenceHandles},
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
    fn does_not_panic_when_entity_does_not_have_asset_id() -> Result<(), Error> {
        run_test(sequence_begin_events, false, SEQUENCE_ID_PREV)
    }

    fn run_test(
        sequence_update_events_fn: fn(&mut World) -> Vec<SequenceUpdateEvent>,
        with_asset_id: bool,
        sequence_id_expected: SequenceId,
    ) -> Result<(), Error> {
        AutexousiousApplication::game_base()
            .with_system(
                SequenceComponentUpdateSystem::<AssetWaitSequenceHandles, WaitSequenceHandles>::new(
                ),
                "",
                &[],
            )
            .with_effect(move |world| initial_values(world, with_asset_id))
            .with_effect(move |world| {
                let events = sequence_update_events_fn(world);
                send_events(world, events);
            })
            .with_assertion(move |world| {
                let wait_sequence_handle_expected = SequenceQueries::wait_sequence_handle(
                    world,
                    &*CHAR_BAT_SLUG,
                    sequence_id_expected,
                );
                expect_component_values(world, wait_sequence_handle_expected)
            })
            .run_isolated()
    }

    fn initial_values(world: &mut World, with_asset_id: bool) {
        let entity = {
            let wait_sequence_handle =
                SequenceQueries::wait_sequence_handle(world, &*CHAR_BAT_SLUG, SEQUENCE_ID_PREV);
            let asset_id = if with_asset_id {
                Some(AssetQueries::id(world, &*CHAR_BAT_SLUG))
            } else {
                None
            };

            let mut entity_builder = world
                .create_entity()
                .with(SEQUENCE_ID_CURRENT)
                .with(wait_sequence_handle);

            if let Some(asset_id) = asset_id {
                entity_builder = entity_builder.with(asset_id);
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
