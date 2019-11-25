use amethyst::ecs::{
    shred::{ResourceId, SystemData},
    storage::VecStorage,
    Component, Entity, World, WriteStorage,
};
use asset_model::ItemComponent;
use derivative::Derivative;
use derive_deref::{Deref, DerefMut};
use derive_more::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Display, From, Not,
};
use derive_new::new;
use typename_derive::TypeName;

/// Whether the object is mirrored.
#[derive(
    BitAnd,
    BitAndAssign,
    BitOr,
    BitOrAssign,
    BitXor,
    BitXorAssign,
    Clone,
    Component,
    Copy,
    Debug,
    Deref,
    DerefMut,
    Default,
    Display,
    From,
    PartialEq,
    Eq,
    Not,
    TypeName,
    new,
)]
#[storage(VecStorage)]
pub struct Mirrored(pub bool);

/// `MirroredSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct MirroredSystemData<'s> {
    /// `Mirrored` components.
    #[derivative(Debug = "ignore")]
    pub mirroreds: WriteStorage<'s, Mirrored>,
}

impl<'s> ItemComponent<'s> for Mirrored {
    type SystemData = MirroredSystemData<'s>;

    fn augment(&self, system_data: &mut Self::SystemData, entity: Entity) {
        let MirroredSystemData { mirroreds } = system_data;

        mirroreds
            .insert(entity, *self)
            .expect("Failed to insert `Mirrored` component.");
    }
}
