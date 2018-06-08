use amethyst::ecs::{prelude::*, storage::DenseVecStorage};

/// Status of an object entity.
///
/// We use a `DenseVecStorage` because all object entities have their own type of `SequenceId`.
#[derive(Constructor, Debug)]
pub struct ObjectStatus<SeqId> {
    /// ID of the current sequence the entity is on.
    pub sequence_id: SeqId,
}

impl<SeqId: 'static + Send + Sync> Component for ObjectStatus<SeqId> {
    type Storage = DenseVecStorage<Self>;
}
