use std::{collections::HashMap, mem, path::PathBuf};

use amethyst::{
    assets::{AssetStorage, Handle, Loader, ProgressCounter},
    ecs::{Read, ReadExpect, System, Write},
    renderer::{SpriteRender, SpriteSheet, Texture},
};
use asset_loading::{AssetDiscovery, TomlFormat};
use asset_model::config::{AssetIndex, AssetRecord};
use derivative::Derivative;
use derive_new::new;
use game_model::loaded::MapAssets;
use log::debug;
use map_model::{
    config::MapDefinition,
    loaded::{Map, Margins},
};
use sequence_model::{
    config::Wait,
    loaded::{
        ComponentSequence, ComponentSequences, ComponentSequencesHandle, WaitSequence,
        WaitSequenceHandle,
    },
};
use shred_derive::SystemData;
use sprite_loading::SpriteLoader;
use sprite_model::{
    config::SpritesDefinition,
    loaded::{SpriteRenderSequence, SpriteRenderSequenceHandle},
};
use typename_derive::TypeName;

use crate::{MapAssetHandles, MapLoadingStatus};

/// Loads game object assets.
#[derive(Default, Derivative, TypeName, new)]
#[derivative(Debug(bound = ""))]
pub struct MapAssetLoadingSystem {
    /// Path to the assets directory.
    assets_dir: PathBuf,
    /// Tracks loaded assets.
    #[derivative(Debug = "ignore")]
    #[new(default)]
    progress_counter: ProgressCounter,

    // Fields below here are used during loading.
    /// Index of assets in the assets directory.
    #[new(default)]
    asset_index: Option<AssetIndex>,
    /// Assets that have not finished loading.
    #[new(default)]
    assets_in_progress: HashMap<AssetRecord, MapAssetHandles>,
    /// Assets that been loaded, but the prefabs have not.
    #[new(default)]
    maps_in_progress: HashMap<AssetRecord, Handle<Map>>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct MapAssetLoadingSystemData<'s> {
    /// `Loader` to load assets.
    #[derivative(Debug = "ignore")]
    loader: ReadExpect<'s, Loader>,
    /// `MapDefinition` assets.
    #[derivative(Debug = "ignore")]
    map_definition_assets: Read<'s, AssetStorage<MapDefinition>>,
    /// `SpritesDefinition` assets.
    #[derivative(Debug = "ignore")]
    sprites_definition_assets: Read<'s, AssetStorage<SpritesDefinition>>,
    /// `ComponentSequences` assets.
    #[derivative(Debug = "ignore")]
    component_sequences_assets: Read<'s, AssetStorage<ComponentSequences>>,
    /// `WaitSequence` assets.
    #[derivative(Debug = "ignore")]
    wait_sequence_assets: Read<'s, AssetStorage<WaitSequence>>,
    /// `SpriteRenderSequence` assets.
    #[derivative(Debug = "ignore")]
    sprite_render_sequence_assets: Read<'s, AssetStorage<SpriteRenderSequence>>,
    /// `Texture` assets.
    #[derivative(Debug = "ignore")]
    texture_assets: Read<'s, AssetStorage<Texture>>,
    /// `SpriteSheet` assets.
    #[derivative(Debug = "ignore")]
    sprite_sheet_assets: Read<'s, AssetStorage<SpriteSheet>>,
    /// `Map` assets.
    #[derivative(Debug = "ignore")]
    map_assets: Read<'s, AssetStorage<Map>>,
    /// `MapAssets` resource.
    #[derivative(Debug = "ignore")]
    loaded_maps: Write<'s, MapAssets>,
    /// `MapLoadingStatus` resource.
    #[derivative(Debug = "ignore")]
    loading_status: Write<'s, MapLoadingStatus>,
}

impl<'s> System<'s> for MapAssetLoadingSystem {
    type SystemData = MapAssetLoadingSystemData<'s>;

