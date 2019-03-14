use amethyst::{core::bundle::SystemBundle, ecs::DispatcherBuilder, Error};
use derive_new::new;
use typename::TypeName;

use crate::HpBarUpdateSystem;

/// Adds game play HUD systems to the provided dispatcher.
#[derive(Debug, new)]
pub struct GamePlayHudBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for GamePlayHudBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<(), Error> {
        builder.add(
            HpBarUpdateSystem::new(),
            &HpBarUpdateSystem::type_name(),
            &[],
        );
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::env;

    use amethyst::Error;
    use amethyst_test::AmethystApplication;

    use super::GamePlayHudBundle;

    #[test]
    fn bundle_build_should_succeed() -> Result<(), Error> {
        env::set_var("APP_DIR", env!("CARGO_MANIFEST_DIR"));

        AmethystApplication::blank()
            .with_bundle(GamePlayHudBundle::new())
            .run()
    }
}
