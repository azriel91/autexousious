//! Data types used for configuration.

pub use self::{
    character_selection_ui::CharacterSelectionUi, csw_definition::CswDefinition,
    csw_portraits::CswPortraits, csw_template::CswTemplate,
};

mod character_selection_ui;
mod csw_definition;
mod csw_portraits;
mod csw_template;
