use amethyst::{
    animation::{Animation, Sampler, SpriteRenderPrimitive},
    assets::{AssetStorage, Loader},
    renderer::{SpriteRender, SpriteSheet, Texture},
};
use collision_model::{
    animation::{InteractionFrameActiveHandle, InteractionFramePrimitive},
    config::InteractionFrame,
};
use derivative::Derivative;

/// Resources needed to load an object.
#[derive(Derivative)]
#[derivative(Debug)]
pub struct ObjectLoaderParams<'s> {
    /// `Loader` to load assets.
    #[derivative(Debug = "ignore")]
    pub loader: &'s Loader,
    /// `AssetStorage` for `Texture`s.
    #[derivative(Debug = "ignore")]
    pub texture_assets: &'s AssetStorage<Texture>,
    /// `AssetStorage` for `SpriteSheet`s.
    #[derivative(Debug = "ignore")]
    pub sprite_sheet_assets: &'s AssetStorage<SpriteSheet>,
    /// `AssetStorage` for `Sampler<SpriteRenderPrimitive>`s.
    #[derivative(Debug = "ignore")]
    pub sprite_render_primitive_sampler_assets: &'s AssetStorage<Sampler<SpriteRenderPrimitive>>,
    /// `AssetStorage` for `Animation<SpriteRender>`s.
    #[derivative(Debug = "ignore")]
    pub sprite_render_animation_assets: &'s AssetStorage<Animation<SpriteRender>>,
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
}
