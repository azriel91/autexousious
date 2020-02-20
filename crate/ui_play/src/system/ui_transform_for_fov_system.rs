use amethyst::{
    core::SystemDesc,
    ecs::{Entities, Join, Read, ReadExpect, System, World, WorldExt, WriteStorage},
    shred::{ResourceId, SystemData},
    ui::{UiText, UiTransform},
    window::ScreenDimensions,
};
use camera_model::play::CameraZoomDimensions;
use derivative::Derivative;
use derive_new::new;

/// Builds a `UiTransformForFovSystem`.
#[derive(Debug, Default)]
pub struct UiTransformForFovSystemDesc;

impl<'a, 'b> SystemDesc<'a, 'b, UiTransformForFovSystem> for UiTransformForFovSystemDesc {
    fn build(self, world: &mut World) -> UiTransformForFovSystem {
        <UiTransformForFovSystem as System<'_>>::SystemData::setup(world);

        let screen_dimensions = &*world.read_resource::<ScreenDimensions>();
        let screen_dimensions = screen_dimensions.clone();

        UiTransformForFovSystem::new(screen_dimensions, 1., 0., 0.)
    }
}

/// Updates `WidgetStatus` based on `ControlInputEvent`s and `Sibling`s.
#[derive(Debug, new)]
pub struct UiTransformForFovSystem {
    /// Last `ScreenDimensions`.
    pub screen_dimensions_last: ScreenDimensions,
    /// Last scale change.
    pub scale_last: f32,
    /// Last width change.
    pub width_delta_last: f32,
    /// Last width change.
    pub height_delta_last: f32,
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

            let width_delta =
                (screen_dimensions.width() - camera_zoom_dimensions.width * scale) / 2.;
            let height_delta =
                (screen_dimensions.height() - camera_zoom_dimensions.height * scale) / 2.;

            (&entities, &mut ui_transforms)
                .join()
                .for_each(|(entity, mut ui_transform)| {
                    ui_transform.local_x -= self.width_delta_last;
                    ui_transform.local_x /= self.scale_last;
                    ui_transform.local_x *= scale;
                    ui_transform.local_x += width_delta;

                    ui_transform.local_y -= self.height_delta_last;
                    ui_transform.local_y /= self.scale_last;
                    ui_transform.local_y *= scale;
                    ui_transform.local_y += height_delta;

                    ui_transform.width /= self.scale_last;
                    ui_transform.width *= scale;
                    ui_transform.height /= self.scale_last;
                    ui_transform.height *= scale;

                    if let Some(ui_text) = ui_texts.get_mut(entity) {
                        ui_text.font_size /= self.scale_last;
                        ui_text.font_size *= scale;
                    }
                });

            self.scale_last = scale;
            self.width_delta_last = width_delta;
            self.height_delta_last = height_delta;
            self.screen_dimensions_last = screen_dimensions.clone();
        }
    }
}
