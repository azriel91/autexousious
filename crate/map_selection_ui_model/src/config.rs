//! Data types used for configuration.

pub use self::{
    map_selection_ui::MapSelectionUi, mpw_template::MpwTemplate, msw_layer::MswLayer,
    msw_layer_name::MswLayerName, msw_portraits::MswPortraits,
};

mod map_selection_ui;
mod mpw_template;
mod msw_layer;
mod msw_layer_name;
mod msw_portraits;
