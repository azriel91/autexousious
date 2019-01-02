use std::collections::HashMap;

use amethyst::{
    assets::{Asset, Handle},
    ecs::{
        storage::{DenseVecStorage, VecStorage},
        Component,
    },
};
use derivative::Derivative;
use derive_new::new;

use crate::{
    config::object::SequenceId,
    loaded::{AnimatedComponentAnimation, AnimatedComponentDefault},
};

/// Represents an in-game object that has been loaded.
#[derive(Clone, Derivative, PartialEq, new)]
#[derivative(Debug)]
pub struct Object<SeqId>
where
    SeqId: SequenceId,
{
    /// Handle to the default sprite sheet to use for the object.
    pub animation_defaults: Vec<AnimatedComponentDefault>,
    /// Handles to the animations that this object uses, keyed by sequence ID.
    pub animations: HashMap<SeqId, Vec<AnimatedComponentAnimation>>,
}

impl<SeqId> Asset for Object<SeqId>
where
    SeqId: SequenceId + 'static,
{
    const NAME: &'static str = "object_model::loaded::Object";
    type Data = Self;
    type HandleStorage = VecStorage<Handle<Self>>;
}

impl<SeqId> Component for Object<SeqId>
where
    SeqId: SequenceId + 'static,
{
    type Storage = DenseVecStorage<Self>;
}

// We are unable to implement `From<SeqId>` for any `Object<SeqId>` because `SeqId` is a type
// parameter, and the orphan rule prevents us from implementing the trait as the concrete type is
// outside this crate.

// === Orphan rule blocked implementation === //
//
// impl<SeqId> From<Object<SeqId>> for Result<ProcessingState<Object<SeqId>>, Error>
// where
//     SeqId: SequenceId,
// {
//     fn from(object: Object<SeqId>) -> Result<ProcessingState<Object<SeqId>>, Error> {
//         Ok(ProcessingState::Loaded(object))
//     }
// }

// === Hack attempt === //
//
// /// Placeholder trait to implement `From<SeqId>` for any `Object<SeqId>`.
// trait Processable {
//     type SequenceId;
//     type ProcessingState;
// }
//
// impl<SeqId> Processable for Object<SeqId>
// where
//     SeqId: SequenceId,
// {
//     type SequenceId = SeqId;
//     type ProcessingState = ProcessingState<Self>;
// }
//
// impl<P, SeqId> From<P> for Result<ProcessingState<P>, Error>
// where
//     P: Processable<SequenceId = SeqId>,
//     SeqId: SequenceId,
// {
//     fn from(processable: P) -> Result<ProcessingState<P>, Error> {
//         Ok(ProcessingState::Loaded(processable))
//     }
// }

#[macro_export]
macro_rules! impl_processing_state_from_object {
    ($obj:ty) => {
        use amethyst::{
            assets::{Asset, Handle},
            ecs::storage::VecStorage,
        };
        impl Asset for $obj {
            const NAME: &'static str = concat!(module_path!(), "::", stringify!($obj));
            type Data = Self;
            type HandleStorage = VecStorage<Handle<Self>>;
        }

        use amethyst::assets::{Error, ProcessingState};
        impl From<$obj> for std::result::Result<ProcessingState<$obj>, Error> {
            fn from(object: $obj) -> std::result::Result<ProcessingState<$obj>, Error> {
                Ok(ProcessingState::Loaded(object))
            }
        }
    };
}

/// Handle to an Object
pub type ObjectHandle<SeqId> = Handle<Object<SeqId>>;
