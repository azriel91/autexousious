use amethyst::{
    ecs::{Entities, Entity, Read, System, World, Write, WriteStorage},
    shred::{ResourceId, SystemData},
};
use character_prefab::CharacterPrefabHandle;
use character_selection_model::CharacterSelections;
use derivative::Derivative;
use derive_new::new;
use game_input::InputControlled;
use game_model::{loaded::CharacterPrefabs, play::GameEntities};
use object_type::ObjectType;
use team_model::play::{IndependentCounter, Team};
use typename_derive::TypeName;

use crate::{CharacterAugmentStatus, GameLoadingStatus};

/// Spawns character entities based on the character selection.
#[derive(Debug, Default, TypeName, new)]
pub(crate) struct CharacterSelectionSpawningSystem;

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct CharacterSelectionSpawningSystemData<'s> {
    /// `Entities` resource.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// `CharacterSelections` resource.
    #[derivative(Debug = "ignore")]
    pub character_selections: Read<'s, CharacterSelections>,
    /// `CharacterPrefabs` resource.
    #[derivative(Debug = "ignore")]
    pub character_prefabs: Read<'s, CharacterPrefabs>,
    /// `GameLoadingStatus` resource.
    #[derivative(Debug = "ignore")]
    pub game_loading_status: Write<'s, GameLoadingStatus>,
    /// `IndependentCounter` resource.
    #[derivative(Debug = "ignore")]
    pub independent_counter: Write<'s, IndependentCounter>,
    /// `CharacterPrefabHandle` components.
    #[derivative(Debug = "ignore")]
    pub character_prefab_handles: WriteStorage<'s, CharacterPrefabHandle>,
    /// `InputControlled` components.
    #[derivative(Debug = "ignore")]
    pub input_controlleds: WriteStorage<'s, InputControlled>,
    /// `Team` components.
    #[derivative(Debug = "ignore")]
    pub teams: WriteStorage<'s, Team>,
    /// `GameEntities` resource.
    #[derivative(Debug = "ignore")]
    pub game_entities: Write<'s, GameEntities>,
}

impl<'s> System<'s> for CharacterSelectionSpawningSystem {
    type SystemData = CharacterSelectionSpawningSystemData<'s>;

