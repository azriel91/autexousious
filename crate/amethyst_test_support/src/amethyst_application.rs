use std::marker::PhantomData;
use std::sync::Mutex;
use std::thread;

use amethyst::{
    animation::AnimationBundle,
    core::{transform::TransformBundle, SystemBundle},
    ecs::prelude::*,
    input::InputBundle,
    prelude::*,
    renderer::{
        ColorMask, DisplayConfig, DrawFlat, Material, Pipeline, PipelineBuilder, PosTex,
        RenderBundle, ScreenDimensions, Stage, StageBuilder, ALPHA,
    },
    shred::Resource,
    ui::{DrawUi, UiBundle},
    Result,
};
use boxfnonce::SendBoxFnOnce;
use hetseq::Queue;

use EmptyState;
use FunctionState;
use SchedulerState;
use SystemInjectionBundle;

type BundleAddFn = SendBoxFnOnce<
    'static,
    (GameDataBuilder<'static, 'static>,),
    Result<GameDataBuilder<'static, 'static>>,
>;
// Hack: Ideally we want a `SendBoxFnOnce`. However implementing it got too crazy:
//
// * When taking in `ApplicationBuilder<SLocal>` as a parameter, I couldn't get the type parameters
//   to be happy. `SLocal` had to change depending on the first state, but it couldn't be
//   consolidated with `T`.
// * When using `SendBoxFnOnce<'w, (&'w mut World,)>`, the lifetime parameter for the function and
//   the `World` could not agree &mdash; you can't coerce a `SendBoxFnOnce<'longer>` into a
//   `SendBoxFnOnce<'shorter>`, which was necessary to indicate the length of the borrow of `World`
//   for the function is not the `'w` needed to store the function in `AmethystApplication`.
//   In addition, it requires the `World` (and hence the `ApplicationBuilder`) to be instantiated
//   in a scope greater than the `AmethystApplication`'s lifetime, which detracts from the
//   ergonomics of this test harness.
type FnResourceAdd = Box<FnMut(&mut World) + Send>;

// Hacks for ergonomics so users don't have to specify the type parameter if they don't specify an
// assertion function such as `AmethystApplication::<fn(&mut World)>`.
//
// See <https://stackoverflow.com/questions/37310941/default-generic-parameter>
type StatePlaceholder = EmptyState;
type FnSetupPlaceholder = &'static fn(&mut World);
type FnStatePlaceholder = &'static fn() -> StatePlaceholder;
type FnEffectPlaceholder = &'static fn(&mut World);
type FnAssertPlaceholder = &'static fn(&mut World);

type DefaultPipeline = PipelineBuilder<
    Queue<(
        Queue<()>,
        StageBuilder<Queue<(Queue<(Queue<()>, DrawFlat<PosTex>)>, DrawUi)>>,
    )>,
>;

/// Screen width used in predefined display configuration.
pub const SCREEN_WIDTH: u32 = 800;
/// Screen height used in predefined display configuration.
pub const SCREEN_HEIGHT: u32 = 600;
/// The ratio between the backing framebuffer resolution and the window size in screen pixels.
/// This is typically one for a normal display and two for a retina display.
pub const HIDPI: f32 = 1.;

// Use a mutex to prevent multiple tests that open GL windows from running simultaneously, due to
// race conditions causing failures in X.
// <https://github.com/tomaka/glutin/issues/1038>
lazy_static! {
    static ref X11_GL_MUTEX: Mutex<()> = Mutex::new(());
}

