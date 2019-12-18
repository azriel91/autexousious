use amethyst::ecs::{World, WorldExt};
use derivative::Derivative;
use derive_deref::{Deref, DerefMut};

/// `World` for assets.
///
/// Strong type makes it clearer that this is a separate `World` instance.
#[derive(Derivative, Deref, DerefMut)]
#[derivative(Debug)]
pub struct AssetWorld(#[derivative(Debug = "ignore")] pub World);

impl Default for AssetWorld {
    fn default() -> Self {
        AssetWorld(World::new())
    }
}
