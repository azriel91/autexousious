use amethyst::ecs::{storage::VecStorage, Component};
use derive_deref::{Deref, DerefMut};
use derive_more::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Display, From, Not,
};
use derive_new::new;

/// Whether the object is mirrored.
#[derive(
    BitAnd,
    BitAndAssign,
    BitOr,
    BitOrAssign,
    BitXor,
    BitXorAssign,
    Clone,
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
    new,
)]
pub struct Mirrored(pub bool);

/// Not every entity will have this, but since it's simply a `bool` wrapper, the indirection table
/// actually uses more space.
impl Component for Mirrored {
    type Storage = VecStorage<Self>;
}
