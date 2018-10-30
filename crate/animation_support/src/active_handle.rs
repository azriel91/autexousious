use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::marker::PhantomData;

use amethyst::{
    animation::{AnimationSampling, ApplyData, BlendMethod},
    assets::Handle,
    ecs::{Component, DenseVecStorage, ReadExpect},
};
use named_type::NamedType;

use ActiveHandleChannel;
use ActiveHandlePrimitive;
use AnimationDataSet;

/// Wrapper `Component` to allow switching between memory-heavy data.
#[derive(Clone, Debug, NamedType, PartialEq, new)]
pub struct ActiveHandle<I, T> {
    /// Handle of the component to use when no animations are running.
    pub rest: Handle<T>,
    /// Handle of the component to use during an animation.
    #[new(default)]
    pub active: Option<Handle<T>>,
    /// `PhantomData` for ID.
    phantom_data: PhantomData<I>,
}

impl<I, T> Component for ActiveHandle<I, T>
where
    I: Clone + Copy + Debug + Display + Hash + PartialEq + Eq + Send + Sync + 'static,
    T: Clone + Debug + Send + Sync + 'static,
{
    type Storage = DenseVecStorage<Self>;
}

impl<'s, I, T> ApplyData<'s> for ActiveHandle<I, T>
where
    I: Clone + Copy + Debug + Display + Hash + PartialEq + Eq + Send + Sync + 'static,
    T: Clone + Debug + Send + Sync + 'static,
{
    type ApplyData = ReadExpect<'s, AnimationDataSet<I, Handle<T>>>;
}

impl<I, T> AnimationSampling for ActiveHandle<I, T>
where
    I: Clone + Copy + Debug + Display + Hash + PartialEq + Eq + Send + Sync + 'static,
    T: Clone + Debug + Send + Sync + 'static,
{
    type Primitive = ActiveHandlePrimitive<I>;
    type Channel = ActiveHandleChannel;

    fn apply_sample(
        &mut self,
        channel: &Self::Channel,
        data: &Self::Primitive,
        animation_data_set: &ReadExpect<AnimationDataSet<I, Handle<T>>>,
    ) {
        use ActiveHandleChannel as Channel;
        use ActiveHandlePrimitive as Primitive;

        match (*channel, *data) {
            (Channel::Handle, Primitive::Handle(id)) => {
                self.active = Some(animation_data_set.data(id).unwrap_or_else(|| {
                    panic!(
                        "Unable to get `{}` from `{}` with id: `{}`.",
                        Self::type_name(),
                        <AnimationDataSet<I, Handle<T>>>::type_name(),
                        id
                    )
                }))
            }
        }
    }

    fn current_sample(
        &self,
        channel: &Self::Channel,
        animation_data_set: &ReadExpect<AnimationDataSet<I, Handle<T>>>,
    ) -> Self::Primitive {
        use ActiveHandleChannel as Channel;
        use ActiveHandlePrimitive as Primitive;

        match *channel {
            Channel::Handle => Primitive::Handle(
                animation_data_set
                    .id(self.active.as_ref().unwrap_or(&self.rest))
                    .unwrap_or_else(|| {
                        panic!(
                            "Unable to get ID for `{}` from `{}`, active_handle: `{:?}`.",
                            Self::type_name(),
                            <AnimationDataSet<I, Handle<T>>>::type_name(),
                            self
                        )
                    }),
            ),
        }
    }

    fn default_primitive(_: &Self::Channel) -> Self::Primitive {
        panic!("Blending is not applicable to ActiveHandle animation")
    }

    fn blend_method(&self, _: &Self::Channel) -> Option<BlendMethod> {
        None
    }
}
