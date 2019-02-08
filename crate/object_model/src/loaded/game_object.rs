use amethyst::assets::{Asset, Handle};

use crate::{
    config::{object::SequenceId, GameObjectDefinition, ObjectAssetData},
    loaded::ObjectWrapper,
};

/// Components common to object types, parameterized by sequence ID.
pub trait GameObject {
    /// Sequence ID that this `GameObject` uses.
    type SequenceId: SequenceId;
    /// Type representing this `GameObject`'s configuration.
    type Definition: GameObjectDefinition<SequenceId = Self::SequenceId>
        + Asset<Data = Self::Definition>;
    /// Newtype wrapper for `Object<SequenceId>`.
    type ObjectWrapper: ObjectWrapper<SequenceId = Self::SequenceId>
        + Asset<Data = ObjectAssetData<Self::Definition>>;

    /// Returns the handle to the loaded `Object` for this `GameObject`.
    fn object_handle(&self) -> &Handle<Self::ObjectWrapper>;
}
