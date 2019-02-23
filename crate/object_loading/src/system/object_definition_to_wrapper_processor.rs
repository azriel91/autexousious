use collision_model::config::{Body, Interactions};
use std::{marker::PhantomData, ops::Deref, sync::Arc};

use amethyst::{
    animation::{Animation, Sampler, SpriteRenderPrimitive},
    assets::{AssetStorage, HotReloadStrategy, Loader, ProcessingState},
    core::Time,
    ecs::{Read, ReadExpect, System, Write},
    renderer::SpriteRender,
};
use collision_model::{
    animation::{
        BodyFrameActiveHandle, BodyFramePrimitive, InteractionFrameActiveHandle,
        InteractionFramePrimitive,
    },
    config::{BodyFrame, InteractionFrame},
};
use derivative::Derivative;
use derive_new::new;
use object_model::{config::GameObjectDefinition, loaded::GameObject};
use rayon::ThreadPool;
use shred_derive::SystemData;
use typename::TypeName as TypeNameTrait;
use typename_derive::TypeName;

use crate::{object::ObjectLoaderParams, ObjectLoader};

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
    /// `AssetStorage` for `ObjectWrapper`s.
    #[derivative(Debug = "ignore")]
    pub object_wrapper_assets: Write<'s, AssetStorage<O::ObjectWrapper>>,
    /// `AssetStorage` for `Sampler<SpriteRenderPrimitive>`s.
    #[derivative(Debug = "ignore")]
    pub sprite_render_primitive_sampler_assets:
        Read<'s, AssetStorage<Sampler<SpriteRenderPrimitive>>>,
    /// `AssetStorage` for `Animation<SpriteRender>`s.
    #[derivative(Debug = "ignore")]
    pub sprite_render_animation_assets: Read<'s, AssetStorage<Animation<SpriteRender>>>,
    /// `AssetStorage` for `BodyFrame`s.
    #[derivative(Debug = "ignore")]
    pub body_frame_assets: Read<'s, AssetStorage<BodyFrame>>,
    /// `AssetStorage` for `Sampler<BodyFramePrimitive>`s.
    #[derivative(Debug = "ignore")]
    pub body_frame_primitive_sampler_assets: Read<'s, AssetStorage<Sampler<BodyFramePrimitive>>>,
    /// `AssetStorage` for `Animation<BodyFrameActiveHandle>`s.
    #[derivative(Debug = "ignore")]
    pub body_frame_animation_assets: Read<'s, AssetStorage<Animation<BodyFrameActiveHandle>>>,
    /// `AssetStorage` for `InteractionFrame`s.
    #[derivative(Debug = "ignore")]
    pub interaction_frame_assets: Read<'s, AssetStorage<InteractionFrame>>,
    /// `AssetStorage` for `Sampler<InteractionFramePrimitive>`s.
    #[derivative(Debug = "ignore")]
    pub interaction_frame_primitive_sampler_assets:
        Read<'s, AssetStorage<Sampler<InteractionFramePrimitive>>>,
    /// `AssetStorage` for `Animation<InteractionFrameActiveHandle>`s.
    #[derivative(Debug = "ignore")]
    pub interaction_frame_animation_assets:
        Read<'s, AssetStorage<Animation<InteractionFrameActiveHandle>>>,
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
            mut object_wrapper_assets,
            sprite_render_primitive_sampler_assets,
            sprite_render_animation_assets,
            body_frame_assets,
            body_frame_primitive_sampler_assets,
            body_frame_animation_assets,
            interaction_frame_assets,
            interaction_frame_primitive_sampler_assets,
            interaction_frame_animation_assets,
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
                            sprite_sheet_handles,
                            sprite_render_primitive_sampler_assets:
                                &sprite_render_primitive_sampler_assets,
                            sprite_render_animation_assets: &sprite_render_animation_assets,
                            body_frame_assets: &body_frame_assets,
                            body_frame_primitive_sampler_assets:
                                &body_frame_primitive_sampler_assets,
                            body_frame_animation_assets: &body_frame_animation_assets,
                            interaction_frame_assets: &interaction_frame_assets,
                            interaction_frame_primitive_sampler_assets:
                                &interaction_frame_primitive_sampler_assets,
                            interaction_frame_animation_assets: &interaction_frame_animation_assets,
                            body_assets: &body_assets,
                            interactions_assets: &interactions_assets,
                        },
                        object_definition,
                    )?;

                    Ok(ProcessingState::Loaded(wrapper))
                } else {
                    Ok(ProcessingState::Loading(object_asset_data))
                }
            },
            time.frame_number(),
            &**thread_pool,
            hot_reload_strategy.as_ref().map(Deref::deref),
        );
    }
}
