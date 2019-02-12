use std::{fmt::Debug, marker::PhantomData};

use amethyst::{GameData, State, StateData, Trans};
use application_event::AppEvent;
use application_state::AutexState;
use application_ui::ThemeLoader;
use derivative::Derivative;
use log::{debug, error};

use crate::LoadingStatus;

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
    /// The `State` that follows this one.
    #[derivative(Debug(bound = "S: Debug"))]
    next_state: Option<S>,
    /// Lifetime tracker.
    phantom_data: PhantomData<dyn AutexState<'a, 'b>>,
}

impl<'a, 'b, S> LoadingState<'a, 'b, S>
where
    S: AutexState<'a, 'b>,
{
    /// Returns a new `State`
    pub fn new(next_state: S) -> Self {
        LoadingState {
            next_state: Some(next_state),
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
    }

    fn update(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
    ) -> Trans<GameData<'a, 'b>, AppEvent> {
        data.data.update(&data.world);

        if *data.world.read_resource::<LoadingStatus>() == LoadingStatus::Complete {
            Trans::Switch(Box::new(
                self.next_state
                    .take()
                    .expect("Expected `next_state` to be set"),
            ))
        } else {
            debug!(
                "If loading never completes, please ensure that you have registered the following \
                 bundles with the application dispatcher:\n\
                 \n\
                 * `SpriteLoadingBundle`\n\
                 * `CharacterLoadingBundle`\n\
                 * `MapLoadingBundle`\n\
                 \n\
                 These provide the necessary `System`s to process the loaded assets.\n"
            );

            Trans::None
        }
    }
}
