use amethyst::ecs::World;
use game_model::{
    config::AssetSlug,
    loaded::{MapAssets, SlugAndHandle},
};
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
                    .read_resource::<MapAssets>()
                    .get(&slug)
                    .unwrap_or_else(|| panic!("Expected `{}` to be loaded.", slug))
                    .clone();

                SlugAndHandle::from((slug.clone(), map_handle))
            };

            world.add_resource(MapSelection::Id(slug_and_handle));
            world.add_resource(MapSelectionStatus::Confirmed);
        }
    }
}
