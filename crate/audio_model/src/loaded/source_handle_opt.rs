use amethyst::{
    assets::Handle,
    audio::Source,
    ecs::{storage::DenseVecStorage, Component},
};
use derive_deref::{Deref, DerefMut};
use derive_new::new;

/// Newtype for an `Option<Handle<Source>>`, as we need to implement `Component`
/// on it.
#[derive(Clone, Component, Debug, Default, Deref, DerefMut, PartialEq, new)]
pub struct SourceHandleOpt(pub Option<Handle<Source>>);
