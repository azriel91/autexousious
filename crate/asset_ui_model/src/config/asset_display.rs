use std::marker::PhantomData;

use derive_new::new;
use kinematic_model::config::PositionInit;
use serde::{Deserialize, Serialize};

use crate::config::AssetDisplayLayout;

/// Display sheet for available assets.
///
/// # Type Parameters
///
/// * `T`: Type to indicate the assets to display.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, new)]
#[serde(deny_unknown_fields)]
pub struct AssetDisplay<T> {
    /// Position of the sheet.
    pub position: PositionInit,
    /// How to layout the available assets.
    pub layout: AssetDisplayLayout,
    /// Marker.
    #[serde(skip)]
    pub marker: PhantomData<T>,
}
