use amethyst::{
    assets::AssetStorage,
    core::Transform,
    ecs::{Entities, Join, Read, ReadStorage, System, World, WriteStorage},
    renderer::{SpriteRender, SpriteSheet},
    shred::{ResourceId, SystemData},
    ui::{Anchor, Interactable, UiTransform},
};
use asset_model::loaded::{AssetId, AssetIdMappings};
use derivative::Derivative;
use derive_new::new;
use log::error;

/// Creates / updates the `UiTransform` of an object based on its `Transform` and current sprite.
///
/// This will allow it to receive `UiEvent`s, so that input button reactions can be reacted to.
#[derive(Debug, Default, new)]
pub struct InteractableObjectSyncSystem;

/// `InteractableObjectSyncSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct InteractableObjectSyncSystemData<'s> {
    /// `Entities`.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// `AssetId` components.
    #[derivative(Debug = "ignore")]
    pub asset_ids: ReadStorage<'s, AssetId>,
    /// `AssetIdMappings` resource.
    #[derivative(Debug = "ignore")]
    pub asset_id_mappings: Read<'s, AssetIdMappings>,
    /// `Interactable` components.
    #[derivative(Debug = "ignore")]
    pub interactables: ReadStorage<'s, Interactable>,
    /// `SpriteRender` components.
    #[derivative(Debug = "ignore")]
    pub sprite_renders: ReadStorage<'s, SpriteRender>,
    /// `SpriteSheet` assets.
    #[derivative(Debug = "ignore")]
    pub sprite_sheet_assets: Read<'s, AssetStorage<SpriteSheet>>,
    /// `Transform` components.
    #[derivative(Debug = "ignore")]
    pub transforms: ReadStorage<'s, Transform>,
    /// `UiTransform` components.
    #[derivative(Debug = "ignore")]
    pub ui_transforms: WriteStorage<'s, UiTransform>,
}

impl<'s> System<'s> for InteractableObjectSyncSystem {
    type SystemData = InteractableObjectSyncSystemData<'s>;

    fn run(
        &mut self,
        InteractableObjectSyncSystemData {
            entities,
            asset_ids,
            asset_id_mappings,
            interactables,
            sprite_renders,
            sprite_sheet_assets,
            transforms,
            mut ui_transforms,
        }: Self::SystemData,
    ) {
        (
            &entities,
            asset_ids.maybe(),
            &interactables,
            &sprite_renders,
            &transforms,
        )
            .join()
            .for_each(
                |(entity, asset_id, _interactable, sprite_render, transform)| {
                    if let Some(sprite_sheet) = sprite_sheet_assets.get(&sprite_render.sprite_sheet)
                    {
                        let (x, y, z) = {
                            let translation = transform.translation();
                            (translation.x, translation.y, translation.z)
                        };
                        let (scale_x, scale_y) = {
                            let scale = transform.scale();
                            (scale.x, scale.y)
                        };
                        let (width, height) = sprite_sheet
                            .sprites
                            .get(sprite_render.sprite_number)
                            .or_else(|| {
                                let asset_slug_str = asset_id
                                    .and_then(|asset_id| asset_id_mappings.slug(*asset_id))
                                    .map(ToString::to_string);
                                let asset_slug_str = asset_slug_str
                                    .as_ref()
                                    .map(String::as_str)
                                    .unwrap_or("Unknown");
                                error!(
                                    "Invalid sprite number: `{}` for entity with asset slug: `{}`",
                                    sprite_render.sprite_number, asset_slug_str
                                );

                                sprite_sheet.sprites.get(0)
                            })
                            .map(|sprite| (sprite.width, sprite.height))
                            .unwrap_or((100., 100.));

                        if let Some(ui_transform) = ui_transforms.get_mut(entity) {
                            ui_transform.local_x = x;
                            ui_transform.local_y = y;
                            ui_transform.local_z = z;
                            ui_transform.width = width * scale_x;
                            ui_transform.height = height * scale_y;
                        } else {
                            let id = format!("{:?}", entity);
                            let anchor = Anchor::BottomLeft;
                            let pivot = Anchor::BottomLeft;
                            let ui_transform =
                                UiTransform::new(id, anchor, pivot, x, y, z, width, height);
                            ui_transforms
                                .insert(entity, ui_transform)
                                .expect("Failed to insert `Transform` component.");
                        }
                    }
                },
            );
    }
}
