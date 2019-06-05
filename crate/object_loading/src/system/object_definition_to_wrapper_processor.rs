use std::{marker::PhantomData, ops::Deref, sync::Arc};

use amethyst::{
    assets::{AssetStorage, HotReloadStrategy, Loader, ProcessingState},
    core::Time,
    ecs::{Read, ReadExpect, System, Write},
};
use collision_model::config::{Body, Interactions};
use derivative::Derivative;
use derive_new::new;
use object_model::{config::GameObjectDefinition, loaded::GameObject};
use rayon::ThreadPool;
use sequence_model::loaded::ComponentSequences;
use serde::{Deserialize, Serialize};
use shred_derive::SystemData;
use typename::TypeName as TypeNameTrait;
use typename_derive::TypeName;

use crate::{ObjectLoader, ObjectLoaderParams};

/// Loads `XObjectWrapper` from `XObjectDefinition`.
#[derive(Debug, Default, TypeName, new)]
pub struct ObjectDefinitionToWrapperProcessor<O>
where
    O: GameObject + TypeNameTrait,
{
    /// Marker.
    phantom_data: PhantomData<O>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct ObjectDefinitionToWrapperProcessorData<'s, O>
where
    O: GameObject,
    <O as GameObject>::SequenceId: for<'de> Deserialize<'de> + Serialize,
{
    /// `Loader` to load assets.
    #[derivative(Debug = "ignore")]
    pub loader: ReadExpect<'s, Loader>,
    /// Pool of worker threads.
    #[derivative(Debug = "ignore")]
    pub thread_pool: ReadExpect<'s, Arc<ThreadPool>>,
    /// Frame timing values.
    #[derivative(Debug = "ignore")]
    pub time: Read<'s, Time>,
    /// The asset hot reload strategy.
    #[derivative(Debug = "ignore")]
    pub hot_reload_strategy: Option<Read<'s, HotReloadStrategy>>,
    /// `AssetStorage` for the `GameObjectDefinition`s.
    #[derivative(Debug = "ignore")]
    pub game_object_definition_assets: Read<'s, AssetStorage<O::Definition>>,
    /// `AssetStorage` for `ComponentSequences`.
    #[derivative(Debug = "ignore")]
    pub component_sequences_assets: Read<'s, AssetStorage<ComponentSequences>>,
    /// `AssetStorage` for `ObjectWrapper`s.
    #[derivative(Debug = "ignore")]
    pub object_wrapper_assets: Write<'s, AssetStorage<O::ObjectWrapper>>,
    /// `AssetStorage` for `Body`s.
    #[derivative(Debug = "ignore")]
    pub body_assets: Read<'s, AssetStorage<Body>>,
    /// `AssetStorage` for `Interactions`s.
    #[derivative(Debug = "ignore")]
    pub interactions_assets: Read<'s, AssetStorage<Interactions>>,
    /// Marker.
    phantom_data: PhantomData<O>,
}

impl<'s, O> System<'s> for ObjectDefinitionToWrapperProcessor<O>
where
    O: GameObject + TypeNameTrait,
    <O as GameObject>::SequenceId: for<'de> Deserialize<'de> + Serialize,
{
    type SystemData = ObjectDefinitionToWrapperProcessorData<'s, O>;

    fn run(
        &mut self,
        ObjectDefinitionToWrapperProcessorData {
            loader,
            thread_pool,
            time,
            hot_reload_strategy,
            game_object_definition_assets,
            component_sequences_assets,
            mut object_wrapper_assets,
            body_assets,
            interactions_assets,
            ..
        }: Self::SystemData,
    ) {
        object_wrapper_assets.process(
            // F: FnMut(A::Data) -> Result<ProcessingState<A>, Error>
            |object_asset_data| {
                let game_object_definition_handle =
                    &object_asset_data.game_object_definition_handle;
                let sprite_sheet_handles = &object_asset_data.sprite_sheet_handles;

                if let Some(game_object_definition) =
                    game_object_definition_assets.get(game_object_definition_handle)
                {
                    let object_definition = game_object_definition.object_definition();

                    let wrapper = ObjectLoader::load::<O>(
                        ObjectLoaderParams {
                            loader: &loader,
                            component_sequences_assets: &component_sequences_assets,
                            sprite_sheet_handles,
                            body_assets: &body_assets,
                            interactions_assets: &interactions_assets,
                        },
                        object_definition,
                    )?;

                    Ok(ProcessingState::Loaded(wrapper))
                // kcov-ignore-start
                } else {
                    Ok(ProcessingState::Loading(object_asset_data))
                    // kcov-ignore-end
                }
            },
            time.frame_number(),
            &**thread_pool,
            hot_reload_strategy.as_ref().map(Deref::deref),
        );
    }
}
