use std::collections::HashMap;

use amethyst::{assets::Handle, renderer::SpriteRender, Error};
use collision_model::{
    config::{Body, Interactions},
    loaded::{
        BodySequence, BodySequenceHandles, InteractionsSequence, InteractionsSequenceHandles,
    },
};
use object_model::{
    config::{GameObjectFrame, GameObjectSequence, ObjectDefinition},
    loaded::{GameObject, Object, ObjectWrapper},
};
use sequence_model::{
    config::Wait,
    loaded::{
        SequenceEndTransition, SequenceEndTransitions, SequenceId, WaitSequence,
        WaitSequenceHandles,
    },
};
use serde::{Deserialize, Serialize};
use spawn_model::{
    config::Spawns,
    loaded::{SpawnsSequence, SpawnsSequenceHandles},
};
use sprite_model::loaded::{SpriteRenderSequence, SpriteRenderSequenceHandles};

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
            wait_sequence_assets,
            sprite_render_sequence_assets,
            body_sequence_assets,
            interactions_sequence_assets,
            spawns_sequence_assets,
            sprite_sheet_handles,
            body_assets,
            interactions_assets,
            spawns_assets,
        }: ObjectLoaderParams,
        object_definition: &ObjectDefinition<O::GameObjectSequence>,
    ) -> Result<O::ObjectWrapper, Error>
    where
        O: GameObject,
        <O as GameObject>::SequenceName: for<'de> Deserialize<'de> + Serialize,
    {
        // Calculate the indices of each sequence ID.
        //
        // TODO: Extract this out to a separate loading phase, as other objects may reference this
        // TODO: object's sequences.
        let sequence_id_mappings = object_definition
            .sequences
            .keys()
            .enumerate()
            .map(|(index, sequence_id)| (sequence_id.clone(), SequenceId(index)))
            .collect::<HashMap<O::SequenceName, SequenceId>>();

        let sequence_end_transitions = object_definition
            .sequences
            .values()
            .map(|sequence| {
                use sequence_model::config;
                match &sequence.object_sequence().next {
                    config::SequenceEndTransition::None => SequenceEndTransition::None,
                    config::SequenceEndTransition::Repeat => SequenceEndTransition::Repeat,
                    config::SequenceEndTransition::Delete => SequenceEndTransition::Delete,
                    config::SequenceEndTransition::SequenceName(sequence_name) => {
                        let sequence_id = sequence_id_mappings
                            .get(sequence_name)
                            .map(|index| SequenceId(**index))
                            .unwrap_or_else(|| {
                                panic!(
                                    "Invalid sequence ID specified for `next`: `{}`",
                                    sequence_name
                                )
                            });
                        SequenceEndTransition::SequenceId(sequence_id)
                    }
                }
            })
            .collect::<Vec<SequenceEndTransition>>();

        // Load frame component datas
        let sequences_handles = (
            WaitSequenceHandles::default(),
            SpriteRenderSequenceHandles::default(),
            BodySequenceHandles::default(),
            InteractionsSequenceHandles::default(),
            SpawnsSequenceHandles::default(),
        );
        let (
            wait_sequence_handles,
            sprite_render_sequence_handles,
            body_sequence_handles,
            interactions_sequence_handles,
            spawns_sequence_handles,
        ) = object_definition.sequences.values().fold(
            sequences_handles,
            |(
                mut wait_sequence_handles,
                mut sprite_render_sequence_handles,
                mut body_sequence_handles,
                mut interactions_sequence_handles,
                mut spawns_sequence_handles,
            ),
             sequence| {
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
                let spawns_sequence = SpawnsSequence::new(
                    sequence
                        .object_sequence()
                        .frames
                        .iter()
                        .map(|frame| {
                            loader.load_from_data(
                                frame.object_frame().spawns.clone(),
                                (),
                                spawns_assets,
                            )
                        })
                        .collect::<Vec<Handle<Spawns>>>(),
                );

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
                let spawns_sequence_handle =
                    loader.load_from_data(spawns_sequence, (), spawns_sequence_assets);

                wait_sequence_handles.push(wait_sequence_handle);
                sprite_render_sequence_handles.push(sprite_render_sequence_handle);
                body_sequence_handles.push(body_sequence_handle);
                interactions_sequence_handles.push(interactions_sequence_handle);
                spawns_sequence_handles.push(spawns_sequence_handle);

                (
                    wait_sequence_handles,
                    sprite_render_sequence_handles,
                    body_sequence_handles,
                    interactions_sequence_handles,
                    spawns_sequence_handles,
                )
            },
        );

        let object = Object::new(
            wait_sequence_handles,
            sprite_render_sequence_handles,
            body_sequence_handles,
            interactions_sequence_handles,
            spawns_sequence_handles,
            SequenceEndTransitions::new(sequence_end_transitions),
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
        ecs::{Read, WorldExt},
        renderer::{types::DefaultBackend, RenderEmptyBundle, SpriteSheet, Texture},
    };
    use amethyst_test::AmethystApplication;
    use application::{load_in, Format};
    use asset_model::config::AssetRecord;
    use assets_test::{CHAR_BAT_PATH, CHAR_BAT_SLUG};
    use character_model::{
        config::CharacterDefinition,
        loaded::{Character, CharacterObjectWrapper},
    };
    use collision_loading::CollisionLoadingBundle;
    use sequence_loading::SequenceLoadingBundle;
    use spawn_loading::SpawnLoadingBundle;
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
                .with_bundle(SpawnLoadingBundle::new())
                .with_bundle(SequenceLoadingBundle::new())
                .with_system(
                    ObjectDefinitionToWrapperProcessor::<Character>::new(),
                    ObjectDefinitionToWrapperProcessor::<Character>::type_name(),
                    &[]
                )
                .with_system(Processor::<Character>::new(), "character_processor", &[])
                .with_effect(|world| {
                    let asset_record =
                        AssetRecord::new(CHAR_BAT_SLUG.clone(), CHAR_BAT_PATH.clone());

                    let character_definition = load_in::<CharacterDefinition, _>(
                        &asset_record.path,
                        "object.yaml",
                        Format::Yaml,
                        None,
                    )
                    .expect("Failed to load object.yaml into CharacterDefinition");

                    let object_wrapper = {
                        let sprites_definition = load_in::<SpritesDefinition, _>(
                            &asset_record.path,
                            "sprites.yaml",
                            Format::Yaml,
                            None,
                        )
                        .expect("Failed to load sprites_definition.");

                        let (
                            ObjectLoaderSystemData {
                                loader,
                                wait_sequence_assets,
                                sprite_render_sequence_assets,
                                body_sequence_assets,
                                interactions_sequence_assets,
                                spawns_sequence_assets,
                                body_assets,
                                interactions_assets,
                                spawns_assets,
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
                                wait_sequence_assets: &wait_sequence_assets,
                                sprite_render_sequence_assets: &sprite_render_sequence_assets,
                                body_sequence_assets: &body_sequence_assets,
                                interactions_sequence_assets: &interactions_sequence_assets,
                                spawns_sequence_assets: &spawns_sequence_assets,
                                sprite_sheet_handles,
                                body_assets: &body_assets,
                                interactions_assets: &interactions_assets,
                                spawns_assets: &spawns_assets,
                            },
                            &character_definition.object_definition,
                        )
                        .expect("Failed to load object")
                    };

                    world.insert(object_wrapper);
                })
                .with_assertion(|world| {
                    let object_wrapper = world.read_resource::<CharacterObjectWrapper>();

                    macro_rules! assert_frame_component_data_count {
                        ($frame_component_data_field:ident) => {
                            assert_eq!(
                                28,
                                object_wrapper.$frame_component_data_field.len(),
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
                .is_ok()
        );
    }

    type TestSystemData<'s> = (
        ObjectLoaderSystemData<'s>,
        Read<'s, AssetStorage<Texture>>,
        Read<'s, AssetStorage<SpriteSheet>>,
    );
}
