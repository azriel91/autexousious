use std::{fmt::Debug, marker::PhantomData, ops::Deref};

use amethyst::{
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

/// Updates the frame component value based on the current frame component data
/// handle.
#[derive(Debug, Default, new)]
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

/// `FrameComponentUpdateSystemData`.
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
                matches!(
                    ev,
                    SequenceUpdateEvent::SequenceBegin { .. }
                        | SequenceUpdateEvent::FrameBegin { .. }
                )
            })
            .for_each(|ev| {
                let entity = ev.entity();
                let frame_index = ev.frame_index();

                let frame_component_data_handle = frame_component_data_handles.get(entity);

                // Some entities will have sequence update events, but not this particular
                // sequence component.
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
