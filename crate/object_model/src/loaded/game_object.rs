use amethyst::assets::{Asset, Handle};
use sequence_model::config::SequenceId;

use crate::{
    config::{GameObjectDefinition, ObjectAssetData},
    loaded::ObjectWrapper,
    ObjectType,
};

/// Components common to object types, parameterized by sequence ID.
pub trait GameObject {
    /// The object type, such as `Character`.
    const OBJECT_TYPE: ObjectType;
    /// Sequence ID that this `GameObject` uses.
    type SequenceId: SequenceId;
    /// Type representing this `GameObject`'s configuration.
    type Definition: GameObjectDefinition<SequenceId = Self::SequenceId>
        + Asset<Data = Self::Definition>
        + Clone;
    /// Newtype wrapper for `Object<SequenceId>`.
    type ObjectWrapper: ObjectWrapper<SequenceId = Self::SequenceId>
        + Asset<Data = ObjectAssetData<Self::Definition>>;

    /// Returns the handle to the loaded `Object` for this `GameObject`.
    fn object_handle(&self) -> &Handle<Self::ObjectWrapper>;
}
