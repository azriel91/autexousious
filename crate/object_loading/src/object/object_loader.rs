use std::collections::HashMap;

use amethyst::{assets::Handle, renderer::SpriteRender, Error};
use collision_model::{
    config::{Body, Interactions},
    loaded::{BodySequence, InteractionsSequence},
};
use fnv::FnvHashMap;
use object_model::{
    config::{GameObjectFrame, GameObjectSequence, ObjectDefinition},
    loaded::{GameObject, Object, ObjectWrapper},
};
use sequence_model::{
    config::Wait,
    loaded::{
        ComponentSequence, ComponentSequences, ComponentSequencesHandle, SequenceEndTransition,
        WaitSequence,
    },
};
use serde::{Deserialize, Serialize};
use sprite_model::loaded::SpriteRenderSequence;

use crate::ObjectLoaderParams;

/// Loads assets specified by object configuration into the loaded object model.
#[derive(Debug)]
pub struct ObjectLoader;

impl ObjectLoader {
    /// Returns the loaded `Object` referenced by the asset record.
    ///
    /// # Parameters
    ///
    /// * `object_loader_params`: Entry of the object's configuration.
    /// * `object_definition`: Object definition configuration.
    pub fn load<O>(
        ObjectLoaderParams {
            loader,
            component_sequences_assets,
            sprite_sheet_handles,
            body_assets,
            interactions_assets,
        }: ObjectLoaderParams,
        object_definition: &ObjectDefinition<O::GameObjectSequence>,
    ) -> Result<O::ObjectWrapper, Error>
    where
        O: GameObject,
        <O as GameObject>::SequenceId: for<'de> Deserialize<'de> + Serialize,
    {
        let sequence_end_transitions = object_definition
            .sequences
            .iter()
            .map(|(sequence_id, sequence)| {
                (
                    *sequence_id,
                    SequenceEndTransition::new(sequence.object_sequence().next),
                )
            })
            .collect::<FnvHashMap<_, _>>();

        // Load component sequences
        let component_sequences_handles = object_definition
            .sequences
            .iter()
            .map(|(sequence_id, sequence)| {
                let wait_sequence = WaitSequence::new(
                    sequence
                        .object_sequence()
                        .frames
                        .iter()
                        .map(|frame| frame.object_frame().wait)
                        .collect::<Vec<Wait>>(),
                );
                let sprite_render_sequence = SpriteRenderSequence::new(
                    sequence
                        .object_sequence()
                        .frames
                        .iter()
                        .map(|frame| {
                            let sprite_ref = &frame.object_frame().sprite;
                            let sprite_sheet = sprite_sheet_handles[sprite_ref.sheet].clone();
                            let sprite_number = sprite_ref.index;
                            SpriteRender {
                                sprite_sheet,
                                sprite_number,
                            }
                        })
                        .collect::<Vec<SpriteRender>>(),
                );
                let body_sequence = BodySequence::new(
                    sequence
                        .object_sequence()
                        .frames
                        .iter()
                        .map(|frame| {
                            loader.load_from_data(
                                frame.object_frame().body.clone(),
                                (),
                                body_assets,
                            )
                        })
                        .collect::<Vec<Handle<Body>>>(),
                );
                let interactions_sequence = InteractionsSequence::new(
                    sequence
                        .object_sequence()
                        .frames
                        .iter()
                        .map(|frame| {
                            loader.load_from_data(
                                frame.object_frame().interactions.clone(),
                                (),
                                interactions_assets,
                            )
                        })
                        .collect::<Vec<Handle<Interactions>>>(),
                );

                let mut component_sequences = Vec::new();
                component_sequences.push(ComponentSequence::Wait(wait_sequence));
                component_sequences.push(ComponentSequence::SpriteRender(sprite_render_sequence));
                component_sequences.push(ComponentSequence::Body(body_sequence));
                component_sequences.push(ComponentSequence::Interactions(interactions_sequence));

                let component_sequences = ComponentSequences::new(component_sequences);
                let component_sequences_handle =
                    loader.load_from_data(component_sequences, (), component_sequences_assets);

                (*sequence_id, component_sequences_handle)
            })
            .collect::<HashMap<O::SequenceId, ComponentSequencesHandle>>();

        let object = Object::new(component_sequences_handles, sequence_end_transitions.into());
        let wrapper = O::ObjectWrapper::new(object);

        Ok(wrapper)
    }
}

