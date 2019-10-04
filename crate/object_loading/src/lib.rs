#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Processes object configuration into the loaded object model.

pub use crate::{
    object_loader::ObjectLoader, object_loader_params::ObjectLoaderParams,
    object_loader_system_data::ObjectLoaderSystemData, object_loading_status::ObjectLoadingStatus,
};

mod object_loader;
mod object_loader_params;
mod object_loader_system_data;
mod object_loading_status;
