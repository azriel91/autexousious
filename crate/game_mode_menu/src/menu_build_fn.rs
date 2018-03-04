use std::fmt::{Debug, Error, Formatter};
use std::ops::{Deref, DerefMut};

use amethyst::ecs::Entity;
use amethyst::prelude::World;
use amethyst::renderer::ScreenDimensions;
use amethyst::ui::{FontHandle, MouseReactive, UiResize, UiText, UiTransform};
use application_menu::MenuItem;

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

        // TODO: Use UI Buttons: https://github.com/amethyst/amethyst/issues/577
        let mut item_indices = vec![Index::StartGame, Index::Exit];
        item_indices
            .drain(..)
            .enumerate()
            .for_each(|(order, index)| {
                let mut text_transform = UiTransform::new(
                    index.title().to_string(),
                    20.,
                    order as f32 * 50. + 20.,
                    1.,
                    400.,
                    100.,
                    0,
                );
                let ui_text_size_fn = |_transform: &mut UiTransform, (_width, _height)| {};

                {
                    let dim = world.read_resource::<ScreenDimensions>();
                    ui_text_size_fn(&mut text_transform, (dim.width(), dim.height()));
                }

                let menu_item_entity = world
                    .create_entity()
                    .with(text_transform)
                    .with(UiText::new(
                        font_bold.clone(),
                        index.title().to_string(),
                        [1., 1., 1., 1.],
                        FONT_SIZE,
                    ))
                    .with(UiResize(Box::new(ui_text_size_fn)))
                    .with(MouseReactive)
                    .with(MenuItem { index })
                    .build();

                menu_items.push(menu_item_entity);
            });
    }

    fn read_font(world: &mut World) -> FontHandle {
        use application_ui::FontVariant::Bold;
        world
            .read_resource_with_id::<FontHandle>(Bold.into())
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
    use amethyst::input::InputBundle;
    use amethyst::prelude::*;
    use amethyst::renderer::{DisplayConfig, Pipeline, RenderBundle, Stage};
    use amethyst::Result;
    use amethyst::ui::{DrawUi, UiBundle};
    use application::resource::dir;
    use application::resource::find_in;
    use application_ui::ApplicationUiBundle;

    use bundle::Bundle;
    use super::MenuBuildFn;

    fn setup<'a, 'b>() -> Result<Application<'a, 'b>> {
        let display_config = DisplayConfig::load(
            find_in(
                dir::RESOURCES,
                "display_config.ron",
                Some(development_base_dirs!()),
            ).unwrap(),
        );

        let pipe = Pipeline::build().with_stage(
            Stage::with_backbuffer()
                .clear_target([0., 0., 0., 1.], 1.)
                .with_pass(DrawUi::new()),
        );

        // We need to instantiate an amethyst::Application because:
        //
        // * The `Loader` needs to be added to the world, and the code to do this is non-trivial
        // * The `AppBundle` in amethyst that does this is non-public
        Application::build(format!("{}/assets", env!("CARGO_MANIFEST_DIR")), MockState)?
            .with_bundle(InputBundle::<String, String>::new())?
            .with_bundle(UiBundle::<String, String>::new())?
            // needed for ScreenDimensions
            .with_bundle(RenderBundle::new(pipe, Some(display_config)))?
            .with_bundle(ApplicationUiBundle::new())?
            .with_bundle(Bundle)?
            .build()
    } // kcov-ignore

    #[test]
    fn initialize_menu_creates_entity_for_each_menu_index() {
        let mut app = setup().expect("Failed to build Application.");
        let mut menu_items = vec![];

        let mut mb_fn = MenuBuildFn::default();
        (&mut *mb_fn)(&mut app.world, &mut menu_items);

        assert_eq!(2, menu_items.len());
    }

    #[derive(Debug)]
    struct MockState;
    impl State for MockState {}
}