    fn run(
        &mut self,
        MapAssetLoadingSystemData {
            loader,
            map_definition_assets,
            sprites_definition_assets,
            component_sequences_assets,
            wait_sequence_assets,
            sprite_render_sequence_assets,
            texture_assets,
            sprite_sheet_assets,
            map_assets,
            mut loaded_maps,
            mut loading_status,
        }: Self::SystemData,
    ) {
        // TODO: Do a diff between existing index and directory based on a file watch / notify.
        // TODO: See <https://github.com/polachok/derive-diff>
        if self.asset_index.is_none() {
            let asset_index = AssetDiscovery::asset_index(&self.assets_dir);
            debug!("Indexed assets: {:?}", &self.asset_index);

            // Borrow self piecewise.
            let assets_in_progress = &self.assets_in_progress;
            let maps_in_progress = &self.maps_in_progress;
            let progress_counter = &mut self.progress_counter;

            let asset_records = asset_index.maps.clone();

            let new_asset_records = asset_records.into_iter().filter(|asset_record| {
                !(assets_in_progress.contains_key(asset_record)
                    || maps_in_progress.contains_key(asset_record)
                    || loaded_maps.contains_key(&asset_record.asset_slug))
            });

            let new_map_asset_handles = new_asset_records
                .map(|asset_record| {
                    let map_asset_handles = Self::asset_record_to_handles(
                        progress_counter,
                        &loader,
                        &map_definition_assets,
                        &sprites_definition_assets,
                        &asset_record,
                    );
                    (asset_record, map_asset_handles)
                })
                // Need to collect the output, otherwise `self` is still borrowed when
                // we wish to extend `assets_in_progress`.
                .collect::<Vec<_>>();

            self.assets_in_progress.extend(new_map_asset_handles);
            self.asset_index = Some(asset_index);
        }

        // Check if any of `assets_in_progress` have completed loading, and move them to
        // `maps_in_progress`
        //
        // TODO: `HashMap` needs a `drain_filter` implementation. Check the following issue:
        // TODO: <https://github.com/rust-lang/rust/issues/43244>
        let new_map = HashMap::with_capacity(self.assets_in_progress.len());
        let mut assets_in_progress = mem::replace(&mut self.assets_in_progress, new_map);
        assets_in_progress
            .drain()
            .for_each(|(asset_record, map_asset_handles)| {
                let map_definition_opt =
                    map_definition_assets.get(&map_asset_handles.map_definition_handle);
                let sprites_definition_opt = map_asset_handles
                    .sprites_definition_handle
                    .as_ref()
                    .map(|sprites_definition_handle| {
                        sprites_definition_assets.get(sprites_definition_handle)
                    });

                match (map_definition_opt, sprites_definition_opt) {
                    // If there is no sprites definition.
                    (Some(map_definition), None) => {
                        let margins = Margins::from(map_definition.header.bounds);
                        let map = Map::new(
                            // TODO: Maybe hold onto the handle, not the definition.
                            map_definition.clone(),
                            margins,
                            Vec::new(),
                            Vec::new(),
                            Vec::new(),
                            Vec::new(),
                        );

                        let map_handle =
                            loader.load_from_data(map, &mut self.progress_counter, &map_assets);

                        self.maps_in_progress.insert(asset_record, map_handle);
                    }
                    // If there is a sprites definition, and it is loaded.
                    (Some(map_definition), Some(Some(sprites_definition))) => {
                        let sprite_sheet_handles = SpriteLoader::load(
                            &mut self.progress_counter,
                            &loader,
                            &texture_assets,
                            &sprite_sheet_assets,
                            &sprites_definition,
                            &asset_record.path,
                        )
                        .expect("Failed to load textures and sprite sheets.");
                        let sequence_handles = (
                            Vec::<ComponentSequencesHandle>::with_capacity(
                                map_definition.layers.len(),
                            ),
                            Vec::<WaitSequenceHandle>::with_capacity(map_definition.layers.len()),
                            Vec::<SpriteRenderSequenceHandle>::with_capacity(
                                map_definition.layers.len(),
                            ),
                        );
                        let (
                            component_sequences_handles,
                            wait_sequence_handles,
                            sprite_render_sequence_handles,
                        ) = map_definition.layers.iter().fold(
                            sequence_handles,
                            |(
                                mut component_sequences_handles,
                                mut wait_sequence_handles,
                                mut sprite_render_sequence_handles,
                            ),
                             layer| {
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

                                let mut component_sequences = Vec::new();
                                component_sequences
                                    .push(ComponentSequence::Wait(wait_sequence.clone()));
                                component_sequences.push(ComponentSequence::SpriteRender(
                                    sprite_render_sequence.clone(),
                                ));
                                let component_sequences =
                                    ComponentSequences::new(component_sequences);
                                let component_sequences_handle = loader.load_from_data(
                                    component_sequences,
                                    (),
                                    &component_sequences_assets,
                                );

                                let wait_sequence_handle =
                                    loader.load_from_data(wait_sequence, (), &wait_sequence_assets);
                                let sprite_render_sequence_handle = loader.load_from_data(
                                    sprite_render_sequence,
                                    (),
                                    &sprite_render_sequence_assets,
                                );

                                component_sequences_handles.push(component_sequences_handle);
                                wait_sequence_handles.push(wait_sequence_handle);
                                sprite_render_sequence_handles.push(sprite_render_sequence_handle);

                                (
                                    component_sequences_handles,
                                    wait_sequence_handles,
                                    sprite_render_sequence_handles,
                                )
                            },
                        );

                        let margins = Margins::from(map_definition.header.bounds);
                        let map = Map::new(
                            // TODO: Maybe hold onto the handle, not the definition.
                            map_definition.clone(),
                            margins,
                            sprite_sheet_handles,
                            component_sequences_handles,
                            wait_sequence_handles,
                            sprite_render_sequence_handles,
                        );

                        let map_handle =
                            loader.load_from_data(map, &mut self.progress_counter, &map_assets);

                        self.maps_in_progress.insert(asset_record, map_handle);
                    }
                    // Either map definition or sprites definition is not loaded, or both.
                    _ => {
                        self.assets_in_progress
                            .insert(asset_record, map_asset_handles);
                    }
                }
            });

        // Check if any of `maps_in_progress` have completed loading, and move them to
        // `loaded_maps`
        //
        // TODO: Split into separate System, since this borrows `MapAssets` mutably.
        //
        // TODO: `HashMap` needs a `drain_filter` implementation. Check the following issue:
        // TODO: <https://github.com/rust-lang/rust/issues/43244>
        let new_map = HashMap::with_capacity(self.maps_in_progress.len());
        let mut maps_in_progress = mem::replace(&mut self.maps_in_progress, new_map);
        maps_in_progress
            .drain()
            .for_each(|(asset_record, prefab_handle)| {
                if map_assets.get(&prefab_handle).is_some() {
                    loaded_maps.insert(asset_record.asset_slug, prefab_handle);
                } else {
                    self.maps_in_progress.insert(asset_record, prefab_handle);
                }
            });

        *loading_status = if self.progress_counter.is_complete() {
            MapLoadingStatus::Complete
        } else {
            MapLoadingStatus::InProgress
        };
        debug!("Map loading status: {:?}", *loading_status);

        debug!(
            "Loading progress: {}/{}",
            self.progress_counter.num_finished(),
            self.progress_counter.num_assets()
        );
    }
}