/// Builder for an Amethyst application.
///
/// This provides varying levels of setup so that users do not have to register common bundles.
#[derive(Derivative, Default)]
#[derivative(Debug)]
pub struct AmethystApplication<S, T, FnSetup, FnState, FnEffect, FnAssert>
where
    S: State<T>,
    FnSetup: Fn(&mut World) + Send,
    FnState: Fn() -> S + Send,
    FnEffect: Fn(&mut World) + Send,
    FnAssert: Fn(&mut World) + Send,
{
    /// Functions to add bundles to the game data.
    ///
    /// This is necessary because `System`s are not `Send`, and so we cannot send `GameDataBuilder`
    /// across a thread boundary, necessary to run the `Application` in a sub thread to avoid a
    /// segfault caused by mesa and the software GL renderer.
    #[derivative(Debug = "ignore")]
    bundle_add_fns: Vec<BundleAddFn>,
    /// Functions to add bundles to the game data.
    ///
    /// This is necessary because `System`s are not `Send`, and so we cannot send `GameDataBuilder`
    /// across a thread boundary, necessary to run the `Application` in a sub thread to avoid a
    /// segfault caused by mesa and the software GL renderer.
    #[derivative(Debug = "ignore")]
    resource_add_fns: Vec<FnResourceAdd>,
    /// Setup function to run.
    setup_fn: Option<FnSetup>,
    /// Function to create user specified state to use for the application.
    state_fn: Option<FnState>,
    /// Effect function to run.
    effect_fn: Option<FnEffect>,
    /// Assertion function to run.
    assertion_fn: Option<FnAssert>,
    /// State data.
    state_data: PhantomData<T>,
    /// Whether or not this application uses the `RenderBundle`.
    render: bool,
}

impl
    AmethystApplication<
        StatePlaceholder,
        GameData<'static, 'static>,
        FnSetupPlaceholder,
        FnStatePlaceholder,
        FnEffectPlaceholder,
        FnAssertPlaceholder,
    >
{
    /// Returns an Amethyst application without any bundles.
    pub fn blank() -> AmethystApplication<
        StatePlaceholder,
        GameData<'static, 'static>,
        FnSetupPlaceholder,
        FnStatePlaceholder,
        FnEffectPlaceholder,
        FnAssertPlaceholder,
    > {
        AmethystApplication {
            bundle_add_fns: Vec::new(),
            resource_add_fns: Vec::new(),
            setup_fn: None,
            state_fn: None,
            effect_fn: None,
            assertion_fn: None,
            state_data: PhantomData,
            render: false,
        }
    }

    /// Returns an application with the Transform, Input, and UI bundles.
    ///
    /// This also adds a `ScreenDimensions` resource to the `World` so that UI calculations can be
    /// done.
    pub fn base() -> AmethystApplication<
        StatePlaceholder,
        GameData<'static, 'static>,
        FnSetupPlaceholder,
        FnStatePlaceholder,
        FnEffectPlaceholder,
        FnAssertPlaceholder,
    > {
        AmethystApplication::blank()
            .with_bundle(TransformBundle::new())
            .with_bundle(InputBundle::<String, String>::new())
            .with_bundle(UiBundle::<String, String>::new())
            .with_resource(ScreenDimensions::new(SCREEN_WIDTH, SCREEN_HEIGHT, HIDPI))
    }

    /// Returns an application with the Animation, Transform, Input, UI, and Render bundles.
    ///
    /// **Note:** The type parameters for the Animation, Input, and UI bundles are [stringly-typed]
    /// [stringly]. It is recommended that you use proper type parameters and register the bundles
    /// yourself if the unit you are testing uses them.
    ///
    /// # Parameters
    ///
    /// * `test_name`: Name of the test, used to populate the window title.
    /// * `visibility`: Whether the window should be visible.
    ///
    /// [stringly]: http://wiki.c2.com/?StringlyTyped
    pub fn render_base<'name, N>(
        test_name: N,
        visibility: bool,
    ) -> AmethystApplication<
        StatePlaceholder,
        GameData<'static, 'static>,
        FnSetupPlaceholder,
        FnStatePlaceholder,
        FnEffectPlaceholder,
        FnAssertPlaceholder,
    >
    where
        N: Into<&'name str>,
    {
        AmethystApplication::blank()
            .with_bundle(AnimationBundle::<u32, Material>::new(
                "animation_control_system",
                "sampler_interpolation_system",
            ))
            .with_bundle(
                TransformBundle::new()
                    .with_dep(&["animation_control_system", "sampler_interpolation_system"]),
            )
            .with_bundle(InputBundle::<String, String>::new())
            .with_bundle(UiBundle::<String, String>::new())
            .with_render_bundle(test_name, visibility)
    }

    /// Returns a `String` to `<crate_dir>/assets`.
    pub fn assets_dir() -> String {
        format!("{}/assets", env!("CARGO_MANIFEST_DIR"))
    }
}

impl<S, FnSetup, FnState, FnEffect, FnAssert>
    AmethystApplication<S, GameData<'static, 'static>, FnSetup, FnState, FnEffect, FnAssert>
