#[cfg(test)]
mod test {
    use amethyst::{
        assets::{AssetStorage, ProgressCounter},
        core::TransformBundle,
        ecs::{Read, SystemData, WorldExt},
        renderer::{types::DefaultBackend, RenderEmptyBundle, SpriteSheet, Texture},
        Error,
    };
    use amethyst_test::AmethystApplication;
    use application::{AppFile, Format};
    use asset_model::config::AssetRecord;
    use assets_test::{CHAR_BAT_PATH, CHAR_BAT_SLUG};
    use character_model::config::{CharacterDefinition, CharacterSequence};
    use collision_loading::CollisionLoadingBundle;
    use object_model::loaded::Object;
    use sequence_loading::SequenceLoadingBundle;
    use spawn_loading::SpawnLoadingBundle;
    use sprite_loading::SpriteLoader;
    use sprite_model::config::SpritesDefinition;

    use object_loading::{ObjectLoader, ObjectLoaderParams, ObjectLoaderSystemData};

    #[test]
    fn loads_object_assets() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_bundle(TransformBundle::new())
            .with_bundle_event_fn(|event_loop| RenderEmptyBundle::<DefaultBackend>::new(event_loop))
            .with_bundle(CollisionLoadingBundle::new())
            .with_bundle(SpawnLoadingBundle::new())
            .with_bundle(SequenceLoadingBundle::new())
            .with_setup(|world| TestSystemData::setup(world))
            .with_effect(|world| {
                let asset_record = AssetRecord::new(CHAR_BAT_SLUG.clone(), CHAR_BAT_PATH.clone());

                let character_definition = AppFile::load_in::<CharacterDefinition, _>(
                    &asset_record.path,
                    "object.yaml",
                    Format::Yaml,
                )
                .expect("Failed to load object.yaml into CharacterDefinition");

                let object = {
                    let sprites_definition = AppFile::load_in::<SpritesDefinition, _>(
                        &asset_record.path,
                        "sprites.yaml",
                        Format::Yaml,
                    )
                    .expect("Failed to load sprites_definition.");

                    let (object_loader_system_data, texture_assets, sprite_sheet_assets) =
                        world.system_data::<TestSystemData>();

                    // TODO: <https://gitlab.com/azriel91/autexousious/issues/94>
                    let sprite_sheet_handles = SpriteLoader::load(
                        &mut ProgressCounter::default(),
                        &object_loader_system_data.loader,
                        &texture_assets,
                        &sprite_sheet_assets,
                        &sprites_definition,
                        &asset_record.path,
                    )
                    .expect("Failed to load sprites.");
                    let sprite_sheet_handles = &sprite_sheet_handles;

                    ObjectLoader::load::<CharacterSequence>(
                        ObjectLoaderParams::from((
                            &object_loader_system_data,
                            sprite_sheet_handles.as_slice(),
                        )),
                        &character_definition.object_definition,
                    )
                };

                world.insert(object);
            })
            .with_assertion(|world| {
                let object = world.read_resource::<Object>();

                macro_rules! assert_frame_component_data_count {
                    ($frame_component_data_field:ident) => {
                        assert_eq!(
                            28,
                            object.$frame_component_data_field.len(),
                            concat!(
                                "Expected 28 ",
                                stringify!($frame_component_data_field),
                                " to be loaded.",
                                "Check `bat/object.yaml` for number of sequences."
                            )
                        );
                    };
                }

                assert_frame_component_data_count!(wait_sequence_handles);
                assert_frame_component_data_count!(sprite_render_sequence_handles);
                assert_frame_component_data_count!(body_sequence_handles);
                assert_frame_component_data_count!(interactions_sequence_handles);
            })
            .run_isolated()
    }

    type TestSystemData<'s> = (
        ObjectLoaderSystemData<'s>,
        Read<'s, AssetStorage<Texture>>,
        Read<'s, AssetStorage<SpriteSheet>>,
    );
}
