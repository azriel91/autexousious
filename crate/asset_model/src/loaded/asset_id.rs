use amethyst::ecs::{storage::VecStorage, Component};

slotmap::new_key_type! {
    /// Asset slug ID.
    ///
    /// This is a cheap `Copy` type to use instead of `AssetSlug` which is `Clone`.
    pub struct AssetId;
}

impl Component for AssetId {
    type Storage = VecStorage<Self>;
}
