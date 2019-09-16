use amethyst::ecs::{storage::VecStorage, Component};

slotmap::new_key_type! {
    /// Asset slug ID.
    ///
    /// This is a cheap `Copy` type to use instead of `AssetSlug` which is `Clone`.
    pub struct AssetSlugId;
}

impl Component for AssetSlugId {
    type Storage = VecStorage<Self>;
}
