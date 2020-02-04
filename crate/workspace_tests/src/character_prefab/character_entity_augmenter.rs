#[cfg(test)]
mod test {
    use std::{iter::FromIterator, str::FromStr};

    use amethyst::{
        assets::{AssetStorage, Loader, Processor},
        ecs::{Builder, Read, ReadExpect, World, WorldExt, Write},
        shred::SystemData,
        Error,
    };
    use amethyst_test::AmethystApplication;
    use asset_model::{
        config::AssetSlug,
        loaded::{AssetId, AssetIdMappings},
    };
    use character_model::{
        config::{CharacterDefinition, CharacterSequenceName},
        loaded::AssetCharacterDefinitionHandle,
        play::RunCounter,
    };
    use charge_model::{
        config::{ChargeDelay, ChargeLimit, ChargeUseMode},
        play::{ChargeRetention, ChargeTrackerClock},
    };
    use game_input_model::play::ControllerInput;
    use map_model::play::MapBounded;
    use object_model::{config::Mass, play::HealthPoints};
    use object_status_model::config::StunPoints;
    use sequence_model::loaded::{AssetSequenceIdMappings, SequenceIdMappings};

    use character_prefab::{
        CharacterComponentStorages, CharacterEntityAugmenter, CharacterSpawningResources,
    };

    #[test]
    fn augments_entity_with_character_components() -> Result<(), Error> {
        let assertion = |world: &mut World| {
            let entity = world.create_entity().build();
            {
                let asset_id = *world.read_resource::<AssetId>();
                let (character_spawning_resources, mut character_component_storages) = world
                    .system_data::<(
                        CharacterSpawningResources<'_>,
                        CharacterComponentStorages<'_>,
                    )>();
                CharacterEntityAugmenter::augment(
                    &character_spawning_resources,
                    &mut character_component_storages,
                    asset_id,
                    entity,
                );
            }

            assert!(world.read_storage::<ControllerInput>().contains(entity));
            assert!(world.read_storage::<HealthPoints>().contains(entity));
            assert!(world.read_storage::<StunPoints>().contains(entity));
            assert!(world.read_storage::<RunCounter>().contains(entity));
            assert!(world.read_storage::<Mass>().contains(entity));
            assert!(world.read_storage::<MapBounded>().contains(entity));
            assert!(world.read_storage::<ChargeTrackerClock>().contains(entity));
            assert!(world.read_storage::<ChargeLimit>().contains(entity));
            assert!(world.read_storage::<ChargeDelay>().contains(entity));
            assert!(world.read_storage::<ChargeUseMode>().contains(entity));
            assert!(world.read_storage::<ChargeRetention>().contains(entity));
        };

        AmethystApplication::blank()
            .with_system(Processor::<CharacterDefinition>::new(), "", &[])
            .with_setup(|world| {
                <Read<'_, AssetIdMappings> as SystemData>::setup(world);
                <CharacterSpawningResources as SystemData>::setup(world);
                <CharacterComponentStorages as SystemData>::setup(world);
            })
            .with_effect(|world| {
                let asset_id = {
                    let mut asset_id_mappings = world.write_resource::<AssetIdMappings>();
                    let asset_slug =
                        AssetSlug::from_str("test/char").expect("Expected asset slug to be valid.");
                    asset_id_mappings.insert(asset_slug)
                };

                {
                    let (
                        loader,
                        mut asset_sequence_id_mappings_character,
                        mut asset_character_definition_handle,
                        character_definition_assets,
                    ) = world.system_data::<(
                        ReadExpect<'_, Loader>,
                        Write<'_, AssetSequenceIdMappings<CharacterSequenceName>>,
                        Write<'_, AssetCharacterDefinitionHandle>,
                        Read<'_, AssetStorage<CharacterDefinition>>,
                    )>();

                    let character_definition = CharacterDefinition::default();

                    let sequence_id_mappings = SequenceIdMappings::from_iter(
                        character_definition.object_definition.sequences.keys(),
                    );
                    asset_sequence_id_mappings_character.insert(asset_id, sequence_id_mappings);

                    let character_definition_handle = loader.load_from_data(
                        character_definition,
                        (),
                        &*character_definition_assets,
                    );
                    asset_character_definition_handle.insert(asset_id, character_definition_handle);
                }

                world.insert(asset_id);
            })
            .with_assertion(assertion)
            .run()
    }
}
