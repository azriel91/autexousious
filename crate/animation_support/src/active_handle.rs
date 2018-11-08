use std::fmt::Debug;

use amethyst::{
    animation::{AnimationSampling, ApplyData, BlendMethod},
    assets::Handle,
    ecs::{Component, DenseVecStorage},
};
use named_type::NamedType;

use ActiveHandleChannel;
use ActiveHandlePrimitive;

/// Wrapper `Component` to allow switching between memory-heavy data.
#[derive(Clone, Debug, NamedType, PartialEq, new)]
pub struct ActiveHandle<T> {
    /// Handle of the component to use when no animations are running.
    pub rest: Handle<T>,
    /// Handle of the component to use during an animation.
    #[new(default)]
    pub active: Option<Handle<T>>,
}

impl<T> Component for ActiveHandle<T>
where
    T: Clone + Debug + Send + Sync + 'static,
{
    type Storage = DenseVecStorage<Self>;
}

impl<'s, T> ApplyData<'s> for ActiveHandle<T>
where
    T: Clone + Debug + Send + Sync + 'static,
{
    type ApplyData = ();
}

impl<T> AnimationSampling for ActiveHandle<T>
where
    T: Clone + Debug + Send + Sync + 'static,
{
    type Primitive = ActiveHandlePrimitive<T>;
    type Channel = ActiveHandleChannel;

    fn apply_sample(&mut self, channel: &Self::Channel, data: &Self::Primitive, _: &()) {
        use ActiveHandleChannel as Channel;
        use ActiveHandlePrimitive as Primitive;

        match (channel, data) {
            (Channel::Handle, Primitive::Handle(handle)) => self.active = Some(handle.clone()),
        }
    }

    fn current_sample(&self, channel: &Self::Channel, _: &()) -> Self::Primitive {
        use ActiveHandleChannel as Channel;
        use ActiveHandlePrimitive as Primitive;

        match channel {
            Channel::Handle => {
                Primitive::Handle(self.active.as_ref().unwrap_or(&self.rest).clone())
            }
        }
    }

    fn default_primitive(_: &Self::Channel) -> Self::Primitive {
        panic!("Blending is not applicable to ActiveHandle animation")
    }

    fn blend_method(&self, _: &Self::Channel) -> Option<BlendMethod> {
        None
    }
}
