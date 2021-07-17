use amethyst::{
    ecs::{
        storage::ComponentEvent, BitSet, Entities, Join, ReadStorage, ReaderId, System, World,
        WriteStorage,
    },
    shred::{ResourceId, SystemData},
};
use derivative::Derivative;
use derive_new::new;
use sequence_model::loaded::SequenceId;
use ui_model_spi::{config::WidgetStatus, loaded::WidgetStatusSequences};

/// Updates `SequenceId` when `WidgetStatus` changes.
#[derive(Debug, Default, new)]
pub struct WidgetSequenceUpdateSystem {
    /// Reader ID for sequence ID changes.
    #[new(default)]
    widget_status_rid: Option<ReaderId<ComponentEvent>>,
    /// Pre-allocated bitset to track insertions and modifications to
    /// `WidgetStatus`s.
    #[new(default)]
    widget_status_updates: BitSet,
}

/// `WidgetSequenceUpdateSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct WidgetSequenceUpdateSystemData<'s> {
    /// `Entities` resource.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// `WidgetStatus` components.
    #[derivative(Debug = "ignore")]
    pub widget_statuses: ReadStorage<'s, WidgetStatus>,
    /// `WidgetStatusSequences` components.
    #[derivative(Debug = "ignore")]
    pub widget_status_sequenceses: ReadStorage<'s, WidgetStatusSequences>,
    /// `SequenceId` components.
    #[derivative(Debug = "ignore")]
    pub sequence_ids: WriteStorage<'s, SequenceId>,
}

impl<'s> System<'s> for WidgetSequenceUpdateSystem {
    type SystemData = WidgetSequenceUpdateSystemData<'s>;

    fn run(
        &mut self,
        WidgetSequenceUpdateSystemData {
            entities,
            widget_statuses,
            widget_status_sequenceses,
            mut sequence_ids,
        }: Self::SystemData,
    ) {
        self.widget_status_updates.clear();

        widget_statuses
            .channel()
            .read(
                self.widget_status_rid
                    .as_mut()
                    .expect("Expected `widget_status_rid` to be set."),
            )
            .for_each(|event| match event {
                ComponentEvent::Inserted(id) | ComponentEvent::Modified(id) => {
                    self.widget_status_updates.add(*id);
                }
                ComponentEvent::Removed(_id) => {}
            });

        (
            &entities,
            &widget_statuses,
            &widget_status_sequenceses,
            &self.widget_status_updates,
        )
            .join()
            .for_each(|(entity, widget_status, widget_status_sequences, _)| {
                if let Some(sequence_id) = widget_status_sequences.get(widget_status).copied() {
                    sequence_ids
                        .insert(entity, sequence_id)
                        .expect("Failed to insert `SequenceId` component.");
                }
            });
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.widget_status_rid =
            Some(WriteStorage::<'_, WidgetStatus>::fetch(world).register_reader());
    }
}
