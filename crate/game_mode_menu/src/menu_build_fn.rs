use std::fmt::{Debug, Error, Formatter};
use std::ops::{Deref, DerefMut};

use amethyst::ecs::prelude::*;
use amethyst::prelude::World;
use amethyst::renderer::ScreenDimensions;
use amethyst::ui::{Anchor, FontHandle, MouseReactive, UiText, UiTransform};
use application_menu::MenuItem;
use application_ui::{FontVariant, Theme};

use index::Index;

/// Wraps a `FnMut(&mut World)` in a `Box`.
///
/// This allows types needing this function to have a known size at compile time.
///
/// This also implements `Debug` to allow consumers to easily derive `Debug`, though the current
/// implementation does not show what the function does.
pub struct MenuBuildFn(pub Box<FnMut(&mut World, &mut Vec<Entity>)>);

const FONT_SIZE: f32 = 25.;

impl MenuBuildFn {
    fn initialize_menu(world: &mut World, menu_items: &mut Vec<Entity>) {
        let font_bold = Self::read_font(world);

        let screen_w = {
            let dim = world.read_resource::<ScreenDimensions>();
            dim.width()
        };
        let text_w = screen_w / 3.;
        let text_h = 50.;

        // TODO: Use UI Buttons: https://github.com/amethyst/amethyst/issues/577
        let mut item_indices = vec![Index::StartGame, Index::Exit];
        let total_items = item_indices.len() as f32;
        item_indices
            .drain(..)
            .enumerate()
            .for_each(|(order, index)| {
                let text_transform = UiTransform::new(
                    index.title().to_string(),
                    Anchor::Middle,
                    0.,
                    (order as f32 * text_h) - (total_items * text_h / 2.),
                    1.,
                    text_w,
                    text_h,
                    0,
                );

                let menu_item_entity = world
                    .create_entity()
                    .with(text_transform)
                    .with(UiText::new(
                        font_bold.clone(),
                        index.title().to_string(),
                        [1., 1., 1., 1.],
                        FONT_SIZE,
                    ))
                    .with(MouseReactive)
                    .with(MenuItem { index })
                    .build();

                menu_items.push(menu_item_entity);
            });
    }

    fn read_font(world: &mut World) -> FontHandle {
        let theme = world.read_resource::<Theme>();
        theme
            .fonts
            .get(&FontVariant::Bold)
            .expect("Failed to get Bold font handle")
            .clone()
    } // kcov-ignore
}

impl Debug for MenuBuildFn {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "MenuBuildFn(\"..\")")
    }
}

impl Default for MenuBuildFn {
    fn default() -> Self {
        MenuBuildFn(Box::new(Self::initialize_menu))
    }
}

impl Deref for MenuBuildFn {
    type Target = Box<FnMut(&mut World, &mut Vec<Entity>)>;

    fn deref(&self) -> &Box<FnMut(&mut World, &mut Vec<Entity>)> {
        &self.0
    }
}

impl DerefMut for MenuBuildFn {
    fn deref_mut(&mut self) -> &mut Box<FnMut(&mut World, &mut Vec<Entity>)> {
        &mut self.0
    }
}

#[cfg(test)]
mod test {
    use std::env;

    use amethyst::core::transform::TransformBundle;
    use amethyst::input::InputBundle;
    use amethyst::prelude::*;
    use amethyst::renderer::ScreenDimensions;
    use amethyst::ui::UiBundle;
    use amethyst::Result;
    use application_ui::ThemeLoader;

    use super::MenuBuildFn;
    use GameModeMenuBundle;

    fn setup<'a, 'b, F>(assertion_fn: Box<F>) -> Result<Application<'a, 'b>>
    where
        F: 'a + Fn(&mut World),
    {
        // We need to instantiate an amethyst::Application because:
        //
        // * The `Loader` needs to be added to the world, and the code to do this is non-trivial
        // * The `AppBundle` in amethyst that does this is non-public
        env::set_var("APP_DIR", env!("CARGO_MANIFEST_DIR"));
        let mut app = Application::build(
            format!("{}/assets", env!("CARGO_MANIFEST_DIR")),
            MockState { assertion_fn },
        )?.with_bundle(TransformBundle::new())?
            .with_bundle(InputBundle::<String, String>::new())?
            .with_bundle(UiBundle::<String, String>::new())?
            .with_bundle(GameModeMenuBundle)?
            .with_resource(ScreenDimensions::new(640, 480))
            .build()?;

        ThemeLoader::load(&mut app.world)?;

        Ok(app)
    } // kcov-ignore

    #[test]
    fn initialize_menu_creates_entity_for_each_menu_index() {
        let assertion_fn = |world: &mut World| {
            let mut menu_items = vec![];

            let mut mb_fn = MenuBuildFn::default();
            (&mut *mb_fn)(world, &mut menu_items);

            assert_eq!(2, menu_items.len());
        };
        setup(Box::new(assertion_fn))
            .expect("Failed to build Application.")
            .run(); // kcov-ignore
    }

    #[derive(Debug)]
    struct MockState<F: Fn(&mut World)> {
        assertion_fn: Box<F>,
    }
    impl<F: Fn(&mut World)> State for MockState<F> {
        fn on_start(&mut self, world: &mut World) {
            (self.assertion_fn)(world);
        }

        fn fixed_update(&mut self, _world: &mut World) -> Trans {
            Trans::Quit
        }
    }
}
