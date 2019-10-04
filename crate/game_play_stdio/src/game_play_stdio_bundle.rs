use amethyst::{
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, World},
    Error,
};
use application_event::AppEventVariant;
use derive_new::new;
use stdio_spi::MapperSystem;
use typename::TypeName;

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
            &MapperSystem::<GamePlayEventStdinMapper>::type_name(),
            &[],
        ); // kcov-ignore
        Ok(())
    }
}
