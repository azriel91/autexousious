use amethyst::{core::bundle::SystemBundle, ecs::DispatcherBuilder, Error};
use application_event::AppEventVariant;
use derive_new::new;
use game_input::InputToControlInputSystem;
use stdio_spi::MapperSystem;
use typename::TypeName;

use crate::ControlInputEventStdinMapper;

/// Adds a `MapperSystem<ControlInputEventStdinMapper>` to the `World`.
#[derive(Debug, new)]
pub struct GameInputStdioBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for GameInputStdioBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<(), Error> {
        builder.add(
            MapperSystem::<ControlInputEventStdinMapper>::new(AppEventVariant::ControlInput),
            &MapperSystem::<ControlInputEventStdinMapper>::type_name(),
            // TODO: Note: Depend on the input handler updated system, so that stdin input takes priority
            // &[&InputToControlInputSystem::type_name()],
            &[],
        ); // kcov-ignore
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::env;

    use amethyst::{input::InputBundle, renderer::ScreenDimensions, shrev::EventChannel};
    use amethyst_test::{prelude::*, HIDPI, SCREEN_HEIGHT, SCREEN_WIDTH};
    use game_input::GameInputBundle;
    use game_input_model::{InputConfig, PlayerActionControl, PlayerAxisControl};
    use stdio_spi::VariantAndTokens;

    use super::GameInputStdioBundle;

    #[test]
    fn bundle_should_add_mapper_system_to_dispatcher() {
        env::set_var("APP_DIR", env!("CARGO_MANIFEST_DIR"));
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::blank()
                .with_resource(ScreenDimensions::new(SCREEN_WIDTH, SCREEN_HEIGHT, HIDPI))
                .with_bundle(InputBundle::<PlayerAxisControl, PlayerActionControl>::new())
                .with_bundle(GameInputBundle::new(InputConfig::default()))
                .with_bundle(GameInputStdioBundle::new())
                // kcov-ignore-start
                .with_effect(|world| {
                    world.read_resource::<EventChannel<VariantAndTokens>>();
                })
                // kcov-ignore-end
                .run()
                .is_ok()
        );
    }
}
