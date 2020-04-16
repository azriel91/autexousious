use amethyst::{
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, World},
    Error,
};
use derive_new::new;

use crate::{FontConfig, ThemeLoader};

/// Loads the `Theme` into the `World`.
#[derive(Debug, new)]
pub struct ApplicationUiBundle {
    /// The `FontConfig` to build the theme from.
    font_config: FontConfig,
}

impl<'a, 'b> SystemBundle<'a, 'b> for ApplicationUiBundle {
    fn build(
        self,
        world: &mut World,
        _builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        ThemeLoader::load(world, self.font_config)
    }
}
