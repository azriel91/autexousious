use amethyst::{
    core::bundle::{Result, SystemBundle},
    ecs::prelude::*,
};

use CharacterGroundingSystem;
use CharacterInputUpdateSystem;
use CharacterKinematicsSystem;
use CharacterSequenceUpdateSystem;
use ObjectKinematicsUpdateSystem;
use ObjectTransformUpdateSystem;

/// Adds the `CharacterInputUpdateSystem` to the `World` with id `"character_input_update_system"`.
///
/// The Amethyst `InputBundle` must be added before this bundle.
#[derive(Debug, new)]
pub struct GamePlayBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for GamePlayBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        // TODO: Custom derive to get snake_cased name
        // See <https://docs.rs/named_type/0.1.3/named_type/>
        builder.add(
            CharacterInputUpdateSystem::new(),
            "character_input_update_system",
            &["input_system"],
        );
        builder.add(
            CharacterSequenceUpdateSystem::new(),
            "character_sequence_update_system",
            &["character_input_update_system"],
        );
        builder.add(
            CharacterKinematicsSystem::new(),
            "character_kinematics_system",
            &["character_sequence_update_system"],
        );
        builder.add(
            ObjectKinematicsUpdateSystem::new(),
            "object_kinematics_update_system",
            &["character_kinematics_system"],
        );
        builder.add(
            CharacterGroundingSystem::new(),
            "character_grounding_system",
            &["object_kinematics_update_system"],
        );
        builder.add(
            ObjectTransformUpdateSystem::new(),
            "object_transform_update_system",
            &["character_grounding_system"],
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
