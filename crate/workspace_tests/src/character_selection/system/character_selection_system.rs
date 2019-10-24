#[cfg(test)]
mod tests {
    use amethyst::{
        assets::Processor,
        audio::Source,
        core::TransformBundle,
        ecs::{World, WorldExt},
        renderer::{types::DefaultBackend, RenderEmptyBundle},
        shrev::EventChannel,
        window::ScreenDimensions,
        Error,
    };
    use amethyst_test::{AmethystApplication, PopState, HIDPI, SCREEN_HEIGHT, SCREEN_WIDTH};
    use application_event::{AppEvent, AppEventReader};
    use asset_model::{
        config::AssetType,
        loaded::{AssetId, AssetTypeMappings},
    };
    use assets_test::ASSETS_PATH;
    use audio_loading::AudioLoadingBundle;
    use background_loading::BackgroundLoadingBundle;
    use character_loading::CharacterLoadingBundle;
    use character_selection_model::{
        CharacterSelection, CharacterSelectionEvent, CharacterSelections,
    };
    use collision_audio_loading::CollisionAudioLoadingBundle;
    use collision_loading::CollisionLoadingBundle;
    use energy_loading::EnergyLoadingBundle;
    use game_input_model::ControlBindings;
    use kinematic_loading::KinematicLoadingBundle;
    use loading::{LoadingBundle, LoadingState};
    use map_loading::MapLoadingBundle;
    use object_type::ObjectType;
    use sequence_loading::SequenceLoadingBundle;
    use spawn_loading::SpawnLoadingBundle;
    use sprite_loading::SpriteLoadingBundle;
    use typename::TypeName;
    use ui_audio_loading::UiAudioLoadingBundle;

    use character_selection::CharacterSelectionSystem;

    #[test]
    fn inserts_character_selection_on_select_event() -> Result<(), Error> {
        run_test(
            SetupParams {
                character_selection_event_fn: |asset_id| CharacterSelectionEvent::Select {
                    controller_id: 123,
                    character_selection: CharacterSelection::Id(asset_id),
                },
            },
            ExpectedParams {
                character_selection_fn: |asset_id| Some(asset_id),
            },
        )
    }

    #[test]
    fn removes_character_selection_on_deselect_event() -> Result<(), Error> {
        run_test(
            SetupParams {
                character_selection_event_fn: |_| CharacterSelectionEvent::Deselect {
                    controller_id: 123,
                },
            },
            ExpectedParams {
                character_selection_fn: |_| None,
            },
        )
    }

    fn run_test(
        SetupParams {
            character_selection_event_fn,
        }: SetupParams,
        ExpectedParams {
            character_selection_fn,
        }: ExpectedParams,
    ) -> Result<(), Error> {
        AmethystApplication::blank()
            .with_custom_event_type::<AppEvent, AppEventReader>()
            .with_resource(ScreenDimensions::new(SCREEN_WIDTH, SCREEN_HEIGHT, HIDPI))
            .with_bundle(TransformBundle::new())
            .with_bundle(RenderEmptyBundle::<DefaultBackend>::new())
            .with_ui_bundles::<ControlBindings>()
            .with_system(Processor::<Source>::new(), "source_processor", &[])
            .with_bundle(SpriteLoadingBundle::new())
            .with_bundle(SequenceLoadingBundle::new())
            .with_bundle(AudioLoadingBundle::new())
            .with_bundle(KinematicLoadingBundle::new())
            .with_bundle(LoadingBundle::new(ASSETS_PATH.clone()))
            .with_bundle(CollisionLoadingBundle::new())
            .with_bundle(SpawnLoadingBundle::new())
            .with_bundle(BackgroundLoadingBundle::new())
            .with_bundle(MapLoadingBundle::new())
            .with_bundle(CharacterLoadingBundle::new())
            .with_bundle(EnergyLoadingBundle::new())
            .with_bundle(CollisionAudioLoadingBundle::new(ASSETS_PATH.clone()))
            .with_bundle(UiAudioLoadingBundle::new(ASSETS_PATH.clone()))
            .with_state(|| LoadingState::new(PopState))
            .with_system(
                CharacterSelectionSystem::new(),
                CharacterSelectionSystem::type_name(),
                &[],
            ) // kcov-ignore
            .with_effect(move |world| {
                let asset_id = first_character_asset_id(world);

                let character_selection_event = character_selection_event_fn(asset_id);
                send_event(world, character_selection_event)
            })
            .with_assertion(move |world| {
                let asset_id = first_character_asset_id(world);
                let character_selection_expected = character_selection_fn(asset_id);

                let character_selections = world.read_resource::<CharacterSelections>();

                assert_eq!(
                    character_selection_expected,
                    character_selections.selections.get(&123).cloned()
                );
            })
            .run_isolated()
    }

    fn send_event(world: &mut World, event: CharacterSelectionEvent) {
        world
            .write_resource::<EventChannel<CharacterSelectionEvent>>()
            .single_write(event);
    }

    fn first_character_asset_id(world: &mut World) -> AssetId {
        let asset_type_mappings = world.read_resource::<AssetTypeMappings>();
        asset_type_mappings
            .iter_ids(&AssetType::Object(ObjectType::Character))
            .next()
            .copied()
            .expect("Expected at least one character to be loaded.")
    }

    struct SetupParams {
        character_selection_event_fn: fn(AssetId) -> CharacterSelectionEvent,
    }

    struct ExpectedParams {
        character_selection_fn: fn(AssetId) -> Option<AssetId>,
    }
}
