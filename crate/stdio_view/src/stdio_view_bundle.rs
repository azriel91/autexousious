use amethyst::{
    core::bundle::{Result, SystemBundle},
    ecs::prelude::*,
};

use StdinSystem;

/// Adds the `StdinSystem` to the `World` with id `"stdin_system"`.
///
/// The Amethyst `InputBundle` must be added before this bundle.
#[derive(Debug, new)]
pub struct StdioViewBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for StdioViewBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        builder.add(StdinSystem::new(), "stdin_system", &[]);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::env;

    use amethyst::{prelude::*, shrev::EventChannel, Result};
    use application_input::ApplicationEvent;

    use super::StdioViewBundle;

    fn setup<'a, 'b>() -> Result<Application<'a, GameData<'a, 'b>>> {
        env::set_var("APP_DIR", env!("CARGO_MANIFEST_DIR"));
        let game_data = GameDataBuilder::default().with_bundle(StdioViewBundle)?;
        let app = Application::new(
            format!("{}/assets", env!("CARGO_MANIFEST_DIR")),
            MockState,
            game_data,
        )?;

        Ok(app)
    } // kcov-ignore

    #[test]
    fn bundle_should_add_stdin_system_to_dispatcher() {
        setup()
            .expect("StdioViewBundle#build() should succeed")
            .run(); // kcov-ignore
    }

    #[derive(Debug)]
    struct MockState;
    impl<'a, 'b> State<GameData<'a, 'b>> for MockState {
        fn update(&mut self, data: StateData<GameData>) -> Trans<GameData<'a, 'b>> {
            data.data.update(&data.world);

            data.world.read_resource::<EventChannel<ApplicationEvent>>();

            Trans::Quit
        }
    }
}
