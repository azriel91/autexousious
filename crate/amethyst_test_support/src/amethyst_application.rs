use amethyst::{core::SystemBundle, prelude::*, Result};

use EmptyState;

/// Builder for an Amethyst application.
///
/// This provides varying levels of setup so that users do not have to register common bundles.
#[derive(Derivative, Default)]
#[derivative(Debug)]
pub struct AmethystApplication<'a, 'b> {
    /// Tracks the bundles that will be used in the application.
    #[derivative(Debug = "ignore")]
    game_data: GameDataBuilder<'a, 'b>,
}

impl<'a, 'b> AmethystApplication<'a, 'b> {
    /// Start a with a blank Amethyst application.
    ///
    /// This does not register any bundles.
    pub fn blank() -> AmethystApplication<'a, 'b> {
        AmethystApplication::default()
    }

    /// Adds a bundle to the list of bundles.
    ///
    /// # Parameters
    ///
    /// * `bundle`: Bundle to add.
    pub fn with_bundle<B: 'static>(mut self, bundle: B) -> Result<Self>
    where
        B: SystemBundle<'a, 'b>,
    {
        self.game_data = self.game_data.with_bundle(bundle)?;
        Ok(self)
    }

    /// Returns the built Application.
    pub fn build(self) -> Result<Application<'a, GameData<'a, 'b>>> {
        let assets_dir = format!("{}/assets", env!("CARGO_MANIFEST_DIR"));
        let first_state = EmptyState;

        Application::new(assets_dir, first_state, self.game_data)
    }
}

#[cfg(test)]
mod test {
    use amethyst::{
        core::bundle::{self, SystemBundle},
        ecs::prelude::*,
    };

    use super::AmethystApplication;

    #[test]
    fn bundle_build_is_ok() {
        assert!(
            AmethystApplication::blank()
                .with_bundle(BundleZero)
                .unwrap()
                .build()
                .is_ok()
        );
    }

    #[test]
    fn load_multiple_bundles() {
        assert!(
            AmethystApplication::blank()
                .with_bundle(BundleZero)
                .unwrap()
                .with_bundle(BundleOne)
                .unwrap()
                .build()
                .is_ok()
        );
    }

    #[derive(Debug)]
    struct SystemZero;
    impl<'s> System<'s> for SystemZero {
        type SystemData = ();
        fn run(&mut self, _: Self::SystemData) {}
    }

    #[derive(Debug)]
    struct SystemOne;
    impl<'s> System<'s> for SystemOne {
        type SystemData = ();
        fn run(&mut self, _: Self::SystemData) {}
    }

    #[derive(Debug)]
    struct BundleZero;
    impl<'a, 'b> SystemBundle<'a, 'b> for BundleZero {
        fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> bundle::Result<()> {
            builder.add(SystemZero, "system_zero", &[]);
            Ok(())
        }
    }

    #[derive(Debug)]
    struct BundleOne;
    impl<'a, 'b> SystemBundle<'a, 'b> for BundleOne {
        fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> bundle::Result<()> {
            builder.add(SystemOne, "system_one", &["system_zero"]);
            Ok(())
        }
    }
}
