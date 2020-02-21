use amethyst::{
    core::SystemDesc,
    ecs::{Entities, Join, Read, ReadExpect, System, World, WorldExt, Write, WriteStorage},
    shred::{ResourceId, SystemData},
    ui::{UiText, UiTransform},
    window::ScreenDimensions,
};
use camera_model::play::CameraZoomDimensions;
use derivative::Derivative;
use derive_new::new;
use ui_model::play::UiFovScaleTransform;

/// Builds a `UiTransformForFovSystem`.
#[derive(Debug, Default)]
pub struct UiTransformForFovSystemDesc;

impl<'a, 'b> SystemDesc<'a, 'b, UiTransformForFovSystem> for UiTransformForFovSystemDesc {
    fn build(self, world: &mut World) -> UiTransformForFovSystem {
        <UiTransformForFovSystem as System<'_>>::SystemData::setup(world);

        let screen_dimensions = &*world.read_resource::<ScreenDimensions>();
        let screen_dimensions = screen_dimensions.clone();

        UiTransformForFovSystem::new(screen_dimensions)
    }
}

/// Updates `WidgetStatus` based on `ControlInputEvent`s and `Sibling`s.
#[derive(Debug, new)]
pub struct UiTransformForFovSystem {
    /// Last `ScreenDimensions`.
    pub screen_dimensions_last: ScreenDimensions,
}

/// `UiTransformForFovSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct UiTransformForFovSystemData<'s> {
    /// `CameraZoomDimensions` resource.
    #[derivative(Debug = "ignore")]
    pub camera_zoom_dimensions: Read<'s, CameraZoomDimensions>,
    /// `ScreenDimensions` resource.
    #[derivative(Debug = "ignore")]
    pub screen_dimensions: ReadExpect<'s, ScreenDimensions>,
    /// `UiFovScaleTransform` resource.
    #[derivative(Debug = "ignore")]
    pub ui_fov_scale_transform: Write<'s, UiFovScaleTransform>,
    /// `Entities`.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// `UiTransform` components.
    #[derivative(Debug = "ignore")]
    pub ui_transforms: WriteStorage<'s, UiTransform>,
    /// `UiText` components.
    #[derivative(Debug = "ignore")]
    pub ui_texts: WriteStorage<'s, UiText>,
}

impl<'s> System<'s> for UiTransformForFovSystem {
    type SystemData = UiTransformForFovSystemData<'s>;

    fn run(
        &mut self,
        UiTransformForFovSystemData {
            camera_zoom_dimensions,
            screen_dimensions,
            mut ui_fov_scale_transform,
            entities,
            mut ui_transforms,
            mut ui_texts,
        }: Self::SystemData,
    ) {
        if self.screen_dimensions_last != *screen_dimensions {
            let aspect_ratio_diff =
                screen_dimensions.aspect_ratio() - camera_zoom_dimensions.aspect_ratio();
            let scale = if aspect_ratio_diff > 0. {
                // Wider than original dimensions, so we use screen height / original height for the
                // scale.
                screen_dimensions.height() / camera_zoom_dimensions.height
            } else if aspect_ratio_diff < 0. {
                // Narrower than original dimensions, so we use screen width / original width for
                // the scale.
                screen_dimensions.width() / camera_zoom_dimensions.width
            } else {
                1.
            };

            let x_offset = (screen_dimensions.width() - camera_zoom_dimensions.width * scale) / 2.;
            let y_offset =
                (screen_dimensions.height() - camera_zoom_dimensions.height * scale) / 2.;

            (&entities, &mut ui_transforms)
                .join()
                .for_each(|(entity, mut ui_transform)| {
                    ui_transform.local_x -= ui_fov_scale_transform.x_offset;
                    ui_transform.local_x /= ui_fov_scale_transform.scale;
                    ui_transform.local_x *= scale;
                    ui_transform.local_x += x_offset;

                    ui_transform.local_y -= ui_fov_scale_transform.y_offset;
                    ui_transform.local_y /= ui_fov_scale_transform.scale;
                    ui_transform.local_y *= scale;
                    ui_transform.local_y += y_offset;

                    ui_transform.width /= ui_fov_scale_transform.scale;
                    ui_transform.width *= scale;
                    ui_transform.height /= ui_fov_scale_transform.scale;
                    ui_transform.height *= scale;

                    if let Some(ui_text) = ui_texts.get_mut(entity) {
                        ui_text.font_size /= ui_fov_scale_transform.scale;
                        ui_text.font_size *= scale;
                    }
                });

            ui_fov_scale_transform.scale = scale;
            ui_fov_scale_transform.x_offset = x_offset;
            ui_fov_scale_transform.y_offset = y_offset;
            self.screen_dimensions_last = screen_dimensions.clone();
        }
    }
}
