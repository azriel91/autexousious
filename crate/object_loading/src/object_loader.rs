use std::{collections::HashMap, str::FromStr};

use amethyst::{assets::Handle, renderer::SpriteRender};
use asset_model::config::AssetType;
use audio_loading::AudioLoader;
use audio_model::loaded::{SourceHandleOpt, SourceSequence, SourceSequenceHandles};
use character_model::config::CharacterSequenceName;
use collision_model::{
    config::{Body, Interactions},
    loaded::{
        BodySequence, BodySequenceHandles, InteractionsSequence, InteractionsSequenceHandles,
    },
};
use energy_model::config::EnergySequenceName;
use kinematic_model::{
    config::{ObjectAcceleration, Position, Velocity},
    loaded::{ObjectAccelerationSequence, ObjectAccelerationSequenceHandles},
};
use log::error;
use object_model::{
    config::{GameObjectFrame, GameObjectSequence, ObjectDefinition},
    loaded::Object,
};
use object_type::ObjectType;
use sequence_model::{
    config::{SequenceNameString, Wait},
    loaded::{
        SequenceEndTransition, SequenceEndTransitions, SequenceId, WaitSequence,
        WaitSequenceHandles,
    },
};
use serde::{Deserialize, Serialize};
use spawn_model::loaded::{Spawn, Spawns, SpawnsSequence, SpawnsSequenceHandles};
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
    pub fn load<GOS>(
        ObjectLoaderParams {
            loader,
            asset_id_mappings,
            asset_type_mappings,
            asset_sequence_id_mappings_character,
            asset_sequence_id_mappings_energy,
            wait_sequence_assets,
            source_assets,
            source_sequence_assets,
            object_acceleration_sequence_assets,
            sprite_render_sequence_assets,
            body_sequence_assets,
            interactions_sequence_assets,
            spawns_sequence_assets,
            sprite_sheet_handles,
            body_assets,
            interactions_assets,
            spawns_assets,
        }: ObjectLoaderParams,
        object_definition: &ObjectDefinition<GOS>,
    ) -> Object
    where
        GOS: GameObjectSequence,
        GOS::SequenceName: for<'de> Deserialize<'de> + Serialize,
    {
        // Calculate the indices of each sequence ID.
        //
        // TODO: Extract this out to a separate loading phase, as other objects may reference this
        // TODO: object's sequences.
        let sequence_id_mappings = object_definition
            .sequences
            .keys()
            .enumerate()
            .map(|(index, sequence_name_string)| (sequence_name_string.clone(), SequenceId(index)))
            .collect::<HashMap<SequenceNameString<GOS::SequenceName>, SequenceId>>();

        let sequence_end_transitions = object_definition
            .sequences
            .values()
            .map(|sequence| {
                use sequence_model::config;
                match &sequence.object_sequence().sequence.next {
                    config::SequenceEndTransition::None => SequenceEndTransition::None,
                    config::SequenceEndTransition::Repeat => SequenceEndTransition::Repeat,
                    config::SequenceEndTransition::Delete => SequenceEndTransition::Delete,
                    config::SequenceEndTransition::SequenceName(sequence_name) => {
                        let sequence_id = sequence_id_mappings
                            .get(&sequence_name)
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
            SourceSequenceHandles::default(),
            ObjectAccelerationSequenceHandles::default(),
            SpriteRenderSequenceHandles::default(),
            BodySequenceHandles::default(),
            InteractionsSequenceHandles::default(),
            SpawnsSequenceHandles::default(),
        );
        let (
            wait_sequence_handles,
            source_sequence_handles,
            object_acceleration_sequence_handles,
            sprite_render_sequence_handles,
            body_sequence_handles,
            interactions_sequence_handles,
            spawns_sequence_handles,
        ) = object_definition.sequences.values().fold(
            sequences_handles,
            |(
                mut wait_sequence_handles,
                mut source_sequence_handles,
                mut object_acceleration_sequence_handles,
                mut sprite_render_sequence_handles,
                mut body_sequence_handles,
                mut interactions_sequence_handles,
                mut spawns_sequence_handles,
            ),
             sequence| {
                let object_sequence = sequence.object_sequence();

                let wait_sequence = WaitSequence::new(
                    object_sequence
                        .sequence
                        .frames
                        .iter()
                        .map(|frame| frame.object_frame().wait)
                        .collect::<Vec<Wait>>(),
                );
                let source_sequence = SourceSequence::new(
                    object_sequence
                        .sequence
                        .frames
                        .iter()
                        .map(|frame| {
                            let source_handle_opt =
                                frame.object_frame().sound.as_ref().map(|source_path| {
                                    AudioLoader::load(loader, source_assets, (), source_path)
                                });
                            SourceHandleOpt::new(source_handle_opt)
                        })
                        .collect::<Vec<SourceHandleOpt>>(),
                );
                let object_acceleration_sequence = ObjectAccelerationSequence::new(
                    object_sequence
                        .sequence
                        .frames
                        .iter()
                        .map(|frame| {
                            frame
                                .object_frame()
                                .acceleration
                                .or(object_sequence.acceleration)
                                .unwrap_or_else(ObjectAcceleration::default)
                        })
                        .collect::<Vec<ObjectAcceleration>>(),
                );
                let sprite_render_sequence = SpriteRenderSequence::new(
                    object_sequence
                        .sequence
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
                    object_sequence
                        .sequence
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
                    object_sequence
                        .sequence
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
                    object_sequence
                        .sequence
                        .frames
                        .iter()
                        .map(|frame| {
                            let spawns = frame
                                .object_frame()
                                .spawns
                                .iter()
                                .map(|spawn_config| {
                                    let spawn_asset_slug = &spawn_config.object;
                                    let spawn_asset_id = asset_id_mappings
                                        .id(spawn_asset_slug)
                                        .copied()
                                        .unwrap_or_else(|| panic!("Asset ID not found for `{}`.", spawn_asset_slug));
                                    let spawn_asset_type = asset_type_mappings
                                        .get(spawn_asset_id)
                                        .expect("Expected `AssetType` mapping to exist.");
                                    let position = {
                                        let position_config = spawn_config.position;
                                        Position::<f32>::new(position_config.x as f32, position_config.y as f32, position_config.z as f32)
                                    };
                                    let velocity = {
                                        let velocity_config = spawn_config.velocity;
                                        Velocity::<f32>::new(velocity_config.x as f32, velocity_config.y as f32, velocity_config.z as f32)
                                    };

                                    let sequence_id = match spawn_asset_type {
                                        AssetType::Object(spawn_object_type) => match spawn_object_type {
                                            ObjectType::Character => {
                                                let spawn_sequence_id_mappings = asset_sequence_id_mappings_character.get(spawn_asset_id)
                                                    .unwrap_or_else(|| panic!("`SequenceIdMappings<Character>` not found for `{}`.", spawn_asset_slug));

                                                if let Some(sequence_string) = spawn_config.sequence.as_ref() {
                                                    let sequence_name_string = SequenceNameString::from_str(sequence_string).expect("Expected `SequenceNameString::from_str` to succeed.");
                                                    spawn_sequence_id_mappings.id(&sequence_name_string).copied().unwrap_or_else(|| {
                                                        let message = format!("Sequence ID not found for string: `{}` in `{}`. Falling back to default.", sequence_string, spawn_asset_slug);
                                                        error!("{}", message);

                                                        let sequence_default = CharacterSequenceName::default();
                                                        spawn_sequence_id_mappings.id(&SequenceNameString::from(sequence_default)).copied()
                                                            .unwrap_or_else(|| panic!("`{}` sequence not found for `{}`", sequence_default, spawn_asset_slug))
                                                    })
                                                } else {
                                                    let sequence_default = CharacterSequenceName::default();
                                                        spawn_sequence_id_mappings.id(&SequenceNameString::from(sequence_default)).copied()
                                                            .unwrap_or_else(|| panic!("`{}` sequence not found for `{}`", sequence_default, spawn_asset_slug))
                                                }
                                            }
                                            ObjectType::Energy => {
                                                let spawn_sequence_id_mappings = asset_sequence_id_mappings_energy.get(spawn_asset_id)
                                                    .unwrap_or_else(|| panic!("`SequenceIdMappings<Energy>` not found for `{}`.", spawn_asset_slug));

                                                if let Some(sequence_string) = spawn_config.sequence.as_ref() {
                                                    let sequence_name_string = SequenceNameString::from_str(sequence_string).expect("Expected `SequenceNameString::from_str` to succeed.");
                                                    spawn_sequence_id_mappings.id(&sequence_name_string).copied().unwrap_or_else(|| {
                                                        let message = format!("Sequence ID not found for string: `{}` in `{}`. Falling back to default.", sequence_string, spawn_asset_slug);
                                                        error!("{}", message);

                                                        let sequence_default = EnergySequenceName::default();
                                                        spawn_sequence_id_mappings.id(&SequenceNameString::from(sequence_default)).copied()
                                                            .unwrap_or_else(|| panic!("`{}` sequence not found for `{}`", sequence_default, spawn_asset_slug))
                                                    })
                                                } else {
                                                    let sequence_default = EnergySequenceName::default();
                                                        spawn_sequence_id_mappings.id(&SequenceNameString::from(sequence_default)).copied()
                                                            .unwrap_or_else(|| panic!("`{}` sequence not found for `{}`", sequence_default, spawn_asset_slug))
                                                }
                                            }
                                            ObjectType::TestObject => {
                                                panic!("Spawning `TestObject`s is not supported.")
                                            }
                                        },
                                        AssetType::Map | AssetType::Ui => panic!("Spawning `Map`s is not supported."),
                                    };

                                    Spawn {
                                        object: spawn_asset_id,
                                        position,
                                        velocity,
                                        sequence_id,
                                    }
                                })
                                .collect::<Vec<Spawn>>();
                            let spawns = Spawns::new(spawns);

                            loader.load_from_data(spawns, (), spawns_assets)
                        })
                        .collect::<Vec<Handle<Spawns>>>(),
                );

                let wait_sequence_handle =
                    loader.load_from_data(wait_sequence, (), wait_sequence_assets);
                let source_sequence_handle =
                    loader.load_from_data(source_sequence, (), source_sequence_assets);
                let object_acceleration_sequence_handle = loader.load_from_data(
                    object_acceleration_sequence,
                    (),
                    object_acceleration_sequence_assets,
                );
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
                source_sequence_handles.push(source_sequence_handle);
                object_acceleration_sequence_handles.push(object_acceleration_sequence_handle);
                sprite_render_sequence_handles.push(sprite_render_sequence_handle);
                body_sequence_handles.push(body_sequence_handle);
                interactions_sequence_handles.push(interactions_sequence_handle);
                spawns_sequence_handles.push(spawns_sequence_handle);

                (
                    wait_sequence_handles,
                    source_sequence_handles,
                    object_acceleration_sequence_handles,
                    sprite_render_sequence_handles,
                    body_sequence_handles,
                    interactions_sequence_handles,
                    spawns_sequence_handles,
                )
            },
        );

        Object::new(
            wait_sequence_handles,
            source_sequence_handles,
            object_acceleration_sequence_handles,
            sprite_render_sequence_handles,
            body_sequence_handles,
            interactions_sequence_handles,
            spawns_sequence_handles,
            SequenceEndTransitions::new(sequence_end_transitions),
        )
    }
}
