use amethyst::{
    core::bundle::{Result, SystemBundle},
    ecs::DispatcherBuilder,
};
use typename::TypeName;

use RunCounterUpdateSystem;

/// Adds the object type update systems to the provided dispatcher.
#[derive(Debug, new)]
pub struct ObjectPlayBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for ObjectPlayBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        builder.add(
            RunCounterUpdateSystem::new(),
            &RunCounterUpdateSystem::type_name(),
            &[],
        ); // kcov-ignore

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::env;

    use amethyst_test::prelude::*;
    use game_input::{PlayerActionControl, PlayerAxisControl};

    use super::ObjectPlayBundle;

    #[test]
    fn bundle_build_should_succeed() {
        env::set_var("APP_DIR", env!("CARGO_MANIFEST_DIR"));

        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::ui_base::<PlayerAxisControl, PlayerActionControl>()
                .with_bundle(ObjectPlayBundle::new())
                .run()
                .is_ok()
        );
    }
}
