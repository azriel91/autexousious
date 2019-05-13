use std::{collections::HashMap, path::PathBuf};

use amethyst::{
    assets::{Asset, Handle, ProcessingState},
    ecs::storage::VecStorage,
    Error,
};
use derive_deref::{Deref, DerefMut};
use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::config::CollisionSfxId;

/// Map of `CollisionSfxId` to the path of the SFX file.
#[derive(Debug, Default, Deref, DerefMut, Deserialize, PartialEq, Eq, Serialize, new)]
#[serde(deny_unknown_fields, transparent)]
pub struct CollisionSfxPaths(HashMap<CollisionSfxId, PathBuf>);

impl Asset for CollisionSfxPaths {
    const NAME: &'static str = concat!(module_path!(), "::", stringify!(CollisionSfxPaths));
    type Data = Self;
    type HandleStorage = VecStorage<Handle<Self>>;
}

impl From<CollisionSfxPaths> for Result<ProcessingState<CollisionSfxPaths>, Error> {
    fn from(
        collision_sfx_paths: CollisionSfxPaths,
    ) -> Result<ProcessingState<CollisionSfxPaths>, Error> {
        Ok(ProcessingState::Loaded(collision_sfx_paths))
    }
}
