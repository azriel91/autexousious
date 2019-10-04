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

/// `SequenceComponentUpdateSystemData`.
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
