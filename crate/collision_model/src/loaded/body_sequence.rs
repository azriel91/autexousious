use std::marker::PhantomData;

use amethyst::{
    assets::{Asset, Handle},
    ecs::storage::VecStorage,
};
use derive_new::new;

use crate::{animation::BodyAnimationSequence, config::BodyFrame};

/// Sequence for volumes that can be hit.
#[derive(Clone, Debug, Default, PartialEq, new)]
pub struct BodySequence<S>
where
    S: BodyAnimationSequence,
{
    /// Handles to the frames in this sequence.
    pub frames: Vec<Handle<BodyFrame>>,
    /// Marker.
    phantom_data: PhantomData<S>,
}

impl<S> Asset for BodySequence<S>
where
    S: BodyAnimationSequence + Send + Sync + 'static,
{
    const NAME: &'static str = "collision_model::loaded::BodySequence";
    type Data = S;
    type HandleStorage = VecStorage<Handle<Self>>;
}
