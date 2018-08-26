use amethyst::{
    core::bundle::{Result, SystemBundle},
    ecs::prelude::*,
};
use typename::TypeName;

use CharacterSelectionSystem;

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
    pub fn with_system_dependencies(&mut self, dependencies: &[String]) {
        self.system_dependencies = Some(Vec::from(dependencies));
    }
}

impl<'a, 'b> SystemBundle<'a, 'b> for CharacterSelectionBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        let deps = self
            .system_dependencies
            .as_ref()
            .map_or_else(Vec::new, |deps| {
                deps.iter().map(|dep| dep.as_ref()).collect::<Vec<&str>>()
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
    use amethyst_test_support::prelude::*;

    use super::CharacterSelectionBundle;

    #[test]
    fn bundle_build_should_succeed() {
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::blank()
                .with_bundle(CharacterSelectionBundle::new())
                .run()
                .is_ok()
        );
    }
}
