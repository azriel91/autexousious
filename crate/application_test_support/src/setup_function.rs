use amethyst::ecs::WorldExt; use amethyst::ecs::World;
use asset_model::{config::AssetSlug, loaded::SlugAndHandle};
use game_model::loaded::MapPrefabs;
use map_selection::MapSelectionStatus;
use map_selection_model::MapSelection;

/// Provides common functions that simplify state set up.
#[derive(Debug)]
pub struct SetupFunction;

impl SetupFunction {
    /// Returns a function that adds a `MapSelection` and `MapSelectionStatus::Confirmed`.
    ///
    /// # Parameters
    ///
    /// * `slug`: Asset slug of the map to select.
    pub fn map_selection(slug: AssetSlug) -> impl Fn(&mut World) {
        move |world| {
            let slug_and_handle = {
                let map_handle = world
                    .read_resource::<MapPrefabs>()
                    .get(&slug)
                    .unwrap_or_else(|| panic!("Expected `{}` to be loaded.", slug))
                    .clone();

                SlugAndHandle::from((slug.clone(), map_handle))
            };

            world.insert(MapSelection::Id(slug_and_handle));
            world.insert(MapSelectionStatus::Confirmed);
        }
    }
}
