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
    }
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
