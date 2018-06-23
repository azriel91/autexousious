use std::marker::PhantomData;
use std::thread;

use amethyst::{core::SystemBundle, prelude::*, Result};
use boxfnonce::SendBoxFnOnce;

use AssertionState;
use EmptyState;

type BundleAddFn = SendBoxFnOnce<
    'static,
    (GameDataBuilder<'static, 'static>,),
    Result<GameDataBuilder<'static, 'static>>,
>;

// Hacks for ergonomics so users don't have to specify the type parameter if they don't specify an
// assertion function such as `AmethystApplication::<fn(&mut World)>`.
//
// See <https://stackoverflow.com/questions/37310941/default-generic-parameter>
type StatePlaceholder = EmptyState;
type FnStatePlaceholder = &'static fn() -> StatePlaceholder;
type FnAssertPlaceholder = &'static fn(&mut World);

/// Builder for an Amethyst application.
///
/// This provides varying levels of setup so that users do not have to register common bundles.
#[derive(Derivative, Default)]
#[derivative(Debug)]
pub struct AmethystApplication<S, T, FnState, FnAssert = FnAssertPlaceholder>
where
    S: State<T>,
    FnState: Fn() -> S + Send,
    FnAssert: Fn(&mut World) + Send,
{
    /// Functions to add bundles to the game data.
    ///
    /// This is necessary because `System`s are not `Send`, and so we cannot send `GameDataBuilder`
    /// across a thread boundary, necessary to run the `Application` in a sub thread to avoid a
    /// segfault caused by mesa and the software GL renderer.
    #[derivative(Debug = "ignore")]
    bundle_add_fns: Vec<BundleAddFn>,
    /// Assertion function to run.
    assertion_fn: Option<FnAssert>,
    /// Function to create user specified state to use for the application.
    first_state_fn: Option<FnState>,
    /// State data.
    state_data: PhantomData<T>,
}

impl
    AmethystApplication<
        StatePlaceholder,
        GameData<'static, 'static>,
        FnStatePlaceholder,
        FnAssertPlaceholder,
    >
{
    /// Start a with a blank Amethyst application.
    ///
    /// This does not register any bundles.
    pub fn blank() -> AmethystApplication<
        StatePlaceholder,
        GameData<'static, 'static>,
        FnStatePlaceholder,
        FnAssertPlaceholder,
    > {
        AmethystApplication {
            bundle_add_fns: Vec::new(),
            assertion_fn: None,
            first_state_fn: None,
            state_data: PhantomData,
        }
    }
}

impl<S, FnState, FnAssert> AmethystApplication<S, GameData<'static, 'static>, FnState, FnAssert>
where
    S: State<GameData<'static, 'static>> + 'static,
    FnState: Fn() -> S + Send + 'static,
    FnAssert: Fn(&mut World) + Send + 'static,
{
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
        let params = (self.bundle_add_fns, self.assertion_fn, self.first_state_fn);
        Self::internal_build(params)
    }

    // Hack to get around `S` or `T` not being `Send`
    // We take a function that constructs `S`, and the function itself is `Send`.
    // However, `Self` has `PhantomData<T>`, which means we cannot send `self` to a thread. Instead
    // we have to take all of the other fields and send those through.
    fn internal_build(
        (bundle_add_fns, mut assertion_fn, first_state_fn): (
            Vec<BundleAddFn>,
            Option<FnAssert>,
            Option<FnState>,
        ),
    ) -> Result<Application<'static, GameData<'static, 'static>>> {
        let assets_dir = format!("{}/assets", env!("CARGO_MANIFEST_DIR"));

        let game_data = bundle_add_fns.into_iter().fold(
            Ok(GameDataBuilder::default()),
            |game_data: Result<GameDataBuilder>, function: BundleAddFn| {
                game_data.and_then(|game_data| function.call(game_data))
            },
        )?;

        if assertion_fn.is_some() {
            let assertion_state = AssertionState::new(assertion_fn.take().unwrap());
            if first_state_fn.is_some() {
                let assertion_state = assertion_state.with_stack_state((first_state_fn.unwrap())());
                Application::new(assets_dir, assertion_state, game_data)
            } else {
                Application::new(assets_dir, assertion_state, game_data)
            }
        } else if let Some(first_state_fn) = first_state_fn {
            Application::new(assets_dir, first_state_fn(), game_data)
        } else {
            Application::new(assets_dir, EmptyState, game_data)
        }
    }

    /// Runs the application and returns `Ok(())` if nothing went wrong.
    ///
    /// This method should be called instead of the `.build()` method if the application is to be
    /// run, as this avoids a segfault on Linux when using the GL software renderer.
    pub fn run(self) -> Result<()> {
        let params = (self.bundle_add_fns, self.assertion_fn, self.first_state_fn);

        // Run in a sub thread due to mesa's threading issues with GL software rendering
        // See: <https://users.rust-lang.org/t/trouble-identifying-cause-of-segfault/18096>
        thread::spawn(|| -> Result<()> {
            Self::internal_build(params)?.run();

            Ok(())
        }).join()
            .expect("Failed to run Amethyst application")
    }
}

