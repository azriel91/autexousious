use amethyst::{
    assets::{AssetStorage, Loader, PrefabData},
    core::{Parent, Transform},
    ecs::{Entity, Read, ReadExpect, WriteStorage},
    renderer::{Material, Mesh, MeshHandle, PosTex, Shape},
    utils::render::MaterialCreator,
    Error,
};
use derive_new::new;

use crate::HpBar;

const HP_BAR_LENGTH: f32 = 50.;
const HP_BAR_HEIGHT: f32 = 2.;
const HP_COLOUR: [f32; 4] = [1., 0.2, 0.1, 1.];

/// Prefab to attach all components of a HP bar.
///
/// These include:
///
/// * `HpBar`: Tag component.
/// * `Transform`: Coordinates of the HP bar to draw.
/// * `Parent`: Link to the parent entity whose `HealthPoints` the `HpBar` entity will display.
/// * `Material`: Material that determines the `HpBar`'s colour.
/// * `MeshHandle`: Handle to the mesh for drawing coordinates.
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
        MaterialCreator<'s>,
        ReadExpect<'s, Loader>,
        Read<'s, AssetStorage<Mesh>>,
        WriteStorage<'s, Material>,
        WriteStorage<'s, MeshHandle>,
    );
    type Result = ();

    fn add_to_entity(
        &self,
        entity: Entity,
        (
            hp_bars,
            transforms,
            parents,
            material_creator,
            loader,
            mesh_assets,
            materials,
            mesh_handles,
        ): &mut Self::SystemData,
        _entities: &[Entity],
    ) -> Result<(), Error> {
        hp_bars.insert(entity, HpBar::default())?;
        let mut transform = Transform::default();
        transform.set_z(1.);
        transform.set_scale(HP_BAR_LENGTH, HP_BAR_HEIGHT, 1.);
        transforms.insert(entity, transform)?;
        parents.insert(entity, Parent::new(self.game_object_entity))?;

        let material = material_creator.material_from_color(HP_COLOUR, ());
        materials.insert(entity, material)?;

        let vertices = Shape::Plane(None).generate::<Vec<PosTex>>(Some((1., 1., 0.)));
        let mesh_handle = loader.load_from_data(vertices, (), &mesh_assets);
        mesh_handles.insert(entity, mesh_handle)?;

        Ok(())
    }
}
