use amethyst::{
    core::bundle::{Result, SystemBundle},
    ecs::prelude::*,
};
use typename::TypeName;

use CharacterGroundingSystem;
use CharacterInputUpdateSystem;
use CharacterKinematicsSystem;
use CharacterSequenceUpdateSystem;
use ObjectKinematicsUpdateSystem;
use ObjectTransformUpdateSystem;

/// Adds the object type update systems to the provided dispatcher.
///
/// The Amethyst `InputBundle` must be added before this bundle.
#[derive(Debug, new)]
pub struct GamePlayBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for GamePlayBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        builder.add(
            CharacterInputUpdateSystem::new(),
            &CharacterInputUpdateSystem::type_name(),
            // TODO: Pending <https://gitlab.com/azriel91/autexousious/issues/53>
            &["input_system"],
        );
        builder.add(
            CharacterSequenceUpdateSystem::new(),
            &CharacterSequenceUpdateSystem::type_name(),
            &[&CharacterInputUpdateSystem::type_name()],
        );
        builder.add(
            CharacterKinematicsSystem::new(),
            &CharacterKinematicsSystem::type_name(),
            &[&CharacterSequenceUpdateSystem::type_name()],
        );
        builder.add(
            ObjectKinematicsUpdateSystem::new(),
            &ObjectKinematicsUpdateSystem::type_name(),
            &[&CharacterKinematicsSystem::type_name()],
        );
        builder.add(
            CharacterGroundingSystem::new(),
            &CharacterGroundingSystem::type_name(),
            &[&ObjectKinematicsUpdateSystem::type_name()],
        );
        builder.add(
            ObjectTransformUpdateSystem::new(),
            &ObjectTransformUpdateSystem::type_name(),
            &[&CharacterGroundingSystem::type_name()],
        );
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::env;

    use amethyst::{core::transform::TransformBundle, input::InputBundle, ui::UiBundle};
    use amethyst_test_support::prelude::*;
    use game_input::{PlayerActionControl, PlayerAxisControl};

    use super::GamePlayBundle;

    #[test]
    fn bundle_build_should_succeed() {
        env::set_var("APP_DIR", env!("CARGO_MANIFEST_DIR"));

        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::blank()
                .with_bundle(TransformBundle::new())
                .with_bundle(InputBundle::<PlayerAxisControl, PlayerActionControl>::new())
                .with_bundle(UiBundle::<PlayerAxisControl, PlayerActionControl>::new())
                .with_bundle(GamePlayBundle)
                .run()
                .is_ok()
        );
    }
}
