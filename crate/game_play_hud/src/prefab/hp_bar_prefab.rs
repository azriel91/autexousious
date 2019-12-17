use amethyst::{
    assets::PrefabData,
    core::{math::Vector3, Transform},
    ecs::{Entity, World, WriteStorage},
    renderer::{transparent::Transparent, SpriteRender},
    shred::{ResourceId, SystemData},
    Error,
};
use asset_gfx_gen::{ColourSpriteSheetGen, ColourSpriteSheetGenData};
use chase_model::play::{ChaseModeStick, TargetObject};
use derivative::Derivative;
use derive_new::new;
use parent_model::play::ParentObject;

use crate::{HpBar, HP_BAR_HEIGHT, HP_BAR_LENGTH, HP_BAR_SPRITE_COUNT};

const COLOUR_HP_LOW: [f32; 4] = [0.8, 0., 0., 0.8];
const COLOUR_HP_HIGH: [f32; 4] = [0.1, 0.9, 0.1, 0.8];

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

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct HpBarPrefabSystemData<'s> {
    /// `HpBar` components.
    #[derivative(Debug = "ignore")]
    pub hp_bars: WriteStorage<'s, HpBar>,
    /// `Transform` components.
    #[derivative(Debug = "ignore")]
    pub transforms: WriteStorage<'s, Transform>,
    /// `ParentObject` components.
    #[derivative(Debug = "ignore")]
    pub parent_objects: WriteStorage<'s, ParentObject>,
    /// `TargetObject` components.
    #[derivative(Debug = "ignore")]
    pub target_objects: WriteStorage<'s, TargetObject>,
    /// `ChaseModeStick` components.
    #[derivative(Debug = "ignore")]
    pub chase_mode_sticks: WriteStorage<'s, ChaseModeStick>,
    /// System data needed to load colour sprites.
    #[derivative(Debug = "ignore")]
    pub colour_sprite_sheet_gen_data: ColourSpriteSheetGenData<'s>,
    /// `SpriteRender` components.
    #[derivative(Debug = "ignore")]
    pub sprite_renders: WriteStorage<'s, SpriteRender>,
    /// `Transparent` components.
    #[derivative(Debug = "ignore")]
    pub transparents: WriteStorage<'s, Transparent>,
}

impl<'s> PrefabData<'s> for HpBarPrefab {
    type SystemData = HpBarPrefabSystemData<'s>;
    type Result = ();

    fn add_to_entity(
        &self,
        entity: Entity,
        HpBarPrefabSystemData {
            hp_bars,
            transforms,
            parent_objects,
            target_objects,
            chase_mode_sticks,
            colour_sprite_sheet_gen_data,
            sprite_renders,
            transparents,
        }: &mut Self::SystemData,
        _entities: &[Entity],
        _children: &[Entity],
    ) -> Result<(), Error> {
        let parent_translation = transforms
            .get(self.game_object_entity)
            .map(Transform::translation)
            .copied();

        hp_bars.insert(entity, HpBar::default())?;
        let mut transform = Transform::default();
        if let Some(translation) = parent_translation {
            *transform.translation_mut() = translation;
        }
        transform.set_scale(Vector3::new(HP_BAR_LENGTH, HP_BAR_HEIGHT, 1.));
        transforms.insert(entity, transform)?;
        parent_objects.insert(entity, ParentObject::new(self.game_object_entity))?;
        target_objects.insert(entity, TargetObject::new(self.game_object_entity))?;
        chase_mode_sticks.insert(entity, Default::default())?;

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
