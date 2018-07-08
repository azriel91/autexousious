#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Test harness to support testing of Amethyst types, including:
//!
//! * `Bundle`
//! * `State`
//! * `System`
//! * Resource loading.
//! * Arbitrary types that `System`s use during processing.
//!
//! The test harness minimizes boilerplate code to set up an Amethyst `Application` with common
//! bundles, and can take in logic that is normally masked behind a number of layers through a thin
//! interface.
//!
//! # Usage
//!
//! The following shows an example of testing a `State`. More examples are in the [Examples](#Examples)
//! section.
//!
//! ```rust
//! # extern crate amethyst;
//! # extern crate amethyst_test_support;
//! #
//! # use std::marker::PhantomData;
//! #
//! # use amethyst_test_support::prelude::*;
//! # use amethyst::{
//! #     core::bundle::{self, SystemBundle},
//! #     ecs::prelude::*,
//! #     prelude::*,
//! # };
//! #
//! # #[derive(Debug)]
//! # struct LoadResource;
//! #
//! # struct LoadingState<'a, 'b, S>
//! # where
//! #     S: State<GameData<'a, 'b>> + 'static,
//! # {
//! #     next_state: Option<S>,
//! #     state_data: PhantomData<State<GameData<'a, 'b>>>,
//! # }
//! # impl<'a, 'b, S> LoadingState<'a, 'b, S>
//! # where
//! #     S: State<GameData<'a, 'b>> + 'static,
//! # {
//! #     fn new(next_state: S) -> Self {
//! #         LoadingState {
//! #             next_state: Some(next_state),
//! #             state_data: PhantomData,
//! #         }
//! #     }
//! # }
//! # impl<'a, 'b, S> State<GameData<'a, 'b>> for LoadingState<'a, 'b, S>
//! # where
//! #     S: State<GameData<'a, 'b>> + 'static,
//! # {
//! #     fn update(&mut self, data: StateData<GameData>) -> Trans<GameData<'a, 'b>> {
//! #         data.data.update(&data.world);
//! #         data.world.add_resource(LoadResource);
//! #         Trans::Switch(Box::new(self.next_state.take().unwrap()))
//! #     }
//! # }
//! #
//! // #[test]
//! fn assertion_push_with_loading_state_with_add_resource_succeeds() {
//!     let assertion = |world: &mut World| {
//!         world.read_resource::<LoadResource>();
//!     };
//!
//!     assert!(
//!         AmethystApplication::blank()
//!             .with_state(|| LoadingState::new(EmptyState))
//!             .with_assertion(assertion)
//!             .run()
//!             .is_ok()
//!     );
//! }
//! #
//! # fn main() {
//! #     assertion_push_with_loading_state_with_add_resource_succeeds();
//! # }
//! ```
//!
//! The Amethyst application is initialized with one of the following functions, each providing a
//! different level of default bundles:
//!
//! ```rust,ignore
//! extern crate amethyst_test_support;
//!
//! use amethyst_test_support::prelude::*;
//!
//! #[test]
//! fn test_name() {
//!     // Start with no bundles
//!     AmethystApplication::blank();
//!
//!     // Start with the Transform, Input, and UI bundles
//!     AmethystApplication::base();
//!
//!     // Start with the Animation, Transform, Input, UI, and Render bundles.
//!     let visibility = false; // Whether the window should be shown
//!     AmethystApplication::render_base("test_name", visibility);
//! }
//! ```
//!
//! Next, attach the logic you wish to test using the various `.with_*(..)` methods:
//!
//! ```rust,ignore
//! #[test]
//! fn test_name() {
//!     let effect = |world: &mut World| {
//!          let entity = world.create_entity().with(MyComponent(0)).build();
//!
//!         // `EffectReturn` is a convenience wrapper struct to pass values between this effect
//!         // function and the assertion function.
//!         world.add_resource(EffectReturn(entity));
//!     };
//!
//!     let assertion = |world: &mut World| {
//!         let entity = world.read_resource::<EffectReturn<Entity>>().0.clone();
//!
//!         let my_component_storage = world.read_storage::<MyComponent>();
//!         let my_component = component_zero_storage
//!             .get(entity)
//!             .expect("Entity should have a `MyComponent` component.");
//!
//!         // If "my_system" ran, the value in the `MyComponent` should be 1.
//!         assert_eq!(1, my_component.0);
//!     };
//!
//!     let visibility = false; // Whether the window should be shown
//!     AmethystApplication::render_base("test_name", visibility)
//!         .with_bundle(MyBundle::new()) // Can be invoked multiple times.
//!         .with_bundle_fn(|| MyNonSendBundle::new()) // Can be invoked multiple times.
//!         .with_state(|| MyState::new()) // Must be before any calls to `.with_resource(R)`.
//!                                        // Can only be invoked once.
//!         .with_system(MySystem::new(), "my_system", &[]) // Can be invoked multiple times.
//!         .with_effect(effect) // Can only be invoked once.
//!         .with_assertion(assertion) // Can only be invoked once.
//!         .with_resource(MyResource::new()) // Adds a resource to the world.
//!                                           // Must be after any calls to `.with_state(S)`.
//!                                           // Can be invoked multiple times.
//!          // ...
//! }
//! ```
//!
//! Finally, call `.run()` to run the application. This returns `amethyst::Result<()>`, so you can
//! wrap it in an `assert!(..);`:
//!
//! ```rust,ignore
//! #[test]
//! fn test_name() {
//!     let effect = // ...
//!     let assertion = // ...
//!
//!     let visibility = false; // Whether the window should be shown
//!     assert!(
//!         AmethystApplication::render_base("test_name", visibility)
//!             // ...
//!             .with_effect(effect)
//!             .with_assertion(assertion)
//!             .run()
//!             .is_ok()
//!     );
//! }
//! ```
//!
//! # Examples
//!
//! Testing a bundle:
//!
//! ```rust
//! # extern crate amethyst;
//! # extern crate amethyst_test_support;
//! #
//! # use amethyst_test_support::prelude::*;
//! # use amethyst::{
//! #     core::bundle::{self, SystemBundle},
//! #     ecs::prelude::*,
//! #     prelude::*,
//! # };
//! #
//! # #[derive(Debug)]
//! # struct ApplicationResource;
//! #
//! # #[derive(Debug)]
//! # struct MySystem;
//! # type MySystemData<'s> = ReadExpect<'s, ApplicationResource>;
//! # impl<'s> System<'s> for MySystem {
//! #     type SystemData = MySystemData<'s>;
//! #     fn run(&mut self, _: Self::SystemData) {}
//! #     fn setup(&mut self, res: &mut Resources) {
//! #         MySystemData::setup(res);
//! #         res.insert(ApplicationResource);
//! #     }
//! # }
//! #
//! # #[derive(Debug)]
//! # struct MyBundle;
//! # impl<'a, 'b> SystemBundle<'a, 'b> for MyBundle {
//! #     fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> bundle::Result<()> {
//! #         builder.add(MySystem, "my_system", &[]);
//! #         Ok(())
//! #     }
//! # }
//! #
//! // #[test]
//! fn bundle_registers_system_with_resource() {
//!     assert!(
//!         AmethystApplication::blank()
//!             .with_bundle(MyBundle)
//!             .with_assertion(|world| { world.read_resource::<ApplicationResource>(); })
//!             .run()
//!             .is_ok()
//!     );
//! }
//! #
//! # fn main() {
//! #     bundle_registers_system_with_resource();
//! # }
//! ```
//!
//! Testing a system:
//!
//! ```rust
//! # extern crate amethyst;
//! # extern crate amethyst_test_support;
//! #
//! # use amethyst_test_support::prelude::*;
//! # use amethyst::{
//! #     ecs::prelude::*,
//! #     prelude::*,
//! # };
//! #
//! # struct MyComponent(pub i32);
//! # impl Component for MyComponent {
//! #     type Storage = DenseVecStorage<Self>;
//! # }
//! #
//! # #[derive(Debug)]
//! # struct MySystem;
//! # type MySystemData<'s> = WriteStorage<'s, MyComponent>;
//! # impl<'s> System<'s> for MySystem {
//! #     type SystemData = MySystemData<'s>;
//! #     fn run(&mut self, mut my_component_storage: Self::SystemData) {
//! #         for mut my_component in (&mut my_component_storage).join() {
//! #             my_component.0 += 1
//! #         }
//! #     }
//! # }
//! #
//! // #[test]
//! fn system_increases_component_value_by_one() {
//!     let effect_fn = |world: &mut World| {
//!         let entity = world.create_entity().with(MyComponent(0)).build();
//!
//!         world.add_resource(EffectReturn(entity));
//!     };
//!     let assertion = |world: &mut World| {
//!         let entity = world.read_resource::<EffectReturn<Entity>>().0.clone();
//!
//!         let my_component_storage = world.read_storage::<MyComponent>();
//!         let my_component = my_component_storage
//!             .get(entity)
//!             .expect("Entity should have a `MyComponent` component.");
//!
//!         // If the system ran, the value in the `MyComponent` should be 1.
//!         assert_eq!(1, my_component.0);
//!     };
//!
//!     assert!(
//!         AmethystApplication::blank()
//!             .with_system(MySystem, "my_system", &[])
//!             .with_effect(effect_fn)
//!             .with_assertion(assertion)
//!             .run()
//!             .is_ok()
//!     );
//! }
//! #
//! # fn main() {
//! #     system_increases_component_value_by_one();
//! # }
//! ```
//!

extern crate amethyst;
extern crate boxfnonce;
#[macro_use]
extern crate derivative;
#[macro_use]
extern crate derive_new;
extern crate hetseq;
#[macro_use]
extern crate lazy_static;

pub use amethyst_application::{AmethystApplication, HIDPI, SCREEN_HEIGHT, SCREEN_WIDTH};
pub use effect_return::EffectReturn;
pub use fixture::MaterialAnimationFixture;
pub use game_update::GameUpdate;
pub use state::{EmptyState, FunctionState, SchedulerState};
pub(crate) use system_injection_bundle::SystemInjectionBundle;

mod amethyst_application;
mod effect_return;
mod fixture;
mod game_update;
pub mod prelude;
mod state;
mod system_injection_bundle;