where
    S: State<GameData<'static, 'static>> + 'static,
    FnSetup: Fn(&mut World) + Send + 'static,
    FnState: Fn() -> S + Send + 'static,
    FnEffect: Fn(&mut World) + Send + 'static,
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
        let params = (
            self.bundle_add_fns,
            self.resource_add_fns,
            self.setup_fn,
            self.state_fn,
            self.effect_fn,
            self.assertion_fn,
        );
        Self::build_internal(params)
    }

    // Hack to get around `S` or `T` not being `Send`
    // We take a function that constructs `S`, and the function itself is `Send`.
    // However, `Self` has `PhantomData<T>`, which means we cannot send `self` to a thread. Instead
    // we have to take all of the other fields and send those through.
    //
    // Need to `#[allow(type_complexity)]` because the type declaration would have unused type
    // parameters which causes a compilation failure.
    #[allow(unknown_lints, type_complexity)]
    fn build_internal(
        (bundle_add_fns, resource_add_fns, setup_fn, state_fn, effect_fn, assertion_fn): (
            Vec<BundleAddFn>,
            Vec<FnResourceAdd>,
            Option<FnSetup>,
            Option<FnState>,
            Option<FnEffect>,
            Option<FnAssert>,
        ),
    ) -> Result<Application<'static, GameData<'static, 'static>>> {
        let game_data = bundle_add_fns.into_iter().fold(
            Ok(GameDataBuilder::default()),
            |game_data: Result<GameDataBuilder>, function: BundleAddFn| {
                game_data.and_then(|game_data| function.call(game_data))
            },
        )?;

        let mut states = Vec::<Box<State<GameData<'static, 'static>>>>::new();
        if assertion_fn.is_some() {
            states.push(Box::new(FunctionState::new(assertion_fn.unwrap())));
        }
        if effect_fn.is_some() {
            states.push(Box::new(FunctionState::new(effect_fn.unwrap())));
        }
        if state_fn.is_some() {
            states.push(Box::new(state_fn.unwrap()()));
        }
        if setup_fn.is_some() {
            states.push(Box::new(FunctionState::new(setup_fn.unwrap())));
        }
        Self::build_application(SchedulerState::new(states), game_data, resource_add_fns)
    }

    fn build_application<SLocal>(
        first_state: SLocal,
        game_data: GameDataBuilder<'static, 'static>,
        resource_add_fns: Vec<FnResourceAdd>,
    ) -> Result<Application<'static, GameData<'static, 'static>>>
    where
        SLocal: State<GameData<'static, 'static>> + 'static,
    {
        let mut application_builder =
            Application::build(AmethystApplication::assets_dir(), first_state)?;
        {
            let world = &mut application_builder.world;
            for mut function in resource_add_fns {
                function(world);
            }
        }
        application_builder.build(game_data)
    }

    /// Runs the application and returns `Ok(())` if nothing went wrong.
    ///
    /// This method should be called instead of the `.build()` method if the application is to be
    /// run, as this avoids a segfault on Linux when using the GL software renderer.
    pub fn run(self) -> Result<()> {
        let params = (
            self.bundle_add_fns,
            self.resource_add_fns,
            self.setup_fn,
            self.state_fn,
            self.effect_fn,
            self.assertion_fn,
        );

        let render = self.render;

        // Run in a sub thread due to mesa's threading issues with GL software rendering
        // See: <https://users.rust-lang.org/t/trouble-identifying-cause-of-segfault/18096>
        thread::spawn(move || -> Result<()> {
            if render {
                let guard = X11_GL_MUTEX.lock().unwrap();

                // Note: if this panics, the Mutex is poisoned.
                // Unfortunately we cannot catch panics, as the application is `!UnwindSafe`
                //
                // We have to build the application after acquiring the lock because the window is
                // already instantiated during the build.
                //
                // The mutex greatly reduces, but does not eliminate X11 window initialization
                // errors from happening:
                //
                // * <https://github.com/tomaka/glutin/issues/1034> can still happen
                // * <https://github.com/tomaka/glutin/issues/1038> may be completely removed
                Self::build_internal(params)?.run();

                drop(guard);
            } else {
                Self::build_internal(params)?.run();
            }

            Ok(())
        }).join()
            .expect("Failed to run Amethyst application")
    }
}

