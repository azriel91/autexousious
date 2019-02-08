use std::{marker::PhantomData, ops::Deref, sync::Arc};

use amethyst::{
    animation::{Animation, Sampler, SpriteRenderPrimitive},
    assets::{AssetStorage, HotReloadStrategy, Loader, ProcessingState},
    core::Time,
    ecs::{Read, ReadExpect, System, Write},
    renderer::SpriteRender,
};
use collision_model::{
    animation::{InteractionFrameActiveHandle, InteractionFramePrimitive},
    config::InteractionFrame,
};
use derive_new::new;
use object_model::{config::GameObjectDefinition, loaded::GameObject};
use rayon::ThreadPool;
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

type ObjectDefinitionToWrapperProcessorData<'s, D, W> = (
    ReadExpect<'s, Loader>,
    ReadExpect<'s, Arc<ThreadPool>>,
    Read<'s, Time>,
    Option<Read<'s, HotReloadStrategy>>,
    Read<'s, AssetStorage<D>>,
    Write<'s, AssetStorage<W>>,
    // `AssetStorage`s needed to load the `Object`
    Read<'s, AssetStorage<Sampler<SpriteRenderPrimitive>>>,
    Read<'s, AssetStorage<Animation<SpriteRender>>>,
    Read<'s, AssetStorage<InteractionFrame>>,
    Read<'s, AssetStorage<Sampler<InteractionFramePrimitive>>>,
    Read<'s, AssetStorage<Animation<InteractionFrameActiveHandle>>>,
);

impl<'s, O> System<'s> for ObjectDefinitionToWrapperProcessor<O>
where
    O: GameObject + TypeNameTrait,
{
    type SystemData = ObjectDefinitionToWrapperProcessorData<'s, O::Definition, O::ObjectWrapper>;

    fn run(
        &mut self,
        (
            loader,
            thread_pool,
            time,
            hot_reload_strategy,
            game_object_definition_assets,
            mut object_wrapper_assets,
            sprite_render_primitive_sampler_assets,
            sprite_render_animation_assets,
            interaction_frame_assets,
            interaction_frame_primitive_sampler_assets,
            interaction_frame_animation_assets,
        ): Self::SystemData,
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
                            sprite_sheet_handles: sprite_sheet_handles,
                            sprite_render_primitive_sampler_assets:
                                &sprite_render_primitive_sampler_assets,
                            sprite_render_animation_assets: &sprite_render_animation_assets,
                            interaction_frame_assets: &interaction_frame_assets,
                            interaction_frame_primitive_sampler_assets:
                                &interaction_frame_primitive_sampler_assets,
                            interaction_frame_animation_assets: &interaction_frame_animation_assets,
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
