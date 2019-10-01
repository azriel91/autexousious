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

#[cfg(test)]
mod test {
    use amethyst::{ecs::WorldExt, shrev::EventChannel, Error};
    use amethyst_test::AmethystApplication;
    use game_play_model::GamePlayEvent;

    use super::GamePlayStdioBundle;

    #[test]
    fn bundle_should_add_mapper_system_to_dispatcher() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_bundle(GamePlayStdioBundle::new())
            // kcov-ignore-start
            .with_effect(|world| {
                world.read_resource::<EventChannel<GamePlayEvent>>();
            })
            // kcov-ignore-end
            .run()
    }
}
