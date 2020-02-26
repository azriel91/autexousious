use amethyst::{
    derive::SystemDesc,
    ecs::{storage::ComponentEvent, BitSet, Join, Read, ReaderId, System, World, WriteStorage},
    shred::{ResourceId, SystemData},
    ui::{UiText, UiTransform},
};
use derivative::Derivative;
use derive_new::new;
use ui_model::play::UiFovScaleTransform;

/// Updates `SequenceId` when `UiTransform` changes.
#[derive(Debug, SystemDesc, new)]
#[system_desc(name(UiTransformInsertionRectifySystemDesc))]
pub struct UiTransformInsertionRectifySystem {
    /// Reader ID for sequence ID changes.
    #[system_desc(flagged_storage_reader(UiTransform))]
    ui_transform_rid: ReaderId<ComponentEvent>,
    /// Pre-allocated bitset to track insertions to `UiTransform`s.
    #[new(default)]
    #[system_desc(skip)]
    ui_transform_insertions: BitSet,
}

/// `UiTransformInsertionRectifySystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct UiTransformInsertionRectifySystemData<'s> {
    /// `UiFovScaleTransform` resource.
    #[derivative(Debug = "ignore")]
    pub ui_fov_scale_transform: Read<'s, UiFovScaleTransform>,
    /// `UiTransform` components.
    #[derivative(Debug = "ignore")]
    pub ui_transforms: WriteStorage<'s, UiTransform>,
    /// `UiText` components.
    #[derivative(Debug = "ignore")]
    pub ui_texts: WriteStorage<'s, UiText>,
}

impl<'s> System<'s> for UiTransformInsertionRectifySystem {
    type SystemData = UiTransformInsertionRectifySystemData<'s>;

    fn run(
        &mut self,
        UiTransformInsertionRectifySystemData {
            ui_fov_scale_transform,
            mut ui_transforms,
            mut ui_texts,
        }: Self::SystemData,
    ) {
        self.ui_transform_insertions.clear();

        ui_transforms
            .channel()
            .read(&mut self.ui_transform_rid)
            .for_each(|event| {
                if let ComponentEvent::Inserted(id) = event {
                    self.ui_transform_insertions.add(*id);
                }
            });

        (
            &mut ui_transforms,
            (&mut ui_texts).maybe(),
            &self.ui_transform_insertions,
        )
            .join()
            .for_each(|(ui_transform, ui_text, _)| {
                ui_fov_scale_transform.apply(ui_transform);

                if let Some(ui_text) = ui_text {
                    ui_text.font_size *= ui_fov_scale_transform.scale;
                }
            });
    }
}