impl<S, T, FnState, FnAssert> AmethystApplication<S, T, FnState, FnAssert>
where
    S: State<T> + 'static,
    FnState: Fn() -> S + Send + 'static,
    FnAssert: Fn(&mut World) + Send + 'static,
{
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

    /// Registers a function to assert an expected outcome.
    ///
    /// The function will be run in an [`AssertionState`](struct.AssertionState.html)
    ///
    /// # Parameters
    ///
    /// * `assertion_fn`: Function that asserts the expected state.
    pub fn with_assertion<FnAssertLocal>(
        self,
        assertion_fn: FnAssertLocal,
    ) -> AmethystApplication<S, T, FnState, FnAssertLocal>
    where
        FnAssertLocal: Fn(&mut World) + Send,
    {
        if self.assertion_fn.is_some() {
            panic!(
                ".with_assertion(F) has previously been called. The current implementation only \
                 supports one assertion function."
            );
        } else {
            AmethystApplication {
                bundle_add_fns: self.bundle_add_fns,
                assertion_fn: Some(assertion_fn),
                first_state_fn: self.first_state_fn,
                state_data: self.state_data,
            }
        }
    }

    /// Sets the state for the Amethyst application.
    ///
    /// # Parameters
    ///
    /// * `state`: `State` to use.
    pub fn with_state<SLocal, TLocal, FnStateLocal>(
        self,
        state: FnStateLocal,
    ) -> AmethystApplication<SLocal, TLocal, FnStateLocal, FnAssert>
    where
        SLocal: State<TLocal>,
        FnStateLocal: Fn() -> SLocal + Send,
    {
        if self.first_state_fn.is_some() {
            panic!(
                ".with_state(S) has previously been called. The current implementation only \
                 supports one starting state."
            );
        } else {
            AmethystApplication {
                bundle_add_fns: self.bundle_add_fns,
                assertion_fn: self.assertion_fn,
                first_state_fn: Some(state),
                state_data: PhantomData,
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::marker::PhantomData;

    use amethyst::{
        core::bundle::{self, SystemBundle},
        ecs::prelude::*,
        prelude::*,
    };

    use super::AmethystApplication;
    use AssertionState;
    use EmptyState;

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

    #[test]
    fn assertion_when_resource_is_added_succeeds() {
        let assertion_fn = |world: &mut World| {
            world.read_resource::<ApplicationResource>();
            world.read_resource::<ApplicationResourceNonDefault>();
        };
        assert!(
            AmethystApplication::blank()
                .with_bundle(BundleZero)
                .with_bundle(BundleOne)
                .with_assertion(assertion_fn)
                .run()
                .is_ok()
        );
    }

    #[test]
    #[should_panic(expected = "Failed to run Amethyst application")]
    fn assertion_when_resource_is_not_added_should_panic() {
        let assertion_fn = |world: &mut World| {
            // Panics if `ApplicationResource` was not added.
            world.read_resource::<ApplicationResource>();
        };

        assert!(
            AmethystApplication::blank()
                    // without BundleOne
                    .with_assertion(assertion_fn)
                    .run()
                    .is_ok()
        );
    }

    #[test]
    fn assertion_switch_with_loading_state_with_add_resource_succeeds() {
        let first_state_fn = || {
            let assertion_fn = |world: &mut World| {
                world.read_resource::<LoadResource>();
            };

            // Necessary if the State being tested is a loading state that returns `Trans::Switch`
            let assertion_state = AssertionState::new(assertion_fn);
            LoadingState::new(assertion_state)
        };

        assert!(
            AmethystApplication::blank()
                .with_state(first_state_fn)
                .run()
                .is_ok()
        );
    }

    #[test]
    fn assertion_push_with_loading_state_with_add_resource_succeeds() {
        // Alternative to embedding the `AssertionState` is to switch to an `EmptyState` but still
        // provide the assertion function
        let first_state_fn = || LoadingState::new(EmptyState);
        let assertion_fn = |world: &mut World| {
            world.read_resource::<LoadResource>();
        };

        assert!(
            AmethystApplication::blank()
                .with_state(first_state_fn)
                .with_assertion(assertion_fn)
                .run()
                .is_ok()
        );
    }

    #[test]
    #[should_panic(expected = "Failed to run Amethyst application")]
    fn assertion_switch_with_loading_state_without_add_resource_should_panic() {
        let first_state_fn = || {
            let assertion_fn = |world: &mut World| {
                world.read_resource::<LoadResource>();
            };

            SwitchState::new(AssertionState::new(assertion_fn))
        };

        assert!(
            AmethystApplication::blank()
                .with_state(first_state_fn)
                .run()
                .is_ok()
        );
    }

    #[test]
    #[should_panic(expected = "Failed to run Amethyst application")]
    fn assertion_push_with_loading_state_without_add_resource_should_panic() {
        // Alternative to embedding the `AssertionState` is to switch to an `EmptyState` but still
        // provide the assertion function
        let first_state_fn = || SwitchState::new(EmptyState);
        let assertion_fn = |world: &mut World| {
            world.read_resource::<LoadResource>();
        };

        assert!(
            AmethystApplication::blank()
                .with_state(first_state_fn)
                .with_assertion(assertion_fn)
                .run()
                .is_ok()
        );
    }

    // === Resources === //
    #[derive(Debug, Default)]
    struct ApplicationResource;
    #[derive(Debug)]
    struct ApplicationResourceNonDefault;
    #[derive(Debug)]
    struct LoadResource;

    // === States === //
    struct LoadingState<'a, 'b, S>
    where
        S: State<GameData<'a, 'b>> + 'static,
    {
        next_state: Option<S>,
        state_data: PhantomData<State<GameData<'a, 'b>>>,
    }
    impl<'a, 'b, S> LoadingState<'a, 'b, S>
    where
        S: State<GameData<'a, 'b>> + 'static,
    {
        fn new(next_state: S) -> Self {
            LoadingState {
                next_state: Some(next_state),
                state_data: PhantomData,
            }
        }
    }
    impl<'a, 'b, S> State<GameData<'a, 'b>> for LoadingState<'a, 'b, S>
    where
        S: State<GameData<'a, 'b>> + 'static,
    {
        fn update(&mut self, data: StateData<GameData>) -> Trans<GameData<'a, 'b>> {
            data.data.update(&data.world);
            data.world.add_resource(LoadResource);
            Trans::Switch(Box::new(self.next_state.take().unwrap()))
        }
    }

    struct SwitchState<S, T>
    where
        S: State<T>,
    {
        next_state: Option<S>,
        state_data: PhantomData<T>,
    }
    impl<S, T> SwitchState<S, T>
    where
        S: State<T>,
    {
        fn new(next_state: S) -> Self {
            SwitchState {
                next_state: Some(next_state),
                state_data: PhantomData,
            }
        }
    }
    impl<S, T> State<T> for SwitchState<S, T>
    where
        S: State<T> + 'static,
    {
        fn update(&mut self, _data: StateData<T>) -> Trans<T> {
            Trans::Switch(Box::new(self.next_state.take().unwrap()))
        }
    }

    // === Systems === //
    #[derive(Debug)]
    struct SystemZero;
    impl<'s> System<'s> for SystemZero {
        type SystemData = ();
        fn run(&mut self, _: Self::SystemData) {}
    }

    #[derive(Debug)]
    struct SystemOne;
    type SystemOneData<'s> = Read<'s, ApplicationResource>;
    impl<'s> System<'s> for SystemOne {
        type SystemData = SystemOneData<'s>;
        fn run(&mut self, _: Self::SystemData) {}
    }

    #[derive(Debug)]
    struct SystemNonDefault;
    type SystemNonDefaultData<'s> = ReadExpect<'s, ApplicationResourceNonDefault>;
    impl<'s> System<'s> for SystemNonDefault {
        type SystemData = SystemNonDefaultData<'s>;
        fn run(&mut self, _: Self::SystemData) {}

        fn setup(&mut self, res: &mut Resources) {
            // Must be called when we override `.setup()`
            SystemNonDefaultData::setup(res);

            // Need to manually insert this when the resource is `!Default`
            res.insert(ApplicationResourceNonDefault);
        }
    }

    // === Bundles === //
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
            builder.add(SystemNonDefault, "system_non_default", &[]);
            Ok(())
        }
    }
}
