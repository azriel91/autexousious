use amethyst::{
    animation::{Animation, Sampler, SpriteRenderPrimitive},
    assets::{AssetStorage, Loader},
    renderer::{SpriteRender, SpriteSheetHandle},
};
use collision_model::{
    animation::{
        BodyFrameActiveHandle, BodyFramePrimitive, InteractionFrameActiveHandle,
        InteractionFramePrimitive,
    },
    config::{Body, BodyFrame, InteractionFrame, Interactions},
};
use derivative::Derivative;

/// Resources needed to load an object.
#[derive(Derivative)]
#[derivative(Debug)]
pub struct ObjectLoaderParams<'s> {
    /// `Loader` to load assets.
    #[derivative(Debug = "ignore")]
    pub loader: &'s Loader,
    /// Handles to the sprite sheets for this `Object`.
    pub sprite_sheet_handles: &'s [SpriteSheetHandle],
    /// `AssetStorage` for `Sampler<SpriteRenderPrimitive>`s.
    #[derivative(Debug = "ignore")]
    pub sprite_render_primitive_sampler_assets: &'s AssetStorage<Sampler<SpriteRenderPrimitive>>,
    /// `AssetStorage` for `Animation<SpriteRender>`s.
    #[derivative(Debug = "ignore")]
    pub sprite_render_animation_assets: &'s AssetStorage<Animation<SpriteRender>>,
    /// `AssetStorage` for `BodyFrame`s.
    #[derivative(Debug = "ignore")]
    pub body_frame_assets: &'s AssetStorage<BodyFrame>,
    /// `AssetStorage` for `Sampler<BodyFramePrimitive>`s.
    #[derivative(Debug = "ignore")]
    pub body_frame_primitive_sampler_assets: &'s AssetStorage<Sampler<BodyFramePrimitive>>,
    /// `AssetStorage` for `Animation<BodyFrameActiveHandle>`s.
    #[derivative(Debug = "ignore")]
    pub body_frame_animation_assets: &'s AssetStorage<Animation<BodyFrameActiveHandle>>,
    /// `AssetStorage` for `InteractionFrame`s.
    #[derivative(Debug = "ignore")]
    pub interaction_frame_assets: &'s AssetStorage<InteractionFrame>,
    /// `AssetStorage` for `Sampler<InteractionFramePrimitive>`s.
    #[derivative(Debug = "ignore")]
    pub interaction_frame_primitive_sampler_assets:
        &'s AssetStorage<Sampler<InteractionFramePrimitive>>,
    /// `AssetStorage` for `Animation<InteractionFrameActiveHandle>`s.
    #[derivative(Debug = "ignore")]
    pub interaction_frame_animation_assets:
        &'s AssetStorage<Animation<InteractionFrameActiveHandle>>,
    /// `AssetStorage` for `Body`s.
    #[derivative(Debug = "ignore")]
    pub body_assets: &'s AssetStorage<Body>,
    /// `AssetStorage` for `Interactions`s.
    #[derivative(Debug = "ignore")]
    pub interactions_assets: &'s AssetStorage<Interactions>,
}
