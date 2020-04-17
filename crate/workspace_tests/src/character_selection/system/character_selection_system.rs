#[cfg(test)]
mod tests {
    use std::{any, str::FromStr};

    use amethyst::{
        core::TransformBundle,
        ecs::{World, WorldExt},
        shrev::EventChannel,
        window::ScreenDimensions,
        Error,
    };
    use amethyst_test::{AmethystApplication, HIDPI, SCREEN_HEIGHT, SCREEN_WIDTH};
    use application_event::{AppEvent, AppEventReader};
    use application_test_support::AssetQueries;
    use asset_model::{
        config::{AssetSlug, AssetType},
        loaded::{AssetId, AssetTypeMappings},
    };
    use asset_selection_model::play::{AssetSelection, AssetSelectionEvent};
    use character_selection_model::CharacterSelections;
    use game_input_model::config::ControlBindings;
    use object_type::ObjectType;

    use character_selection::CharacterSelectionSystem;

    #[test]
    fn inserts_character_selection_on_select_event() -> Result<(), Error> {
        run_test(
            SetupParams {
                with_character_selection_initial: false,
                asset_selection_event_fn: |asset_id| AssetSelectionEvent::Select {
                    entity: None,
                    controller_id: 123,
                    asset_selection: AssetSelection::Id(asset_id),
                },
            },
            ExpectedParams {
                character_selection_fn: |asset_id| Some(asset_id),
            },
        )
    }

    #[test]
    fn overwrites_character_selection_on_next_select_event() -> Result<(), Error> {
        run_test(
            SetupParams {
                with_character_selection_initial: true,
                asset_selection_event_fn: |asset_id| AssetSelectionEvent::Select {
                    entity: None,
                    controller_id: 123,
                    asset_selection: AssetSelection::Id(asset_id),
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
                with_character_selection_initial: false,
                asset_selection_event_fn: |_| AssetSelectionEvent::Deselect {
                    entity: None,
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
            with_character_selection_initial,
            asset_selection_event_fn,
        }: SetupParams,
        ExpectedParams {
            character_selection_fn,
        }: ExpectedParams,
    ) -> Result<(), Error> {
        AmethystApplication::blank()
            .with_custom_event_type::<AppEvent, AppEventReader>()
            .with_resource(ScreenDimensions::new(SCREEN_WIDTH, SCREEN_HEIGHT, HIDPI))
            .with_bundle(TransformBundle::new())
            .with_ui_bundles::<ControlBindings>()
            .with_system(
                CharacterSelectionSystem::new(),
                any::type_name::<CharacterSelectionSystem>(),
                &[],
            ) // kcov-ignore
            .with_effect(move |world| {
                let asset_slug_zero =
                    AssetSlug::from_str("test/zero").expect("Expected `AssetSlug` to be valid.");
                let asset_id_zero = AssetQueries::id_generate(world, asset_slug_zero);
                world.insert(asset_id_zero);

                if with_character_selection_initial {
                    let asset_slug =
                        AssetSlug::from_str("test/one").expect("Expected `AssetSlug` to be valid.");
                    let asset_id_one = AssetQueries::id_generate(world, asset_slug);

                    let controller_id = 123;

                    let mut character_selections = world.write_resource::<CharacterSelections>();

                    character_selections
                        .selections
                        .insert(controller_id, asset_id_one);
                }

                {
                    let mut asset_type_mappings = world.write_resource::<AssetTypeMappings>();
                    asset_type_mappings
                        .insert(asset_id_zero, AssetType::Object(ObjectType::Character));
                }

                let asset_selection_event = asset_selection_event_fn(asset_id_zero);
                send_event(world, asset_selection_event);
            })
            .with_assertion(move |world| {
                let asset_id = *world.read_resource::<AssetId>();
                let character_selection_expected = character_selection_fn(asset_id);

                let character_selections = world.read_resource::<CharacterSelections>();

                assert_eq!(
                    character_selection_expected,
                    character_selections.selections.get(&123).cloned()
                );
            })
            .run_winit_loop()
    }

    fn send_event(world: &mut World, event: AssetSelectionEvent) {
        world
            .write_resource::<EventChannel<AssetSelectionEvent>>()
            .single_write(event);
    }

    struct SetupParams {
        with_character_selection_initial: bool,
        asset_selection_event_fn: fn(AssetId) -> AssetSelectionEvent,
    }

    struct ExpectedParams {
        character_selection_fn: fn(AssetId) -> Option<AssetId>,
    }
}
