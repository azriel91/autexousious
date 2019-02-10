use std::{
    error,
    fmt::{self, Debug},
};

use amethyst::assets::Handle;
use object_model::loaded::ObjectWrapper;

/// Error while using `ObjectPrefab`.
#[derive(Clone, Debug, PartialEq)]
pub enum ObjectPrefabError<W>
where
    W: Debug + ObjectWrapper + Send + Sync + 'static,
{
    /// `ObjectWrapper` for the given handle was not loaded.
    NotLoaded {
        /// Handle to the `ObjectWrapper`.
        object_wrapper_handle: Handle<W>,
    },
}

impl<W> fmt::Display for ObjectPrefabError<W>
where
    W: Debug + ObjectWrapper + Send + Sync + 'static,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ObjectPrefabError::NotLoaded {
                object_wrapper_handle,
            } => writeln!(
                f,
                "`ObjectWrapper` not loaded for handle: `{:?}`.",
                object_wrapper_handle
            ),
        }
    }
}

impl<W> error::Error for ObjectPrefabError<W> where W: Debug + ObjectWrapper + Send + Sync + 'static {}
