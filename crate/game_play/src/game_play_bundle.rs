use amethyst::{
    core::bundle::{Result, SystemBundle},
    ecs::prelude::*,
};
use typename::TypeName;

use CharacterCollisionEffectSystem;
use CharacterGroundingSystem;
use CharacterKinematicsSystem;
use CharacterSequenceUpdateSystem;
use ObjectCollisionDetectionSystem;
use ObjectKinematicsUpdateSystem;
use ObjectTransformUpdateSystem;

/// Adds the object type update systems to the provided dispatcher.
#[derive(Debug, new)]
pub struct GamePlayBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for GamePlayBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        // Note: The `CharacterSequenceUpdateSystem` depends on
        // `game_input::ControllerInputUpdateSystem`. We rely on the main dispatcher to be run
        // before the `GamePlayState` dispatcher.
        builder.add(
            CharacterSequenceUpdateSystem::new(),
            &CharacterSequenceUpdateSystem::type_name(),
            &[],
        ); // kcov-ignore
        builder.add(
            CharacterKinematicsSystem::new(),
            &CharacterKinematicsSystem::type_name(),
            &[&CharacterSequenceUpdateSystem::type_name()],
        ); // kcov-ignore
        builder.add(
            ObjectKinematicsUpdateSystem::new(),
            &ObjectKinematicsUpdateSystem::type_name(),
            &[&CharacterKinematicsSystem::type_name()],
        ); // kcov-ignore
        builder.add(
            CharacterGroundingSystem::new(),
            &CharacterGroundingSystem::type_name(),
            &[&ObjectKinematicsUpdateSystem::type_name()],
        ); // kcov-ignore
        builder.add(
            ObjectTransformUpdateSystem::new(),
            &ObjectTransformUpdateSystem::type_name(),
            &[&CharacterGroundingSystem::type_name()],
        ); // kcov-ignore
        builder.add(
            ObjectCollisionDetectionSystem::new(),
            &ObjectCollisionDetectionSystem::type_name(),
            &[&ObjectTransformUpdateSystem::type_name()],
        ); // kcov-ignore
        builder.add(
            CharacterCollisionEffectSystem::new(),
            &CharacterCollisionEffectSystem::type_name(),
            &[&ObjectCollisionDetectionSystem::type_name()],
        ); // kcov-ignore
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::env;

    use amethyst_test::prelude::*;
    use game_input::{PlayerActionControl, PlayerAxisControl};

    use super::GamePlayBundle;

    #[test]
    fn bundle_build_should_succeed() {
        env::set_var("APP_DIR", env!("CARGO_MANIFEST_DIR"));

        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::ui_base::<PlayerAxisControl, PlayerActionControl>()
                .with_bundle(GamePlayBundle::new())
                .run()
                .is_ok()
        );
    }
}
