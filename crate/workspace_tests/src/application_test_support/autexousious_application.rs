#[cfg(test)]
mod test {
    use amethyst::{ecs::WorldExt, input::InputHandler, ui::Interactable, Error};
    use asset_model::{config::AssetType, loaded::AssetTypeMappings};
    use game_input_model::ControlBindings;
    use game_model::play::GameEntities;
    use object_type::ObjectType;
    use strum::IntoEnumIterator;

    use application_test_support::AutexousiousApplication;

    #[test]
    fn ui_base_uses_strong_types_for_input_and_ui_bundles() -> Result<(), Error> {
        AutexousiousApplication::ui_base()
            .with_assertion(|world| {
                // Panics if the type parameters used are not these ones.
                world.read_resource::<InputHandler<ControlBindings>>();
                world.read_storage::<Interactable>();
            })
            .run_isolated()
    }

    #[test]
    fn render_and_ui_uses_strong_types_for_input_and_ui_bundles() -> Result<(), Error> {
        AutexousiousApplication::render_and_ui()
            .with_assertion(|world| {
                // Panics if the type parameters used are not these ones.
                world.read_resource::<InputHandler<ControlBindings>>();
                world.read_storage::<Interactable>();
            })
            .run_isolated()
    }

    #[test]
    fn config_base_loads_assets_from_self_crate_directory() -> Result<(), Error> {
        AutexousiousApplication::config_base()
            .with_assertion(|world| {
                let asset_type_mappings = world.read_resource::<AssetTypeMappings>();
                assert!(asset_type_mappings.iter_ids(&AssetType::Map).count() > 0);
                assert!(
                    asset_type_mappings
                        .iter_ids(&AssetType::Object(ObjectType::Character))
                        .count()
                        > 0
                );
                assert!(
                    asset_type_mappings
                        .iter_ids(&AssetType::Object(ObjectType::Energy))
                        .count()
                        > 0
                );
            })
            .run_isolated()
    }

    #[test]
    fn game_base_loads_object_and_map_entities() -> Result<(), Error> {
        AutexousiousApplication::game_base()
            .with_assertion(|world| {
                let game_entities = &*world.read_resource::<GameEntities>();

                // Ensure there is at least one entity per object type.
                ObjectType::iter()
                    .filter(|object_type| *object_type != ObjectType::TestObject)
                    .filter(|object_type| *object_type != ObjectType::Energy)
                    .for_each(|object_type| {
                        let objects = game_entities.objects.get(&object_type);
                        let object_entities = objects.unwrap_or_else(|| {
                            // kcov-ignore-start
                            panic!("Expected entry for the `{}` object type.", object_type)
                            // kcov-ignore-end
                        });

                        assert!(
                            !object_entities.is_empty(),
                            // kcov-ignore-start
                            format!(
                                // kcov-ignore-end
                                "Expected at least one entity for the `{}` object type",
                                object_type
                            )
                        );
                    });

                // Ensure there is at least one map layer (map is loaded).
                assert!(
                    !game_entities.map_layers.is_empty(),
                    "Expected map to be loaded."
                );
            })
            .run_isolated()
    }
}