impl MapAssetLoadingSystem {
    /// Initiates the asset loading for an object, and returns the handles.
    ///
    /// # Parameters
    ///
    /// * `progress_counter`: `ProgressCounter` to track loading progress.
    /// * `loader`: `Loader` to load assets.
    /// * `map_definition_assets`: `AssetStorage` of the `MapDefinition`.
    /// * `sprites_definition_assets`: `AssetStorage` of the `SpritesDefinition`.
    /// * `asset_record`: The asset record of the object.
    fn asset_record_to_handles(
        progress_counter: &mut ProgressCounter,
        loader: &Loader,
        map_definition_assets: &AssetStorage<MapDefinition>,
        sprites_definition_assets: &AssetStorage<SpritesDefinition>,
        asset_record: &AssetRecord,
    ) -> MapAssetHandles {
        debug!(
            "Loading `{}` from: `{}`",
            asset_record.asset_slug,
            asset_record.path.display()
        );

        let map_definition_handle = loader.load(
            asset_record
                .path
                .join("map.toml")
                .to_str()
                .expect("Expected path to be valid unicode."),
            TomlFormat,
            &mut *progress_counter,
            map_definition_assets,
        );
        let sprites_definition_handle = {
            let sprites_definition_path = asset_record.path.join("sprites.toml");

            if sprites_definition_path.exists() {
                let handle = loader.load(
                    sprites_definition_path
                        .to_str()
                        .expect("Expected path to be valid unicode."),
                    TomlFormat,
                    &mut *progress_counter,
                    sprites_definition_assets,
                );
                Some(handle)
            } else {
                None
            }
        };

        MapAssetHandles::new(map_definition_handle, sprites_definition_handle)
    }
}
