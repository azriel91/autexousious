use std::any;

use amethyst::{
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, World},
    Error,
};
use application_event::AppEventVariant;
use derive_new::new;
use stdio_spi::MapperSystem;

use crate::GamePlayEventStdinMapper;

/// Adds a `MapperSystem<GamePlayEventStdinMapper>` to the `World`.
#[derive(Debug, new)]
pub struct GamePlayStdioBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for GamePlayStdioBundle {
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        builder.add(
            MapperSystem::<GamePlayEventStdinMapper>::new(AppEventVariant::GamePlay),
            any::type_name::<MapperSystem<GamePlayEventStdinMapper>>(),
            &[],
        ); // kcov-ignore
        Ok(())
    }
}
