use amethyst::{
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, World},
    Error,
};
use derive_new::new;
use typename::TypeName;

use crate::CharacterSelectionSystem;

/// Adds the `CharacterSelectionSystem` to the `World`.
///
/// The Amethyst `InputBundle` must be added before this bundle.
#[derive(Debug, new)]
pub struct CharacterSelectionBundle {
    /// System names that the `CharacterSelectionSystem` should depend on.
    #[new(default)]
    system_dependencies: Option<Vec<String>>,
}

impl CharacterSelectionBundle {
    /// Specifies system dependencies for the `CharacterSelectionSystem`.
    ///
    /// # Parameters
    ///
    /// * `dependencies`: Names of the systems to depend on.
    pub fn with_system_dependencies(mut self, dependencies: &[String]) -> Self {
        self.system_dependencies = Some(Vec::from(dependencies));
        self
    }
}

impl<'a, 'b> SystemBundle<'a, 'b> for CharacterSelectionBundle {
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        let deps = self
            .system_dependencies
            .as_ref()
            .map_or_else(Vec::new, |deps| {
                deps.iter().map(AsRef::as_ref).collect::<Vec<&str>>()
            });

        builder.add(
            CharacterSelectionSystem::new(),
            &CharacterSelectionSystem::type_name(),
            &deps,
        ); // kcov-ignore

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use amethyst::core::transform::TransformBundle;
    use amethyst_test::prelude::*;

    use super::CharacterSelectionBundle;

    #[test]
    fn bundle_build_should_succeed() {
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::blank()
                .with_bundle(TransformBundle::new())
                .with_bundle(
                    CharacterSelectionBundle::new()
                        .with_system_dependencies(&["transform_system".to_string()])
                )
                .run()
                .is_ok()
        );
    }
}