impl<S, T, FnSetup, FnState, FnEffect, FnAssert>
    AmethystApplication<S, T, FnSetup, FnState, FnEffect, FnAssert>
where
    S: State<T> + 'static,
    FnSetup: Fn(&mut World) + Send + 'static,
    FnState: Fn() -> S + Send + 'static,
    FnEffect: Fn(&mut World) + Send + 'static,
    FnAssert: Fn(&mut World) + Send + 'static,
{
    /// Adds a bundle to the list of bundles.
    ///
    /// **Note:** If you are adding the `RenderBundle`, you need to use `.with_bundle_fn(F)` as the
    /// `Pipeline` type used by the bundle is `!Send`. Furthermore, you must also invoke
    /// `.mark_render()` to avoid a race condition that causes render tests to fail.
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

    /// Adds a bundle to the list of bundles.
    ///
    /// This provides an alternative to `.with_bundle(B)` where `B` is `!Send`. The function that
    /// instantiates the bundle must be `Send`.
    ///
    /// **Note:** If you are adding the `RenderBundle`, you must also invoke `.mark_render()` to
    /// avoid a race condition that causes render tests to fail.
    ///
    /// **Note:** There is a `.with_render_bundle()` convenience function if you just need the
    /// `RenderBundle` with predefined parameters.
    ///
    /// # Parameters
    ///
    /// * `bundle_function`: Function to instantiate the Bundle.
    pub fn with_bundle_fn<FnBundle, B>(mut self, bundle_function: FnBundle) -> Self
    where
        FnBundle: FnOnce() -> B + Send + 'static,
        B: SystemBundle<'static, 'static> + 'static,
    {
        self.bundle_add_fns.push(SendBoxFnOnce::from(
            move |game_data: GameDataBuilder<'static, 'static>| {
                game_data.with_bundle(bundle_function())
            },
        ));
        self
    }

    /// Registers the `RenderBundle` with this application.
    ///
    /// This is a convenience function that registers the `RenderBundle` using the predefined
    /// [`display_config`][disp] and [`pipeline`][pipe].
    ///
    /// # Parameters
    ///
    /// * `title`: Window title.
    /// * `visibility`: Whether the window should be visible.
    ///
    /// [disp]: #method.display_config
    /// [pipe]: #method.pipeline
    pub fn with_render_bundle<'name, N>(self, title: N, visibility: bool) -> Self
    where
        N: Into<&'name str>,
    {
        // TODO: We can default to the function name once this RFC is implemented:
        // <https://github.com/rust-lang/rfcs/issues/1743>
        // <https://github.com/rust-lang/rfcs/pull/1719>
        let title = title.into().to_string();

        let display_config = Self::display_config(title, visibility);
        let render_bundle_fn = move || RenderBundle::new(Self::pipeline(), Some(display_config));

        self.with_bundle_fn(render_bundle_fn).mark_render()
    }

    /// Adds a resource to the `World`.
    ///
    /// # Parameters
    ///
    /// * `resource`: Bundle to add.
    pub fn with_resource<R>(mut self, resource: R) -> Self
    where
        R: Resource,
    {
        let mut resource_opt = Some(resource);
        self.resource_add_fns
            .push(Box::new(move |world: &mut World| {
                let resource = resource_opt.take();
                if resource.is_some() {
                    world.add_resource(resource.unwrap());
                }
            }));
        self
    }

    /// Sets the state for the Amethyst application.
    ///
    /// **NOTE:** This must be called before any calls to `.with_resource()`, as complexities with
    /// type parameters disallows us to store earlier resource registrations.
    ///
    /// **NOTE:** This function is exclusive of `.with_effect()`, as each of them are ways to test
    /// an effect before an assertion.
    ///
    /// # Parameters
    ///
    /// * `state`: `State` to use.
    pub fn with_state<SLocal, TLocal, FnStateLocal>(
        self,
        state: FnStateLocal,
    ) -> AmethystApplication<SLocal, TLocal, FnSetup, FnStateLocal, FnEffect, FnAssert>
    where
        SLocal: State<TLocal>,
        FnStateLocal: Fn() -> SLocal + Send,
    {
        if self.state_fn.is_some() {
            panic!(
                "`.with_state(S)` has previously been called. The current implementation only \
                 supports one starting state."
            );
        } else if !self.resource_add_fns.is_empty() {
            panic!(
                "`.with_state(S)` called after `.with_resource(R)`. Due to restrictions on type \
                 parameter specification, you must register resources after `.with_state(S)`."
            );
        } else {
            AmethystApplication {
                bundle_add_fns: self.bundle_add_fns,
                resource_add_fns: Vec::new(),
                setup_fn: self.setup_fn,
                state_fn: Some(state),
                effect_fn: self.effect_fn,
                assertion_fn: self.assertion_fn,
                state_data: PhantomData,
                render: self.render,
            }
        }
    }

    /// Registers a `System` to be tested in this application.
    ///
    /// # Parameters
    ///
    /// * `system`: The `System` to be tested.
    pub fn with_system<SysLocal>(
        self,
        system: SysLocal,
        name: &'static str,
        deps: &'static [&'static str],
    ) -> AmethystApplication<S, T, FnSetup, FnState, FnEffect, FnAssert>
    where
        SysLocal: for<'sys_local> System<'sys_local> + Send + 'static,
    {
        self.with_bundle_fn(move || SystemInjectionBundle::new(system, name, deps))
    }

    /// Registers a function that sets up the `World`.
    ///
    /// The function will be run before the state registered with `.with_state(S)`.
    ///
    /// # Parameters
    ///
    /// * `setup_fn`: Function that executes an effect.
    pub fn with_setup<FnSetupLocal>(
        self,
        setup_fn: FnSetupLocal,
    ) -> AmethystApplication<S, T, FnSetupLocal, FnState, FnEffect, FnAssert>
    where
        FnSetupLocal: Fn(&mut World) + Send,
    {
        if self.setup_fn.is_some() {
            panic!(
                "`.with_setup(F)` has previously been called. The current implementation only \
                 supports one setup function."
            );
        } else {
            AmethystApplication {
                bundle_add_fns: self.bundle_add_fns,
                resource_add_fns: self.resource_add_fns,
                setup_fn: Some(setup_fn),
                state_fn: self.state_fn,
                effect_fn: self.effect_fn,
                assertion_fn: self.assertion_fn,
                state_data: self.state_data,
                render: self.render,
            }
        }
    }

    /// Registers a function that executes a desired effect.
    ///
    /// The function will be run after the state registered with `.with_state(S)`, but before the
    /// function registered with `.with_assertion(F)`.
    ///
    /// # Parameters
    ///
    /// * `effect_fn`: Function that executes an effect.
    pub fn with_effect<FnEffectLocal>(
        self,
        effect_fn: FnEffectLocal,
    ) -> AmethystApplication<S, T, FnSetup, FnState, FnEffectLocal, FnAssert>
    where
        FnEffectLocal: Fn(&mut World) + Send,
    {
        if self.effect_fn.is_some() {
            panic!(
                "`.with_effect(F)` has previously been called. The current implementation only \
                 supports one effect function."
            );
        } else {
            AmethystApplication {
                bundle_add_fns: self.bundle_add_fns,
                resource_add_fns: self.resource_add_fns,
                setup_fn: self.setup_fn,
                state_fn: self.state_fn,
                effect_fn: Some(effect_fn),
                assertion_fn: self.assertion_fn,
                state_data: self.state_data,
                render: self.render,
            }
        }
    }

    /// Registers a function to assert an expected outcome.
    ///
    /// The function will be run as the final `State` in the application, given none of the
    /// previous states return `Trans::Quit`.
    ///
    /// # Parameters
    ///
    /// * `assertion_fn`: Function that asserts the expected state.
    pub fn with_assertion<FnAssertLocal>(
        self,
        assertion_fn: FnAssertLocal,
    ) -> AmethystApplication<S, T, FnSetup, FnState, FnEffect, FnAssertLocal>
    where
        FnAssertLocal: Fn(&mut World) + Send,
    {
        if self.assertion_fn.is_some() {
            panic!(
                "`.with_assertion(F)` has previously been called. The current implementation only \
                 supports one assertion function."
            );
        } else {
            AmethystApplication {
                bundle_add_fns: self.bundle_add_fns,
                resource_add_fns: self.resource_add_fns,
                setup_fn: self.setup_fn,
                state_fn: self.state_fn,
                effect_fn: self.effect_fn,
                assertion_fn: Some(assertion_fn),
                state_data: self.state_data,
                render: self.render,
            }
        }
    }

    /// Marks that this application uses the `RenderBundle`.
    ///
    /// **Note:** There is a `.with_render_bundle()` convenience function if you just need the
    /// `RenderBundle` with predefined parameters.
    ///
    /// This is used to avoid a window initialization race condition that causes tests to fail.
    /// See <https://github.com/tomaka/glutin/issues/1038>.
    pub fn mark_render(mut self) -> Self {
        self.render = true;
        self
    }

    /// Convenience function that returns a `DisplayConfig`.
    ///
    /// The configuration uses the following parameters:
    ///
    /// * `title`: As provided.
    /// * `fullscreen`: `false`
    /// * `dimensions`: `Some((800, 600))`
    /// * `min_dimensions`: `Some((400, 300))`
    /// * `max_dimensions`: `None`
    /// * `vsync`: `true`
    /// * `multisampling`: `0` (disabled)
    /// * `visibility`: As provided.
    ///
    /// This is exposed to allow external crates a convenient way of obtaining display
    /// configuration.
    ///
    /// # Parameters
    ///
    /// * `title`: Window title.
    /// * `visibility`: Whether the window should be visible.
    pub fn display_config(title: String, visibility: bool) -> DisplayConfig {
        DisplayConfig {
            title,
            fullscreen: false,
            dimensions: Some((SCREEN_WIDTH, SCREEN_HEIGHT)),
            min_dimensions: Some((SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2)),
            max_dimensions: None,
            vsync: true,
            multisampling: 0, // Must be multiple of 2, use 0 to disable
            visibility,
        }
    }

    /// Convenience function that returns a `PipelineBuilder`.
    ///
    /// The pipeline is built from the following:
    ///
    /// * Black clear target.
    /// * `DrawFlat::<PosTex>` pass with transparency.
    /// * `DrawUi` pass.
    ///
    /// This is exposed to allow external crates a convenient way of obtaining a render pipeline.
    pub fn pipeline() -> DefaultPipeline {
        Pipeline::build().with_stage(
            Stage::with_backbuffer()
                .clear_target([0., 0., 0., 0.], 1.)
                .with_pass(DrawFlat::<PosTex>::new().with_transparency(
                    ColorMask::all(),
                    ALPHA,
                    None,
                ))
                .with_pass(DrawUi::new()),
        )
    }
}