    fn run(
        &mut self,
        CharacterSelectionSpawningSystemData {
            entities,
            character_selections,
            character_prefabs,
            mut game_loading_status,
            mut independent_counter,
            mut character_prefab_handles,
            mut input_controlleds,
            mut teams,
            mut game_entities,
        }: Self::SystemData,
    ) {
        if game_loading_status.character_augment_status != CharacterAugmentStatus::Prefab {
            return;
        }

        let character_entities = character_selections
            .selections
            .iter()
            .map(|(controller_id, asset_slug)| {
                let entity = entities.create();

                let handle = character_prefabs
                    .get(asset_slug)
                    .unwrap_or_else(|| {
                        panic!(
                            "Expected prefab handle to exist for asset slug: `{}`",
                            asset_slug
                        )
                    })
                    .clone();

                input_controlleds
                    .insert(entity, InputControlled::new(*controller_id))
                    .expect("Failed to insert `InputControlled` for character.");
                character_prefab_handles
                    .insert(entity, handle)
                    .expect("Failed to insert `CharacterPrefabHandle` for character.");
                teams
                    .insert(
                        entity,
                        Team::Independent(independent_counter.get_and_increment()),
                    )
                    .expect("Failed to insert `Team` for character.");

                entity
            })
            .collect::<Vec<Entity>>();

        game_entities
            .objects
            .insert(ObjectType::Character, character_entities);

        game_loading_status.character_augment_status = CharacterAugmentStatus::Rectify;
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use amethyst::{
        assets::Processor,
        audio::Source,
        core::TransformBundle,
        ecs::{Join, ReadStorage, System, World, WorldExt},
        renderer::{types::DefaultBackend, RenderEmptyBundle},
        shred::SystemData,
        window::ScreenDimensions,
        Error,
    };
    use amethyst_test::{AmethystApplication, PopState, HIDPI, SCREEN_HEIGHT, SCREEN_WIDTH};
    use application_event::{AppEvent, AppEventReader};
    use assets_test::{ASSETS_PATH, CHAR_BAT_SLUG};
    use character_loading::{CharacterLoadingBundle, CHARACTER_PROCESSOR};
    use character_prefab::{CharacterPrefabBundle, CharacterPrefabHandle};
    use character_selection_model::CharacterSelections;
    use collision_audio_loading::CollisionAudioLoadingBundle;
    use collision_loading::CollisionLoadingBundle;
    use game_input::InputControlled;
    use game_input_model::ControlBindings;
    use game_model::play::GameEntities;
    use loading::{LoadingBundle, LoadingState};
    use map_loading::MapLoadingBundle;
    use object_type::ObjectType;
    use sequence_loading::SequenceLoadingBundle;
    use spawn_loading::SpawnLoadingBundle;
    use sprite_loading::SpriteLoadingBundle;
    use team_model::play::{IndependentCounter, Team};
    use typename::TypeName;
    use ui_audio_loading::UiAudioLoadingBundle;

    use super::CharacterSelectionSpawningSystem;
    use crate::{CharacterAugmentStatus, GameLoadingStatus};

    #[test]
    fn returns_if_augment_status_is_not_prefab() -> Result<(), Error> {
        run_test(
            |world| {
                let mut game_loading_status = GameLoadingStatus::new();
                game_loading_status.character_augment_status = CharacterAugmentStatus::Rectify;
                world.insert(game_loading_status);

                let mut character_selections = CharacterSelections::default();
                character_selections
                    .selections
                    .insert(0, CHAR_BAT_SLUG.clone());
                world.insert(character_selections);
            },
            |world| {
                let (input_controlleds, character_prefab_handles, teams) =
                    world.system_data::<TestSystemData<'_>>();
                assert_eq!(0, input_controlleds.count());
                assert_eq!(0, character_prefab_handles.count());
                assert_eq!(0, teams.count());
            },
        )
    }

    #[test]
    fn spawns_characters_when_they_havent_been_spawned() -> Result<(), Error> {
        run_test(
            |world| {
                let mut game_loading_status = GameLoadingStatus::new();
                game_loading_status.character_augment_status = CharacterAugmentStatus::Prefab;
                world.insert(game_loading_status);

                let mut character_selections = CharacterSelections::default();
                character_selections
                    .selections
                    .insert(0, CHAR_BAT_SLUG.clone());
                character_selections
                    .selections
                    .insert(123, CHAR_BAT_SLUG.clone());
                world.insert(character_selections);
            },
            |world| {
                let (input_controlleds, character_prefab_handles, teams) =
                    world.system_data::<TestSystemData<'_>>();
                let components = (&input_controlleds, &character_prefab_handles, &teams)
                    .join()
                    .collect::<Vec<_>>();

                // Need to use `find()` because the joins may be presented out of order.
                assert_eq!(2, components.len());
                assert!(
                    components
                        .iter()
                        .find(|(&input_controlled, _character_prefab_handle, &_team)| {
                            input_controlled == InputControlled::new(0)
                        })
                        .is_some(),
                    "Expected entity with `InputControlled`, `CharacterPrefabHandle`, and \
                     `Team` components to exist. Components: {:?}",
                    components
                );
                assert!(
                    components
                        .iter()
                        .find(|(&_input_controlled, _character_prefab_handle, &team)| {
                            team == Team::Independent(IndependentCounter::new(0))
                        })
                        .is_some(),
                    "Expected entity with `InputControlled`, `CharacterPrefabHandle`, and \
                     `Team` components to exist. Components: {:?}",
                    components
                );
                assert!(
                    components
                        .iter()
                        .find(|(&input_controlled, _character_prefab_handle, &_team)| {
                            input_controlled == InputControlled::new(123)
                        })
                        .is_some(),
                    "Expected entity with `InputControlled`, `CharacterPrefabHandle`, and \
                     `Team` components to exist. Components: {:?}",
                    components
                );
                assert!(
                    components
                        .iter()
                        .find(|(&_input_controlled, _character_prefab_handle, &team)| {
                            team == Team::Independent(IndependentCounter::new(1))
                        })
                        .is_some(),
                    "Expected entity with `InputControlled`, `CharacterPrefabHandle`, and \
                     `Team` components to exist. Components: {:?}",
                    components
                );

                assert_eq!(
                    2,
                    world
                        .read_resource::<GameEntities>()
                        .objects
                        .get(&ObjectType::Character)
                        .expect("Expected `ObjectType::Character` key in `GameEntities`.")
                        .len()
                );
                assert_eq!(
                    CharacterAugmentStatus::Rectify,
                    world
                        .read_resource::<GameLoadingStatus>()
                        .character_augment_status
                );
            },
        )
    }

    fn run_test(setup_fn: fn(&mut World), assertion_fn: fn(&mut World)) -> Result<(), Error> {
        env::set_var("APP_DIR", env!("CARGO_MANIFEST_DIR"));

        AmethystApplication::blank()
            .with_custom_event_type::<AppEvent, AppEventReader>()
            .with_bundle(TransformBundle::new())
            .with_bundle(RenderEmptyBundle::<DefaultBackend>::new())
            .with_resource(ScreenDimensions::new(SCREEN_WIDTH, SCREEN_HEIGHT, HIDPI))
            .with_ui_bundles::<ControlBindings>()
            .with_system(Processor::<Source>::new(), "source_processor", &[])
            .with_bundle(SpriteLoadingBundle::new())
            .with_bundle(SequenceLoadingBundle::new())
            .with_bundle(LoadingBundle::new(ASSETS_PATH.clone()))
            .with_bundle(CollisionLoadingBundle::new())
            .with_bundle(SpawnLoadingBundle::new())
            .with_bundle(MapLoadingBundle::new())
            .with_bundle(CharacterLoadingBundle::new())
            .with_bundle(
                CharacterPrefabBundle::new()
                    .with_system_dependencies(&[String::from(CHARACTER_PROCESSOR)]),
            )
            .with_bundle(CollisionAudioLoadingBundle::new(ASSETS_PATH.clone()))
            .with_bundle(UiAudioLoadingBundle::new(ASSETS_PATH.clone()))
            .with_state(|| LoadingState::new(PopState))
            .with_effect(|world| {
                <CharacterSelectionSpawningSystem as System>::SystemData::setup(world)
            })
            .with_effect(setup_fn)
            .with_system_single(
                CharacterSelectionSpawningSystem,
                CharacterSelectionSpawningSystem::type_name(),
                &[],
            ) // kcov-ignore
            .with_assertion(assertion_fn)
            .run_isolated()
    }

    type TestSystemData<'s> = (
        ReadStorage<'s, InputControlled>,
        ReadStorage<'s, CharacterPrefabHandle>,
        ReadStorage<'s, Team>,
    );
}