#[cfg(test)]
mod test {
    use amethyst::{
        assets::{AssetStorage, Loader, Processor, ProgressCounter},
        core::TransformBundle,
        renderer::{RenderEmptyBundle, SpriteSheet, Texture},
    };
    use amethyst_test::AmethystApplication;
    use application::{load_in, Format};
    use asset_model::config::AssetRecord;
    use assets_test::{ASSETS_CHAR_BAT_PATH, ASSETS_CHAR_BAT_SLUG};
    use character_model::{
        config::{CharacterDefinition, CharacterSequenceId},
        loaded::{Character, CharacterObjectWrapper},
    };
    use collision_loading::CollisionLoadingBundle;
    use collision_model::config::{Body, Interactions};
    use sequence_loading::SequenceLoadingBundle;
    use sequence_model::loaded::ComponentSequences;
    use sprite_loading::SpriteLoader;
    use sprite_model::config::SpritesDefinition;
    use typename::TypeName;

    use super::ObjectLoader;
    use crate::{
        object::object_loader_params::ObjectLoaderParams, ObjectDefinitionToWrapperProcessor,
    };

    #[test]
    fn loads_object_assets() {
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::blank()
                .with_bundle(TransformBundle::new())
                .with_bundle(RenderEmptyBundle::new())
                .with_bundle(CollisionLoadingBundle::new())
                .with_bundle(SequenceLoadingBundle::new())
                .with_system(
                    ObjectDefinitionToWrapperProcessor::<Character>::new(),
                    ObjectDefinitionToWrapperProcessor::<Character>::type_name(),
                    &[]
                )
                .with_system(Processor::<Character>::new(), "character_processor", &[])
                .with_effect(|world| {
                    let asset_record = AssetRecord::new(
                        ASSETS_CHAR_BAT_SLUG.clone(),
                        ASSETS_CHAR_BAT_PATH.clone(),
                    );

                    let character_definition = load_in::<CharacterDefinition, _>(
                        &asset_record.path,
                        "object.toml",
                        Format::Toml,
                        None,
                    )
                    .expect("Failed to load object.toml into CharacterDefinition");

                    let object_wrapper = {
                        let sprites_definition = load_in::<SpritesDefinition, _>(
                            &asset_record.path,
                            "sprites.toml",
                            Format::Toml,
                            None,
                        )
                        .expect("Failed to load sprites_definition.");

                        let loader = &world.read_resource::<Loader>();
                        let component_sequences_assets =
                            &world.read_resource::<AssetStorage<ComponentSequences>>();
                        let texture_assets = &world.read_resource::<AssetStorage<Texture>>();
                        let sprite_sheet_assets =
                            &world.read_resource::<AssetStorage<SpriteSheet>>();

                        let body_assets = &world.read_resource::<AssetStorage<Body>>();
                        let interactions_assets =
                            &world.read_resource::<AssetStorage<Interactions>>();

                        // TODO: <https://gitlab.com/azriel91/autexousious/issues/94>
                        let sprite_sheet_handles = SpriteLoader::load(
                            &mut ProgressCounter::default(),
                            loader,
                            texture_assets,
                            sprite_sheet_assets,
                            &sprites_definition,
                            &asset_record.path,
                        )
                        .expect("Failed to load sprites.");
                        let sprite_sheet_handles = &sprite_sheet_handles;

                        ObjectLoader::load::<Character>(
                            ObjectLoaderParams {
                                loader,
                                component_sequences_assets,
                                sprite_sheet_handles,
                                body_assets,
                                interactions_assets,
                            },
                            &character_definition.object_definition,
                        )
                        .expect("Failed to load object")
                    };

                    world.add_resource(object_wrapper);
                })
                .with_assertion(|world| {
                    let object_wrapper = world.read_resource::<CharacterObjectWrapper>();

                    assert_eq!(
                        28,
                        object_wrapper.component_sequences_handles.len(),
                        "Expected 28 sequences to be loaded. \
                         Check `bat/object.toml` for number of sequences."
                    );

                    let component_sequences_assets =
                        world.read_resource::<AssetStorage<ComponentSequences>>();

                    let stand_attack_0_handle = object_wrapper
                        .component_sequences_handles
                        .get(&CharacterSequenceId::StandAttack0)
                        .expect("Expected to read `StandAttack0` component_sequences.");
                    let stand_attack_0_component_sequences = component_sequences_assets
                        .get(stand_attack_0_handle)
                        .expect("Expected `StandAttack0` component sequences to be loaded.");

                    // Wait, SpriteRender, Body, and Interactions
                    assert_eq!(4, stand_attack_0_component_sequences.len());
                })
                .run_isolated()
                .is_ok()
        );
    }
}
