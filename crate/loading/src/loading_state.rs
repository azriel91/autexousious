use std::fmt::Debug;
use std::marker::PhantomData;
use std::path::PathBuf;

use amethyst::{assets::ProgressCounter, prelude::*};
use application_ui::ThemeLoader;

use AssetLoader;

/// `State` where resource loading takes place.
///
/// If you use this `State`, you **MUST** ensure that both the `ObjectLoadingBundle` and
/// `MapLoadingBundle`s are included in the application dispatcher that this `State` delegates to
/// to load the assets.
///
/// # Type Parameters
///
/// * `S`: State to return after loading is complete.
#[derive(Derivative)]
#[derivative(Debug)]
pub struct LoadingState<'a, 'b, S>
where
    S: State<GameData<'a, 'b>> + 'static,
{
    /// Path to the assets directory.
    assets_dir: PathBuf,
    /// The `State` that follows this one.
    #[derivative(Debug(bound = "S: Debug"))]
    next_state: Option<Box<S>>,
    /// Tracks loaded assets.
    #[derivative(Debug = "ignore")]
    progress_counter: ProgressCounter,
    /// Lifetime tracker.
    state_data: PhantomData<State<GameData<'a, 'b>>>,
}

impl<'a, 'b, S> LoadingState<'a, 'b, S>
where
    S: State<GameData<'a, 'b>> + 'static,
{
    /// Returns a new `State`
    pub fn new(assets_dir: PathBuf, next_state: Box<S>) -> Self {
        LoadingState {
            assets_dir,
            next_state: Some(next_state),
            progress_counter: ProgressCounter::new(),
            state_data: PhantomData,
        }
    }
}

impl<'a, 'b, S> State<GameData<'a, 'b>> for LoadingState<'a, 'b, S>
where
    S: State<GameData<'a, 'b>> + 'static,
{
    fn on_start(&mut self, mut data: StateData<GameData>) {
        if let Err(e) = ThemeLoader::load(&mut data.world) {
            let err_msg = format!("Failed to load theme: {}", e);
            error!("{}", &err_msg);
            panic!(err_msg);
        }

        AssetLoader::load_game_config(
            &mut data.world,
            &self.assets_dir,
            &mut self.progress_counter,
        );
    }

    fn update(&mut self, data: StateData<GameData>) -> Trans<GameData<'a, 'b>> {
        data.data.update(&data.world);

        if self.progress_counter.is_complete() {
            Trans::Switch(
                self.next_state
                    .take()
                    .expect("Expected `next_state` to be set"),
            )
        } else {
            warn!(
                "If loading never completes, please ensure that you have registered both the \
                 `ObjectLoadingBundle` and `MapLoadingBundle`s to the application dispatcher, as \
                 those provide the necessary `System`s to process the loaded assets."
            );
            debug!(
                "Loading progress: {}/{}",
                self.progress_counter.num_finished(),
                self.progress_counter.num_assets()
            );

            Trans::None
        }
    }
}
