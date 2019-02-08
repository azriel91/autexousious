use std::{fmt::Debug, marker::PhantomData, path::PathBuf};

use amethyst::{assets::ProgressCounter, prelude::*};
use application_event::AppEvent;
use application_state::AutexState;
use application_ui::ThemeLoader;
use derivative::Derivative;
use log::{debug, error, warn};

use crate::AssetLoader;

/// `State` where resource loading takes place.
///
/// If you use this `State`, you **MUST** ensure that both the `CharacterLoadingBundle` and
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
    S: AutexState<'a, 'b>,
{
    /// Path to the assets directory.
    assets_dir: PathBuf,
    /// The `State` that follows this one.
    #[derivative(Debug(bound = "S: Debug"))]
    next_state: Option<S>,
    /// Tracks loaded assets.
    #[derivative(Debug = "ignore")]
    progress_counter: ProgressCounter,
    /// Lifetime tracker.
    phantom_data: PhantomData<dyn AutexState<'a, 'b>>,
}

impl<'a, 'b, S> LoadingState<'a, 'b, S>
where
    S: AutexState<'a, 'b>,
{
    /// Returns a new `State`
    pub fn new(assets_dir: PathBuf, next_state: S) -> Self {
        LoadingState {
            assets_dir,
            next_state: Some(next_state),
            progress_counter: ProgressCounter::new(),
            phantom_data: PhantomData,
        }
    }
}

impl<'a, 'b, S> State<GameData<'a, 'b>, AppEvent> for LoadingState<'a, 'b, S>
where
    S: AutexState<'a, 'b> + 'static,
{
    fn on_start(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
        if let Err(e) = ThemeLoader::load(&mut data.world) {
            let err_msg = format!("Failed to load theme: {}", e);
            error!("{}", &err_msg);
            panic!(err_msg);
        }

        AssetLoader::load(
            &mut data.world,
            &self.assets_dir,
            &mut self.progress_counter,
        );
    }

    fn update(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
    ) -> Trans<GameData<'a, 'b>, AppEvent> {
        data.data.update(&data.world);

        if self.progress_counter.is_complete() {
            Trans::Switch(Box::new(
                self.next_state
                    .take()
                    .expect("Expected `next_state` to be set"),
            ))
        } else {
            warn!(
                "If loading never completes, please ensure that you have registered both the \
                 `CharacterLoadingBundle` and `MapLoadingBundle`s to the application dispatcher, as \
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
