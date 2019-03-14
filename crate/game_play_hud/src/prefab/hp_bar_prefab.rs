use amethyst::{
    assets::PrefabData,
    core::{Parent, Transform},
    ecs::{Entity, WriteStorage},
    renderer::{SpriteRender, Transparent},
    Error,
};
use asset_gfx_gen::{ColourSpriteSheetGen, ColourSpriteSheetGenData};
use derive_new::new;

use crate::{HpBar, HP_BAR_HEIGHT, HP_BAR_LENGTH, HP_BAR_SPRITE_COUNT};

const COLOUR_HP_LOW: [f32; 4] = [1., 0.2, 0.1, 1.];
const COLOUR_HP_HIGH: [f32; 4] = [0.2, 1., 0.1, 1.];

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
        ColourSpriteSheetGenData<'s>,
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
            colour_sprite_sheet_gen_data,
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

        let sprite_render = ColourSpriteSheetGen::gradient(
            colour_sprite_sheet_gen_data,
            COLOUR_HP_LOW,
            COLOUR_HP_HIGH,
            HP_BAR_SPRITE_COUNT,
        );
        sprite_renders.insert(entity, sprite_render)?;
        transparents.insert(entity, Transparent)?;

        Ok(())
    }
}
