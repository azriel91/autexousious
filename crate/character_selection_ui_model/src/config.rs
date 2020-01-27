//! Data types used for configuration.

pub use self::{
    character_selection_ui::CharacterSelectionUi, csw_definition::CswDefinition,
    csw_layer::CswLayer, csw_layer_name::CswLayerName, csw_template::CswTemplate,
};

mod character_selection_ui;
mod csw_definition;
mod csw_layer;
mod csw_layer_name;
mod csw_template;
