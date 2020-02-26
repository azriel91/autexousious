//! Data types used in configuration.

pub use self::{
    dimensions::Dimensions, widget_status::WidgetStatus,
    widget_status_sequences::WidgetStatusSequences,
};

mod dimensions;
mod widget_status;
mod widget_status_sequences;
