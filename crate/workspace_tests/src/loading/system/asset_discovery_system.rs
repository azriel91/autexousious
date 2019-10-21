#[cfg(test)]
mod tests {

    use amethyst::Error;
    use amethyst_test::AmethystApplication;
    use asset_model::config::AssetType;
    use assets_test::{
        ASSETS_PATH, CHAR_BAT_PATH, CHAR_BAT_SLUG, ENERGY_SQUARE_PATH, ENERGY_SQUARE_SLUG,
        MAP_FADE_PATH, MAP_FADE_SLUG,
    };
    use loading_model::loaded::LoadStage;
    use object_type::ObjectType;

    use loading::{AssetDiscoverySystem, AssetDiscoverySystemData};

    #[test]
    fn inserts_metadata_of_indexed_assets() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_system(AssetDiscoverySystem::new(ASSETS_PATH.clone()), "", &[])
            .with_assertion(move |world| {
                let AssetDiscoverySystemData {
                    asset_index,
                    asset_id_mappings,
                    asset_type_mappings,
                    asset_load_stage,
                    asset_id_to_path,
                } = world.system_data::<AssetDiscoverySystemData<'_>>();

                assert!(asset_index.is_some());

                [
                    (
                        CHAR_BAT_SLUG.clone(),
                        CHAR_BAT_PATH.clone(),
                        AssetType::Object(ObjectType::Character),
                    ),
                    (
                        ENERGY_SQUARE_SLUG.clone(),
                        ENERGY_SQUARE_PATH.clone(),
                        AssetType::Object(ObjectType::Energy),
                    ),
                    (MAP_FADE_SLUG.clone(), MAP_FADE_PATH.clone(), AssetType::Map),
                ]
                .iter()
                .for_each(|(asset_slug, asset_path, asset_type)| {
                    let asset_id = asset_id_mappings.id(asset_slug).copied();

                    assert!(asset_id.is_some());

                    let asset_id = asset_id.expect("Expected `AssetId` to exist.");
                    assert_eq!(Some(asset_type), asset_type_mappings.get(asset_id));
                    assert_eq!(
                        Some(LoadStage::New),
                        asset_load_stage.get(asset_id).copied()
                    );
                    assert_eq!(Some(asset_path), asset_id_to_path.get(asset_id));
                })
            })
            .run()
    }
}
