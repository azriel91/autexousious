use std::collections::HashMap;

use amethyst::{assets::Handle, renderer::SpriteRender, Error};
use collision_model::{
    config::{Body, Interactions},
    loaded::{BodySequence, BodySequenceHandle, InteractionsSequence, InteractionsSequenceHandle},
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
        WaitSequence, WaitSequenceHandle,
    },
};
use serde::{Deserialize, Serialize};
use sprite_model::loaded::{SpriteRenderSequence, SpriteRenderSequenceHandle};

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
            wait_sequence_assets,
            sprite_render_sequence_assets,
            body_sequence_assets,
            interactions_sequence_assets,
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
        let sequences_handles = (
            HashMap::<O::SequenceId, ComponentSequencesHandle>::new(),
            HashMap::<O::SequenceId, WaitSequenceHandle>::new(),
            HashMap::<O::SequenceId, SpriteRenderSequenceHandle>::new(),
            HashMap::<O::SequenceId, BodySequenceHandle>::new(),
            HashMap::<O::SequenceId, InteractionsSequenceHandle>::new(),
        );
        let (
            component_sequences_handles,
            wait_sequence_handles,
            sprite_render_sequence_handles,
            body_sequence_handles,
            interactions_sequence_handles,
        ) = object_definition.sequences.iter().fold(
            sequences_handles,
            |(
                mut component_sequences_handles,
                mut wait_sequence_handles,
                mut sprite_render_sequence_handles,
                mut body_sequence_handles,
                mut interactions_sequence_handles,
            ),
             (sequence_id, sequence)| {
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
                component_sequences.push(ComponentSequence::Wait(wait_sequence.clone()));
                component_sequences.push(ComponentSequence::SpriteRender(
                    sprite_render_sequence.clone(),
                ));
                component_sequences.push(ComponentSequence::Body(body_sequence.clone()));
                component_sequences.push(ComponentSequence::Interactions(
                    interactions_sequence.clone(),
                ));

                let wait_sequence_handle =
                    loader.load_from_data(wait_sequence, (), wait_sequence_assets);
                let sprite_render_sequence_handle = loader.load_from_data(
                    sprite_render_sequence,
                    (),
                    sprite_render_sequence_assets,
                );
                let body_sequence_handle =
                    loader.load_from_data(body_sequence, (), body_sequence_assets);
                let interactions_sequence_handle =
                    loader.load_from_data(interactions_sequence, (), interactions_sequence_assets);

                let component_sequences = ComponentSequences::new(component_sequences);
                let component_sequences_handle =
                    loader.load_from_data(component_sequences, (), component_sequences_assets);

                let sequence_id = *sequence_id;

                component_sequences_handles.insert(sequence_id, component_sequences_handle);
                wait_sequence_handles.insert(sequence_id, wait_sequence_handle);
                sprite_render_sequence_handles.insert(sequence_id, sprite_render_sequence_handle);
                body_sequence_handles.insert(sequence_id, body_sequence_handle);
                interactions_sequence_handles.insert(sequence_id, interactions_sequence_handle);

                (
                    component_sequences_handles,
                    wait_sequence_handles,
                    sprite_render_sequence_handles,
                    body_sequence_handles,
                    interactions_sequence_handles,
                )
            },
        );

        let object = Object::new(
            component_sequences_handles,
            wait_sequence_handles,
            sprite_render_sequence_handles,
            body_sequence_handles,
            interactions_sequence_handles,
            sequence_end_transitions.into(),
        );
        let wrapper = O::ObjectWrapper::new(object);

        Ok(wrapper)
    }
}

#[cfg(test)]
mod test {
    use amethyst::{
        assets::{AssetStorage, Processor, ProgressCounter},
        core::TransformBundle,
        ecs::Read,
        renderer::{types::DefaultBackend, RenderEmptyBundle, SpriteSheet, Texture},
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
    use sequence_loading::SequenceLoadingBundle;
    use sequence_model::loaded::ComponentSequences;
    use sprite_loading::SpriteLoader;
    use sprite_model::config::SpritesDefinition;
    use typename::TypeName;

    use super::ObjectLoader;
    use crate::{ObjectDefinitionToWrapperProcessor, ObjectLoaderParams, ObjectLoaderSystemData};

    #[test]
    fn loads_object_assets() {
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::blank()
                .with_bundle(TransformBundle::new())
                .with_bundle(RenderEmptyBundle::<DefaultBackend>::new())
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

                        let (
                            ObjectLoaderSystemData {
                                loader,
                                component_sequences_assets,
                                wait_sequence_assets,
                                sprite_render_sequence_assets,
                                body_sequence_assets,
                                interactions_sequence_assets,
                                body_assets,
                                interactions_assets,
                            },
                            texture_assets,
                            sprite_sheet_assets,
                        ) = world.system_data::<TestSystemData>();

                        // TODO: <https://gitlab.com/azriel91/autexousious/issues/94>
                        let sprite_sheet_handles = SpriteLoader::load(
                            &mut ProgressCounter::default(),
                            &loader,
                            &texture_assets,
                            &sprite_sheet_assets,
                            &sprites_definition,
                            &asset_record.path,
                        )
                        .expect("Failed to load sprites.");
                        let sprite_sheet_handles = &sprite_sheet_handles;

                        ObjectLoader::load::<Character>(
                            ObjectLoaderParams {
                                loader: &loader,
                                component_sequences_assets: &component_sequences_assets,
                                wait_sequence_assets: &wait_sequence_assets,
                                sprite_render_sequence_assets: &sprite_render_sequence_assets,
                                body_sequence_assets: &body_sequence_assets,
                                interactions_sequence_assets: &interactions_sequence_assets,
                                sprite_sheet_handles,
                                body_assets: &body_assets,
                                interactions_assets: &interactions_assets,
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

    type TestSystemData<'s> = (
        ObjectLoaderSystemData<'s>,
        Read<'s, AssetStorage<Texture>>,
        Read<'s, AssetStorage<SpriteSheet>>,
    );
}
