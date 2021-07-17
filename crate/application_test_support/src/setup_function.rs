use amethyst::ecs::{World, WorldExt};
use asset_model::{config::AssetSlug, loaded::AssetIdMappings};
use map_selection::MapSelectionStatus;
use map_selection_model::MapSelection;

/// Provides common functions that simplify state set up.
#[derive(Debug)]
pub struct SetupFunction;

impl SetupFunction {
    /// Returns a function that adds a `MapSelection` and
    /// `MapSelectionStatus::Confirmed`.
    ///
    /// # Parameters
    ///
    /// * `slug`: Asset slug of the map to select.
    pub fn map_selection(slug: AssetSlug) -> impl Fn(&mut World) {
        move |world| {
            let map_asset_id = world
                .read_resource::<AssetIdMappings>()
                .id(&slug)
                .copied()
                .unwrap_or_else(|| panic!("Expected `{}` to be loaded.", slug));

            world.insert(MapSelection::Id(map_asset_id));
            world.insert(MapSelectionStatus::Confirmed);
        }
    }
}
