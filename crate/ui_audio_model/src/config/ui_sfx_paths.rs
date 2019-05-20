use std::{collections::HashMap, path::PathBuf};

use amethyst::{
    assets::{Asset, Handle, ProcessingState},
    ecs::storage::VecStorage,
    Error,
};
use derive_deref::{Deref, DerefMut};
use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::config::UiSfxId;

/// Map of `UiSfxId` to the path of the SFX file.
#[derive(Debug, Default, Deref, DerefMut, Deserialize, PartialEq, Eq, Serialize, new)]
#[serde(deny_unknown_fields, transparent)]
pub struct UiSfxPaths(HashMap<UiSfxId, PathBuf>);

impl Asset for UiSfxPaths {
    const NAME: &'static str = concat!(module_path!(), "::", stringify!(UiSfxPaths));
    type Data = Self;
    type HandleStorage = VecStorage<Handle<Self>>;
}

impl From<UiSfxPaths> for Result<ProcessingState<UiSfxPaths>, Error> {
    fn from(ui_sfx_paths: UiSfxPaths) -> Result<ProcessingState<UiSfxPaths>, Error> {
        Ok(ProcessingState::Loaded(ui_sfx_paths))
    }
}
