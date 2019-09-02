use amethyst::{
    assets::{AssetStorage, Handle, Loader, Prefab, PrefabLoader},
    core::TransformBundle,
    ecs::{Builder, Entity, Read, ReadExpect, World, WorldExt},
    renderer::{
        loaders::load_from_srgba,
        palette::Srgba,
        sprite::{Sprite, SpriteSheet, SpriteSheetHandle},
        types::{DefaultBackend, TextureData},
        RenderEmptyBundle, Texture,
    },
    Error,
};
use amethyst_test::AmethystApplication;
use energy_loading::{EnergyLoadingBundle, ENERGY_PROCESSOR};
use energy_model::{
    config::{EnergyDefinition, EnergySequenceName},
    loaded::{Energy, EnergyHandle},
};
use energy_prefab::{EnergyPrefab, EnergyPrefabBundle, EnergyPrefabHandle};
use indexmap::IndexMap;
use object_model::config::{ObjectAssetData, ObjectDefinition, ObjectFrame, ObjectSequence};
use sequence_loading::SequenceLoadingBundle;
use sequence_model::config::{SequenceEndTransition, SequenceNameString};

#[test]
fn energy_prefab_load() -> Result<(), Error> {
    AmethystApplication::blank()
        .with_bundle(TransformBundle::new())
        .with_bundle(RenderEmptyBundle::<DefaultBackend>::new())
        .with_bundle(SequenceLoadingBundle::new())
        .with_bundle(EnergyLoadingBundle::new())
        .with_bundle(
            EnergyPrefabBundle::new().with_system_dependencies(&[String::from(ENERGY_PROCESSOR)]),
        )
        .with_effect(|world| {
            let energy_prefab_handle = {
                let (loader, energy_definition_assets, energy_prefab_loader) =
                    world.system_data::<TestSystemData>();
                let energy_definition_handle =
                    loader.load_from_data(energy_definition(), (), &energy_definition_assets);
                let object_asset_data =
                    ObjectAssetData::new(energy_definition_handle, sprite_sheet_handles(&world));
                let energy_prefab = EnergyPrefab::new(object_asset_data);
                energy_prefab_loader.load_from_data(Prefab::new_main(energy_prefab), ())
            };
            world.insert(energy_prefab_handle);
        })
        .with_effect(|_world| {}) // Allow texture to load.
        .with_effect(|world| {
            let energy_prefab_handle = (*world.read_resource::<EnergyPrefabHandle>()).clone();
            let energy_entity = world.create_entity().with(energy_prefab_handle).build();
            world.insert(energy_entity);
        })
        .with_effect(|_world| {})
        .with_assertion(|world| {
            let energy_entity = *world.read_resource::<Entity>();
            let energy_handles = world.read_storage::<EnergyHandle>();
            let energy_handle = energy_handles
                .get(energy_entity)
                .expect("Expected entity to have `EnergyHandle` component.");
            let energy_assets = world.read_resource::<AssetStorage<Energy>>();
            let _energy = energy_assets
                .get(energy_handle)
                .expect("Expected `Energy` to be loaded.");

            // TODO: assertions
        })
        .run_isolated()
}

fn energy_definition() -> EnergyDefinition {
    use energy_model::config::{EnergyFrame, EnergySequence};
    use sequence_model::config::Wait;

    let frames = vec![EnergyFrame::new(ObjectFrame {
        wait: Wait::new(5),
        ..Default::default()
    })];
    let sequence = EnergySequence::new(ObjectSequence {
        next: SequenceEndTransition::SequenceName(SequenceNameString::Name(
            EnergySequenceName::Hover,
        )),
        frames,
        ..Default::default()
    });
    let mut sequences = IndexMap::new();
    sequences.insert(
        SequenceNameString::Name(EnergySequenceName::Hover),
        sequence,
    );
    let object_definition = ObjectDefinition::new(sequences);

    EnergyDefinition::new(object_definition)
}

fn sprite_sheet_handles(world: &World) -> Vec<SpriteSheetHandle> {
    let loader = world.read_resource::<Loader>();
    let texture_assets = world.read_resource::<AssetStorage<Texture>>();
    let texture_builder = load_from_srgba(Srgba::new(0., 0., 0., 0.));
    let texture_handle: Handle<Texture> =
        loader.load_from_data(TextureData::from(texture_builder), (), &texture_assets);

    let image_w = 1;
    let image_h = 1;
    let sprite_w = 1;
    let sprite_h = 1;
    let pixel_left = 0;
    let pixel_top = 0;
    let offsets = [0.; 2];

    let sprite_sheet_assets = world.read_resource::<AssetStorage<SpriteSheet>>();
    let sprite_sheet = SpriteSheet {
        texture: texture_handle,
        sprites: vec![Sprite::from_pixel_values(
            image_w, image_h, sprite_w, sprite_h, pixel_left, pixel_top, offsets, false, false,
        )],
    }; // kcov-ignore

    vec![loader.load_from_data(sprite_sheet, (), &sprite_sheet_assets)]
}

type TestSystemData<'s> = (
    ReadExpect<'s, Loader>,
    Read<'s, AssetStorage<EnergyDefinition>>,
    PrefabLoader<'s, EnergyPrefab>,
);
