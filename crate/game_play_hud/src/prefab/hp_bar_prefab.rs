use amethyst::{
    assets::{AssetStorage, Loader, PrefabData},
    core::{Parent, Transform},
    ecs::{Entity, Read, ReadExpect, WriteStorage},
    renderer::{Sprite, SpriteRender, SpriteSheet, Texture, Transparent},
    Error,
};
use derive_new::new;

use crate::{HpBar, HP_BAR_HEIGHT, HP_BAR_LENGTH};

const HP_COLOUR: [f32; 4] = [1., 0.2, 0.1, 1.];

/// Prefab to attach all components of a HP bar.
///
/// These include:
///
/// * `HpBar`: Tag component.
/// * `Transform`: Coordinates of the HP bar to draw.
/// * `Parent`: Link to the parent entity whose `HealthPoints` the `HpBar` entity will display.
/// * `SpriteRender`: Indicates which "sprite" (colour) of the `HpBar` to draw.
/// * `Transparent`: Tags the `HpBar` for sorting when rendering.
///
/// Ideally, the `Parent` component will be inserted by the `PrefabLoaderSystem`, so the (game
/// object) entity whose `HealthPoints` should displayed is specified as the `parent` of the `HpBar`
/// entity. However this is not currently possible ergonomically, see
/// <https://community.amethyst-engine.org/t/prefabs-with-special-cases-at-runtime/589> for
/// discussion.
#[derive(Clone, Copy, Debug, PartialEq, new)]
pub struct HpBarPrefab {
    /// Entity whose `HealthPoints` to display.
    pub game_object_entity: Entity,
}

impl<'s> PrefabData<'s> for HpBarPrefab {
    type SystemData = (
        WriteStorage<'s, HpBar>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Parent>,
        ReadExpect<'s, Loader>,
        Read<'s, AssetStorage<Texture>>,
        Read<'s, AssetStorage<SpriteSheet>>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, Transparent>,
    );
    type Result = ();

    fn add_to_entity(
        &self,
        entity: Entity,
        (
            hp_bars,
            transforms,
            parents,
            loader,
            texture_assets,
            sprite_sheet_assets,
            sprite_renders,
            transparents,
        ): &mut Self::SystemData,
        _entities: &[Entity],
    ) -> Result<(), Error> {
        hp_bars.insert(entity, HpBar::default())?;
        let mut transform = Transform::default();
        transform.set_z(1.);
        transform.set_scale(HP_BAR_LENGTH, HP_BAR_HEIGHT, 1.);
        transforms.insert(entity, transform)?;
        parents.insert(entity, Parent::new(self.game_object_entity))?;

        let sprite_sheet_handle = {
            let texture_handle = loader.load_from_data(HP_COLOUR.into(), (), &texture_assets);
            let sprite = Sprite::from_pixel_values(1, 1, 1, 1, 0, 0, [0.; 2]);
            let sprites = vec![sprite];

            let sprite_sheet = SpriteSheet {
                texture: texture_handle,
                sprites,
            };

            loader.load_from_data(sprite_sheet, (), sprite_sheet_assets)
        };
        let sprite_render = SpriteRender {
            sprite_sheet: sprite_sheet_handle,
            sprite_number: 0,
        };
        sprite_renders.insert(entity, sprite_render)?;
        transparents.insert(entity, Transparent)?;

        Ok(())
    }
}