#[cfg(test)]
mod test {
    use std::marker::PhantomData;

    use amethyst::{
        self,
        assets::{self, Asset, AssetStorage, Handle, Loader, ProcessingState, Processor},
        core::bundle::{self, SystemBundle},
        ecs::prelude::*,
        prelude::*,
        renderer::ScreenDimensions,
        ui::FontAsset,
    };

    use super::AmethystApplication;
    use EffectReturn;
    use EmptyState;
    use FunctionState;
    use MaterialAnimationFixture;

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
        let state_fn = || {
            let assertion_fn = |world: &mut World| {
                world.read_resource::<LoadResource>();
            };

            // Necessary if the State being tested is a loading state that returns `Trans::Switch`
            let assertion_state = FunctionState::new(assertion_fn);
            LoadingState::new(assertion_state)
        };

        assert!(
            AmethystApplication::blank()
                .with_state(state_fn)
                .run()
                .is_ok()
        );
    }

    #[test]
    fn assertion_push_with_loading_state_with_add_resource_succeeds() {
        // Alternative to embedding the `FunctionState` is to switch to an `EmptyState` but still
        // provide the assertion function
        let state_fn = || LoadingState::new(EmptyState);
        let assertion_fn = |world: &mut World| {
            world.read_resource::<LoadResource>();
        };

        assert!(
            AmethystApplication::blank()
                .with_state(state_fn)
                .with_assertion(assertion_fn)
                .run()
                .is_ok()
        );
    }

    #[test]
    #[should_panic(expected = "Failed to run Amethyst application")]
    fn assertion_switch_with_loading_state_without_add_resource_should_panic() {
        let state_fn = || {
            let assertion_fn = |world: &mut World| {
                world.read_resource::<LoadResource>();
            };

            SwitchState::new(FunctionState::new(assertion_fn))
        };

        assert!(
            AmethystApplication::blank()
                .with_state(state_fn)
                .run()
                .is_ok()
        );
    }

    #[test]
    #[should_panic(expected = "Failed to run Amethyst application")]
    fn assertion_push_with_loading_state_without_add_resource_should_panic() {
        // Alternative to embedding the `FunctionState` is to switch to an `EmptyState` but still
        // provide the assertion function
        let state_fn = || SwitchState::new(EmptyState);
        let assertion_fn = |world: &mut World| {
            world.read_resource::<LoadResource>();
        };

        assert!(
            AmethystApplication::blank()
                .with_state(state_fn)
                .with_assertion(assertion_fn)
                .run()
                .is_ok()
        );
    }

    #[test]
    fn game_data_must_update_before_assertion() {
        let effect_fn = |world: &mut World| {
            let handles = vec![
                AssetZeroLoader::load(world, AssetZero(10)).unwrap(),
                AssetZeroLoader::load(world, AssetZero(20)).unwrap(),
            ];

            world.add_resource::<Vec<AssetZeroHandle>>(handles);
        };
        let assertion_fn = |world: &mut World| {
            let asset_zero_handles = world.read_resource::<Vec<AssetZeroHandle>>();

            let store = world.read_resource::<AssetStorage<AssetZero>>();
            assert_eq!(Some(&AssetZero(10)), store.get(&asset_zero_handles[0]));
            assert_eq!(Some(&AssetZero(20)), store.get(&asset_zero_handles[1]));
        };

        assert!(
            AmethystApplication::blank()
                .with_bundle(BundleAsset)
                .with_effect(effect_fn)
                .with_assertion(assertion_fn)
                .run()
                .is_ok()
        );
    }

    #[test]
    fn execution_order_is_setup_state_effect_assertion() {
        struct Setup;
        let setup_fn = |world: &mut World| world.add_resource(Setup);
        let state_fn = || {
            LoadingState::new(FunctionState::new(|world| {
                // Panics if setup is not run before this.
                world.read_resource::<Setup>();
            }))
        };
        let effect_fn = |world: &mut World| {
            // If `LoadingState` is not run before this, this will panic
            world.read_resource::<LoadResource>();

            let handles = vec![AssetZeroLoader::load(world, AssetZero(10)).unwrap()];
            world.add_resource(handles);
        };
        let assertion_fn = |world: &mut World| {
            let asset_zero_handles = world.read_resource::<Vec<AssetZeroHandle>>();

            let store = world.read_resource::<AssetStorage<AssetZero>>();
            assert_eq!(Some(&AssetZero(10)), store.get(&asset_zero_handles[0]));
        };

        assert!(
            AmethystApplication::blank()
                .with_bundle(BundleAsset)
                .with_setup(setup_fn)
                .with_state(state_fn)
                .with_effect(effect_fn)
                .with_assertion(assertion_fn)
                .run()
                .is_ok()
        );
    }

    #[test]
    fn base_application_can_load_ui() {
        let assertion_fn = |world: &mut World| {
            // Next line would panic if `UiBundle` wasn't added.
            world.read_resource::<AssetStorage<FontAsset>>();
            // `.base()` should add `ScreenDimensions` as this is necessary for `UiBundle` to
            // initialize properly.
            world.read_resource::<ScreenDimensions>();
        };

        assert!(
            AmethystApplication::base()
                .with_assertion(assertion_fn)
                .run()
                .is_ok()
        );
    }

    #[test]
    fn render_base_application_can_load_material_animations() {
        assert!(
            AmethystApplication::render_base(
                "render_base_application_can_load_material_animations",
                false
            ).with_effect(MaterialAnimationFixture::effect)
                .with_assertion(MaterialAnimationFixture::assertion)
                .run()
                .is_ok()
        );
    }

    #[test]
    fn system_increases_component_value_by_one() {
        let effect_fn = |world: &mut World| {
            let entity = world.create_entity().with(ComponentZero(0)).build();

            world.add_resource(EffectReturn(entity));
        };
        let assertion_fn = |world: &mut World| {
            let entity = world.read_resource::<EffectReturn<Entity>>().0.clone();

            let component_zero_storage = world.read_storage::<ComponentZero>();
            let component_zero = component_zero_storage
                .get(entity)
                .expect("Entity should have a `ComponentZero` component.");

            // If the system ran, the value in the `ComponentZero` should be 1.
            assert_eq!(1, component_zero.0);
        };

        assert!(
            AmethystApplication::blank()
                .with_system(SystemEffect, "system_effect", &[])
                .with_effect(effect_fn)
                .with_assertion(assertion_fn)
                .run()
                .is_ok()
        );
    }

    #[test]
    fn with_system_invoked_twice_should_not_panic() {
        AmethystApplication::blank()
            .with_system(SystemZero, "zero", &[])
            .with_system(SystemOne, "one", &["zero"]);
    }

    // Incorrect usage tests

    #[test]
    #[should_panic(expected = "`.with_setup(F)` has previously been called.")]
    fn with_setup_invoked_twice_should_panic() {
        AmethystApplication::blank()
            .with_setup(|_world| {})
            .with_setup(|_world| {});
    }

    #[test]
    #[should_panic(expected = "`.with_effect(F)` has previously been called.")]
    fn with_effect_invoked_twice_should_panic() {
        AmethystApplication::blank()
            .with_effect(|_world| {})
            .with_effect(|_world| {});
    }

    #[test]
    #[should_panic(expected = "`.with_assertion(F)` has previously been called.")]
    fn with_assertion_invoked_twice_should_panic() {
        AmethystApplication::blank()
            .with_assertion(|_world| {})
            .with_assertion(|_world| {});
    }

    #[test]
    #[should_panic(expected = "`.with_state(S)` has previously been called.")]
    fn with_state_invoked_twice_should_panic() {
        AmethystApplication::blank()
            .with_state::<_, (), _>(|| EmptyState)
            .with_state::<_, (), _>(|| EmptyState);
    }

    #[test]
    #[should_panic(expected = "`.with_state(S)` called after `.with_resource(R)`.")]
    fn with_state_invoked_after_with_resource_should_panic() {
        AmethystApplication::blank()
            .with_resource(ApplicationResource)
            .with_state::<_, (), _>(|| EmptyState);
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

    #[derive(Debug)]
    struct SystemEffect;
    type SystemEffectData<'s> = WriteStorage<'s, ComponentZero>;
    impl<'s> System<'s> for SystemEffect {
        type SystemData = SystemEffectData<'s>;
        fn run(&mut self, mut component_zero_storage: Self::SystemData) {
            for mut component_zero in (&mut component_zero_storage).join() {
                component_zero.0 += 1
            }
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

    #[derive(Debug)]
    struct BundleAsset;
    impl<'a, 'b> SystemBundle<'a, 'b> for BundleAsset {
        fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> bundle::Result<()> {
            builder.add(Processor::<AssetZero>::new(), "asset_zero_processor", &[]);
            Ok(())
        }
    }

    // === Assets === //
    #[derive(Debug, PartialEq)]
    struct AssetZero(u32);
    impl Asset for AssetZero {
        const NAME: &'static str = "amethyst_test_support::AssetZero";
        type Data = Self;
        type HandleStorage = VecStorage<Handle<Self>>;
    }
    impl Component for AssetZero {
        type Storage = DenseVecStorage<Self>;
    }
    impl From<AssetZero> for Result<ProcessingState<AssetZero>, assets::Error> {
        fn from(asset_zero: AssetZero) -> Result<ProcessingState<AssetZero>, assets::Error> {
            Ok(ProcessingState::Loaded(asset_zero))
        }
    }
    type AssetZeroHandle = Handle<AssetZero>;

    // === System delegates === //
    struct AssetZeroLoader;
    impl AssetZeroLoader {
        fn load(world: &World, asset_zero: AssetZero) -> Result<AssetZeroHandle, amethyst::Error> {
            let loader = world.read_resource::<Loader>();
            Ok(loader.load_from_data(
                asset_zero,
                (),
                &world.read_resource::<AssetStorage<AssetZero>>(),
            ))
        }
    }

    // === Components === //
    struct ComponentZero(pub i32);
    impl Component for ComponentZero {
        type Storage = DenseVecStorage<Self>;
    }
}
