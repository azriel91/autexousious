use derivative::Derivative;

/// Status of augmenting character entities.
#[derive(Clone, Copy, Debug, Derivative, PartialEq, Eq)]
#[derivative(Default)]
pub enum CharacterAugmentStatus {
    /// Entities are being augmented with `Character` components.
    #[derivative(Default)]
    Prefab,
    /// Additional specific components / values are assigned to the entity.
    Rectify,
    /// Augmentation has been completed.
    Complete,
}
