use amethyst::{
    assets::PrefabData,
    core::{Parent, Transform},
    ecs::{Entity, WriteStorage},
    Error,
};
use derive_new::new;

use crate::HpBar;

/// Prefab to attach all components of a HP bar.
///
/// These include:
///
/// * `HpBar`: Tag component.
/// * `Transform`: Coordinates of the HP bar to draw.
/// * `Parent`: Link to the parent entity whose `HealthPoints` the `HpBar` entity will display.
///
/// Ideally, the `Parent` component will be inserted by the `PrefabLoaderSystem`, so the (game
/// object) entity whose `HealthPoints` should displayed is specified as the `parent` of the `HpBar`
/// entity. However this is not currently possible ergonomically, see
/// <https://community.amethyst-engine.org/t/prefabs-with-special-cases-at-runtime/589> for
/// discussion.
///
/// The following components will be attached as development further progresses:
///
/// * `Handle<Material>`: Handle to the material
/// * `MeshHandle`: Handle to the mesh for drawing.
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
    );
    type Result = ();

    fn add_to_entity(
        &self,
        entity: Entity,
        (hp_bars, transforms, parents): &mut Self::SystemData,
        _entities: &[Entity],
    ) -> Result<(), Error> {
        hp_bars.insert(entity, HpBar::default()).map(|_| ())?;
        transforms
            .insert(entity, Transform::default())
            .map(|_| ())?;
        parents
            .insert(entity, Parent::new(self.game_object_entity))
            .map(|_| ())?;

        Ok(())
    }
}
