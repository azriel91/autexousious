use std::{fmt::Debug, marker::PhantomData, ops::Deref};

use amethyst::{
    ecs::{Entity, Read, ReadStorage, System, World, WorldExt, WriteStorage},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use asset_model::{loaded::ItemId, play::AssetWorld, ItemComponent};
use derivative::Derivative;
use derive_new::new;
use named_type::NamedType;
use named_type_derive::NamedType;
use sequence_model::{loaded::SequenceId, play::SequenceUpdateEvent};
use sequence_model_spi::loaded::{ComponentDataExt, SequenceComponentData};

/// Updates the sequence component based on the current sequence ID.
///
/// # Type Parameters
///
/// * `ICSCD`: Item component which is also sequence component data, e.g. `SequenceEndTransitions`.
#[derive(Debug, Default, NamedType, new)]
pub struct SequenceComponentUpdateSystem<ICSCD> {
    /// Reader ID for the `SequenceUpdateEvent` event channel.
    #[new(default)]
    reader_id: Option<ReaderId<SequenceUpdateEvent>>,
    /// Marker.
    phantom_data: PhantomData<ICSCD>,
}

/// `SequenceComponentUpdateSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct SequenceComponentUpdateSystemData<'s, ICSCD>
where
    ICSCD: ItemComponent<'s>
        + ComponentDataExt
        + Debug
        + Deref<Target = SequenceComponentData<<ICSCD as ComponentDataExt>::Component>>,
{
    /// Event channel for `SequenceUpdateEvent`s.
    #[derivative(Debug = "ignore")]
    pub sequence_update_ec: Read<'s, EventChannel<SequenceUpdateEvent>>,
    /// `ItemId` components.
    #[derivative(Debug = "ignore")]
    pub item_ids: ReadStorage<'s, ItemId>,
    /// `AssetWorld` resource.
    #[derivative(Debug = "ignore")]
    pub asset_world: Read<'s, AssetWorld>,
    /// Frame `Component` storages.
    #[derivative(Debug = "ignore")]
    pub sequence_components: WriteStorage<'s, <ICSCD as ComponentDataExt>::Component>,
}

impl<ICSCD> SequenceComponentUpdateSystem<ICSCD>
where
    ICSCD: ComponentDataExt
        + Debug
        + Deref<Target = SequenceComponentData<<ICSCD as ComponentDataExt>::Component>>,
{
    fn update_component(
        sequence_components: &mut WriteStorage<<ICSCD as ComponentDataExt>::Component>,
        sequence_component_data: &ICSCD,
        entity: Entity,
        sequence_id: SequenceId,
    ) {
        if let Some(component) = sequence_component_data
            .get(*sequence_id)
            .map(ICSCD::to_owned)
        {
            sequence_components
                .insert(entity, component)
                .expect("Failed to insert sequence component.");
        }
    }
}

impl<'s, ICSCD> System<'s> for SequenceComponentUpdateSystem<ICSCD>
where
    ICSCD: ItemComponent<'s>
        + ComponentDataExt
        + Debug
        + Deref<Target = SequenceComponentData<<ICSCD as ComponentDataExt>::Component>>,
{
    type SystemData = SequenceComponentUpdateSystemData<'s, ICSCD>;

    fn run(
        &mut self,
        SequenceComponentUpdateSystemData {
            sequence_update_ec,
            item_ids,
            asset_world,
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
                let item_components = asset_world.read_storage::<ICSCD>();
                let sequence_component_data = item_ids
                    .get(entity)
                    .and_then(|item_id| item_components.get(item_id.0));

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
