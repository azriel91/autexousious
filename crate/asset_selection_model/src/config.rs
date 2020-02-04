//! Types representing asset selection configuration.

pub use self::{
    asset_selection_event_args::AssetSelectionEventArgs,
    asset_selection_event_command::AssetSelectionEventCommand, asset_switch::AssetSwitch,
};

mod asset_selection_event_args;
mod asset_selection_event_command;
mod asset_switch;
