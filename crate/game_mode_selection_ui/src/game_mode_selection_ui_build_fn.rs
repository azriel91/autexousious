use std::ops::{Deref, DerefMut};

use amethyst::{
    assets::{AssetStorage, Loader},
    ecs::prelude::*,
    renderer::{ScreenDimensions, Texture},
    ui::{Anchor, UiButtonBuilder},
};
use application_menu::MenuItem;
use application_ui::{FontVariant, Theme};
use game_mode_selection_model::{
    GameModeIndex, GameModeSelectionEntity, GameModeSelectionEntityId,
};
use heck::TitleCase;
use strum::IntoEnumIterator;

/// Wraps a `FnMut(&mut World)`.
///
/// This allows types needing this function to have a known size at compile time.
///
/// This also implements `Debug` to allow consumers to easily derive `Debug`, though the current
/// implementation does not show what the function does.
#[derive(Derivative)]
#[derivative(Debug)]
pub struct GameModeSelectionUiBuildFn<F>(F)
where
    F: FnMut(&mut World);

const FONT_SIZE: f32 = 25.;

impl GameModeSelectionUiBuildFn<fn(&mut World)> {
    /// Returns a `GameModeSelectionUiBuildFn` to build the `GameModeSelectionUi`.
    pub fn new() -> Self {
        Default::default()
    }
}

impl Default for GameModeSelectionUiBuildFn<fn(&mut World)> {
    fn default() -> Self {
        GameModeSelectionUiBuildFn(Self::initialize_ui)
    }
}

impl<F> GameModeSelectionUiBuildFn<F>
where
    F: FnMut(&mut World),
{
    fn initialize_ui(world: &mut World) {
        world.register::<MenuItem<GameModeIndex>>();

        let font = world
            .read_resource::<Theme>()
            .fonts
            .get(&FontVariant::Bold)
            .expect("Failed to get Bold font handle")
            .clone();

        let text_w = world.read_resource::<ScreenDimensions>().width() / 3.;
        let text_h = 50.;

        // Background colour
        let black_bg = {
            let loader = world.read_resource::<Loader>();
            loader.load_from_data(
                [0.; 4].into(),
                (),
                &world.read_resource::<AssetStorage<Texture>>(),
            )
        };

        let item_count = GameModeIndex::iter().len();
        GameModeIndex::iter()
            .enumerate()
            .for_each(|(order, index)| {
                let index_id = index.to_string();
                let index_text = index_id.to_title_case();
                let entity = UiButtonBuilder::new(index_id, index_text)
                    .with_position(
                        0.,
                        ((item_count - order) as f32 * text_h) - (item_count as f32 * text_h / 2.),
                    ).with_text_color([0.7; 4])
                    .with_image(black_bg.clone())
                    .with_hover_text_color([1.; 4])
                    .with_press_text_color([0.5; 4])
                    .with_font_size(FONT_SIZE)
                    .with_size(text_w, text_h)
                    .with_tab_order(order as i32)
                    .with_anchor(Anchor::Middle)
                    .with_font(font.clone())
                    .build_from_world(world);

                world
                    .write_storage::<MenuItem<GameModeIndex>>()
                    .insert(entity, MenuItem { index })
                    // kcov-ignore-start
                    .unwrap_or_else(|e| {
                        panic!(
                            "Failed to insert {} component. Error: `{}`.",
                            stringify!(MenuItem<GameModeIndex>),
                            e
                        )
                    });
                // kcov-ignore-end

                world
                    .write_storage::<GameModeSelectionEntity>()
                    .insert(
                        entity,
                        GameModeSelectionEntity::new(GameModeSelectionEntityId),
                    )
                    // kcov-ignore-start
                    .unwrap_or_else(|e| {
                        panic!(
                            "Failed to insert {} component. Error: `{}`.",
                            stringify!(GameModeSelectionEntity),
                            e
                        )
                    });
                // kcov-ignore-end
            });
    }
}

impl<F> Deref for GameModeSelectionUiBuildFn<F>
where
    F: FnMut(&mut World),
{
    type Target = F;

    fn deref(&self) -> &F {
        &self.0
    }
}

impl<F> DerefMut for GameModeSelectionUiBuildFn<F>
where
    F: FnMut(&mut World),
{
    fn deref_mut(&mut self) -> &mut F {
        &mut self.0
    }
}

#[cfg(test)]
mod test {
    use std::env;

    use amethyst::{ecs::Join, prelude::*};
    use amethyst_test_support::prelude::*;
    use application_ui::ThemeLoader;
    use game_mode_selection_model::GameModeSelectionEntity;

    use super::GameModeSelectionUiBuildFn;

    #[test]
    fn initialize_ui_creates_entity_for_each_menu_index() {
        env::set_var("APP_DIR", env!("CARGO_MANIFEST_DIR"));

        let assertion = |world: &mut World| {
            ThemeLoader::load(world).expect("Failed to load theme.");

            let mut ui_build_fn = GameModeSelectionUiBuildFn::new();
            (&mut *ui_build_fn)(world);

            assert_eq!(
                2,
                world
                    .read_storage::<GameModeSelectionEntity>()
                    .join()
                    .count()
            );
        };

        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::ui_base::<String, String>()
                .with_setup(|world| {
                    world.register::<GameModeSelectionEntity>();
                }).with_assertion(assertion)
                .run()
                .is_ok()
        );
    }
}
