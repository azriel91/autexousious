//! Data types used for configuration.

pub use self::{
    map_selection_ui::MapSelectionUi, msw_layer::MswLayer, msw_layer_name::MswLayerName,
    msw_portraits::MswPortraits, msw_template::MswTemplate,
};

mod map_selection_ui;
mod msw_layer;
mod msw_layer_name;
mod msw_portraits;
mod msw_template;
