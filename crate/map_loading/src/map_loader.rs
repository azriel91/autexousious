use std::path::Path;

use amethyst::{
    assets::{AssetStorage, Loader, ProgressCounter},
    ecs::{World, WorldExt},
    renderer::{SpriteRender, SpriteSheet, Texture},
    Error,
};
use application::{load_in, resource::FindContext, Format};
use log::debug;
use map_model::{
    config::MapDefinition,
    loaded::{Map, MapHandle, Margins},
};
use sequence_model::{
    config::Wait,
    loaded::{WaitSequence, WaitSequenceHandle},
};
use sprite_loading::SpriteLoader;
use sprite_model::{
    config::SpritesDefinition,
    loaded::{SpriteRenderSequence, SpriteRenderSequenceHandle},
};

/// Loads assets specified by map configuration into the loaded map model.
#[derive(Debug)]
pub struct MapLoader;

impl MapLoader {
    /// Returns the loaded `Map` referenced by the asset record.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` to store the map's assets.
    /// * `base_dir`: Base directory from which to load the map.
    pub fn load(world: &World, base_dir: &Path) -> Result<MapHandle, Error> {
        debug!("Loading map in `{}`", base_dir.display());

        let map_definition = load_in::<MapDefinition, _>(base_dir, "map.yaml", Format::Yaml, None)?;
        let sprite_load_result =
            load_in::<SpritesDefinition, _>(&base_dir, "sprites.yaml", Format::Yaml, None)
                .and_then(|sprites_definition| {
                    let loader = &world.read_resource::<Loader>();
                    let texture_assets = &world.read_resource::<AssetStorage<Texture>>();
                    let sprite_sheet_assets = &world.read_resource::<AssetStorage<SpriteSheet>>();

                    SpriteLoader::load(
                        &mut ProgressCounter::default(),
                        loader,
                        texture_assets,
                        sprite_sheet_assets,
                        &sprites_definition,
                        &base_dir,
                    )
                });

        let loaded_sprites = match sprite_load_result {
            Ok(loaded_sprites) => Ok(Some(loaded_sprites)),
            Err(e) => {
                if e.as_error().downcast_ref::<FindContext>().is_some() {
                    Ok(None)
                } else {
                    Err(e)
                }
            }
        }?;

        let (sprite_sheet_handles, wait_sequence_handles, sprite_render_sequence_handles) = {
            if let Some(sprite_sheet_handles) = loaded_sprites {
                let loader = world.read_resource::<Loader>();
                let wait_sequence_assets = world.read_resource::<AssetStorage<WaitSequence>>();
                let sprite_render_sequence_assets =
                    world.read_resource::<AssetStorage<SpriteRenderSequence>>();

                let sequence_handles = (
                    Vec::<WaitSequenceHandle>::with_capacity(map_definition.layers.len()),
                    Vec::<SpriteRenderSequenceHandle>::with_capacity(map_definition.layers.len()),
                );
                let (wait_sequence_handles, sprite_render_sequence_handles) =
                    map_definition.layers.iter().fold(
                        sequence_handles,
                        |(mut wait_sequence_handles, mut sprite_render_sequence_handles), layer| {
                            let wait_sequence = WaitSequence::new(
                                layer
                                    .frames
                                    .iter()
                                    .map(|frame| frame.wait)
                                    .collect::<Vec<Wait>>(),
                            );
                            let sprite_render_sequence = SpriteRenderSequence::new(
                                layer
                                    .frames
                                    .iter()
                                    .map(|frame| {
                                        let sprite_ref = &frame.sprite;
                                        let sprite_sheet =
                                            sprite_sheet_handles[sprite_ref.sheet].clone();
                                        let sprite_number = sprite_ref.index;
                                        SpriteRender {
                                            sprite_sheet,
                                            sprite_number,
                                        }
                                    })
                                    .collect::<Vec<SpriteRender>>(),
                            );

                            let wait_sequence_handle =
                                loader.load_from_data(wait_sequence, (), &wait_sequence_assets);
                            let sprite_render_sequence_handle = loader.load_from_data(
                                sprite_render_sequence,
                                (),
                                &sprite_render_sequence_assets,
                            );

                            wait_sequence_handles.push(wait_sequence_handle);
                            sprite_render_sequence_handles.push(sprite_render_sequence_handle);

                            (wait_sequence_handles, sprite_render_sequence_handles)
                        },
                    );
                (
                    sprite_sheet_handles,
                    wait_sequence_handles,
                    sprite_render_sequence_handles,
                )
            } else {
                (Vec::new(), Vec::new(), Vec::new())
            }
        };

        let margins = Margins::from(map_definition.header.bounds);

        let map = Map::new(
            map_definition,
            margins,
            sprite_sheet_handles,
            wait_sequence_handles,
            sprite_render_sequence_handles,
        );

        let loader = world.read_resource::<Loader>();
        let map_handle = loader.load_from_data(map, (), &world.read_resource());
        Ok(map_handle)
    }
}

#[cfg(test)]
mod tests {
    use amethyst::{
        assets::AssetStorage,
        core::TransformBundle,
        ecs::WorldExt,
        renderer::{types::DefaultBackend, RenderEmptyBundle},
        Error,
    };
    use amethyst_test::{AmethystApplication, EffectReturn};
    use assets_test::MAP_EMPTY_PATH;
    use map_model::loaded::{Map, MapHandle};
    use sequence_loading::SequenceLoadingBundle;

    use super::MapLoader;
    use crate::MapLoadingBundle;

    // Map with layers case covered by `MapLoadingBundle` test

    #[test]
    fn loads_map_without_sprites() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_bundle(TransformBundle::new())
            .with_bundle(RenderEmptyBundle::<DefaultBackend>::new())
            .with_bundle(SequenceLoadingBundle::new())
            .with_bundle(MapLoadingBundle::new())
            .with_effect(|world| {
                let map_handle =
                    MapLoader::load(world, &MAP_EMPTY_PATH).expect("Failed to load map.");

                world.insert(EffectReturn(map_handle));
            })
            .with_assertion(|world| {
                let map_handle = world.read_resource::<EffectReturn<MapHandle>>().0.clone();
                let map_store = world.read_resource::<AssetStorage<Map>>();
                let map = map_store
                    .get(&map_handle)
                    .expect("Expected map to be loaded.");

                // See empty/map.yaml
                assert!(map.wait_sequence_handles.is_empty());
            })
            .run_isolated()
    }
}
