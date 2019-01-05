use amethyst::ecs::prelude::*;
use game_input::{ControllerInput, InputControlled};
use game_model::config::AssetSlug;
use log::debug;
use object_model::entity::{Grounding, HealthPoints, RunCounter};

use crate::CharacterComponentStorages;

/// Spawns character entities into the world.
#[derive(Debug)]
pub struct CharacterEntityAugmenter;

impl CharacterEntityAugmenter {
    /// Spawns a player controlled character entity.
    ///
    /// # Parameters
    ///
    /// * `object_spawning_resources`: Resources to construct the character with.
    /// * `character_component_storages`: Character specific `Component` storages.
    /// * `object_component_storages`: Common object `Component` storages.
    /// * `position`: Position of the entity in game.
    /// * `velocity`: Velocity of the entity in game.
    /// * `slug_and_handle`: Slug and handle of the character to spawn.
    /// * `input_controlled`: `Component` that links the character entity to the controller.
    pub fn augment<'res, 's>(
        entity: Entity,
        CharacterComponentStorages {
            ref mut input_controlleds,
            ref mut controller_inputs,
            ref mut health_pointses,
            ref mut run_counters,
            ref mut groundings,
        }: &mut CharacterComponentStorages<'s>,
        slug: &AssetSlug,
        input_controlled: InputControlled,
    ) {
        debug!("Augmenting `{}`", slug);

        // Controller of this entity
        input_controlleds
            .insert(entity, input_controlled)
            .expect("Failed to insert input_controlled component.");
        // Controller of this entity
        controller_inputs
            .insert(entity, ControllerInput::default())
            .expect("Failed to insert controller_input component.");
        // Health points.
        health_pointses
            .insert(entity, HealthPoints::default())
            .expect("Failed to insert health_points component.");
        // Run counter.
        run_counters
            .insert(entity, RunCounter::default())
            .expect("Failed to insert run_counter component.");
        // Grounding.
        groundings
            .insert(entity, Grounding::default())
            .expect("Failed to insert grounding component.");
    }
}

#[cfg(test)]
mod test {
    use std::env;

    use amethyst::{animation::AnimationBundle, assets::AssetStorage, ecs::prelude::*};
    use amethyst_test::prelude::*;
    use application_event::{AppEvent, AppEventReader};
    use assets_test::{ASSETS_CHAR_BAT_SLUG, ASSETS_PATH};
    use character_model::{config::CharacterSequenceId, loaded::Character};
    use collision_loading::CollisionLoadingBundle;
    use collision_model::animation::{BodyFrameActiveHandle, InteractionFrameActiveHandle};
    use game_input::{ControllerInput, InputControlled};
    use loading::LoadingState;
    use map_loading::MapLoadingBundle;
    use map_model::loaded::Map;
    use object_loading::ObjectLoadingBundle;
    use object_model::entity::{Grounding, HealthPoints, RunCounter};
    use typename::TypeName as TypeNameTrait;
    use typename_derive::TypeName;

    use super::CharacterEntityAugmenter;
    use crate::{
        CharacterComponentStorages, ObjectAnimationStorages, ObjectComponentStorages,
        ObjectSpawningResources,
    };

    #[test]
    fn augments_entity_with_character_components() {
        env::set_var("APP_DIR", env!("CARGO_MANIFEST_DIR"));

        let assertion = |world: &mut World| {
            let controller_id = 0;
            let input_controlled = InputControlled::new(controller_id);

            let entity = world.create_entity().build();
            {
                let mut character_component_storages =
                    CharacterComponentStorages::fetch(&world.res);
                CharacterEntityAugmenter::augment(
                    entity,
                    &mut character_component_storages,
                    &ASSETS_CHAR_BAT_SLUG,
                    input_controlled,
                );
            }

            assert!(world.read_storage::<InputControlled>().contains(entity));
            assert!(world.read_storage::<ControllerInput>().contains(entity));
            assert!(world.read_storage::<HealthPoints>().contains(entity));
            assert!(world.read_storage::<RunCounter>().contains(entity));
            assert!(world.read_storage::<Grounding>().contains(entity));
        };

        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::render_base("augments_entity_with_character_components", false)
                .with_custom_event_type::<AppEvent, AppEventReader>()
                .with_bundle(
                    AnimationBundle::<CharacterSequenceId, BodyFrameActiveHandle>::new(
                        "character_body_frame_acs",
                        "character_body_frame_sis",
                    )
                )
                .with_bundle(AnimationBundle::<
                    CharacterSequenceId,
                    InteractionFrameActiveHandle,
                >::new(
                    "character_interaction_acs", "character_interaction_sis",
                ))
                .with_bundle(CollisionLoadingBundle::new())
                .with_bundle(MapLoadingBundle::new())
                .with_bundle(ObjectLoadingBundle::new())
                .with_system(TestSystem, TestSystem::type_name(), &[])
                .with_state(|| LoadingState::new(ASSETS_PATH.clone(), PopState))
                .with_assertion(assertion)
                .run()
                .is_ok()
        );
    }

    // Sets up storages for the various `Component`.
    #[derive(Debug, TypeName)]
    struct TestSystem;
    type TestSystemData<'s> = (
        CharacterComponentStorages<'s>,
        ObjectAnimationStorages<'s, CharacterSequenceId>,
        ObjectComponentStorages<'s, CharacterSequenceId>,
        ObjectSpawningResources<'s, Character>,
        Read<'s, AssetStorage<Map>>,
    );
    impl<'s> System<'s> for TestSystem {
        type SystemData = TestSystemData<'s>;
        fn run(&mut self, _: Self::SystemData) {}
    }
}
