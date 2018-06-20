use std::thread;

use amethyst::{core::SystemBundle, prelude::*, Result};
use boxfnonce::SendBoxFnOnce;

use EmptyState;

type BundleAddFn = SendBoxFnOnce<
    'static,
    (GameDataBuilder<'static, 'static>,),
    Result<GameDataBuilder<'static, 'static>>,
>;

/// Builder for an Amethyst application.
///
/// This provides varying levels of setup so that users do not have to register common bundles.
#[derive(Derivative, Default)]
#[derivative(Debug)]
pub struct AmethystApplication {
    /// Functions to add bundles to the game data.
    ///
    /// This is necessary because `System`s are not `Send`, and so we cannot send `GameDataBuilder`
    /// across a thread boundary, necessary to run the `Application` in a sub thread to avoid a
    /// segfault caused by mesa and the software GL renderer.
    #[derivative(Debug = "ignore")]
    bundle_add_fns: Vec<BundleAddFn>,
}

impl AmethystApplication {
    /// Start a with a blank Amethyst application.
    ///
    /// This does not register any bundles.
    pub fn blank() -> AmethystApplication {
        AmethystApplication::default()
    }

    /// Adds a bundle to the list of bundles.
    ///
    /// # Parameters
    ///
    /// * `bundle`: Bundle to add.
    pub fn with_bundle<B>(mut self, bundle: B) -> Self
    where
        B: SystemBundle<'static, 'static> + Send + 'static,
    {
        // We need to use `SendBoxFnOnce` because:
        //
        // * `FnOnce` takes itself by value when you call it.
        // * To pass a `FnOnce` around (transferring ownership), it must be boxed, since it's not
        //   `Sized`.
        // * A `Box<FnOnce()>` is a `Sized` type with a reference to the `FnOnce`
        // * To call the function inside the `Box<FnOnce()>`, it must be moved out of the box
        //   because we need to own the `FnOnce` to be able to call it by value, whereas the `Box`
        //   only holds the reference.
        // * To own it, we would have to move it onto the stack.
        // * However, since it's not `Sized`, we can't do that.
        //
        // To make this work, we can implement a trait for `FnOnce` with a trait function which
        // takes `Box<Self>` and can invoke the `FnOnce` whilst inside the Box.
        // `SendBoxFnOnce` is an implementation this.
        //
        // See <https://users.rust-lang.org/t/move-a-boxed-function-inside-a-closure/18199>
        self.bundle_add_fns.push(SendBoxFnOnce::from(
            |game_data: GameDataBuilder<'static, 'static>| game_data.with_bundle(bundle),
        ));
        self
    }

    /// Returns the built Application.
    ///
    /// If you are intending to call `.run()` on the `Application` in a test, be aware that on
    /// Linux, this will cause a segfault when `RenderBundle` is added and GL is using software
    /// rendering, such as when using Xvfb or when the following environmental variable is set:
    /// `LIBGL_ALWAYS_SOFTWARE=1`.
    ///
    /// To avoid this, please call `.run()` instead of this method, which runs the application in a
    /// separate thread and waits for it to end before returning.
    ///
    /// See <https://users.rust-lang.org/t/trouble-identifying-cause-of-segfault/18096>
    pub fn build(self) -> Result<Application<'static, GameData<'static, 'static>>> {
        let assets_dir = format!("{}/assets", env!("CARGO_MANIFEST_DIR"));
        let first_state = EmptyState;
        let game_data = self.bundle_add_fns.into_iter().fold(
            Ok(GameDataBuilder::default()),
            |game_data: Result<GameDataBuilder>, function: BundleAddFn| {
                game_data.and_then(|game_data| function.call(game_data))
            },
        )?;

        Application::new(assets_dir, first_state, game_data)
    }

    /// Runs the application and returns `Ok(())` if nothing went wrong.
    ///
    /// This method should be called instead of the `.build()` method if the application is to be
    /// run, as this avoids a segfault on Linux when using the GL software renderer.
    pub fn run(self) -> Result<()> {
        // Run in a sub thread due to mesa's threading issues with GL software rendering
        // See: <https://users.rust-lang.org/t/trouble-identifying-cause-of-segfault/18096>
        thread::spawn(|| -> Result<()> {
            self.build()?.run();

            Ok(())
        }).join()
            .expect("Failed to run Amethyst application")
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
                .run()
                .is_ok()
        );
    }

    #[test]
    fn load_multiple_bundles() {
        assert!(
            AmethystApplication::blank()
                .with_bundle(BundleZero)
                .with_bundle(BundleOne)
                .run()
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
