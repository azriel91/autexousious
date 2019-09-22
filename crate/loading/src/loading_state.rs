use std::{fmt::Debug, marker::PhantomData, time::Duration};

use amethyst::{
    core::Stopwatch,
    ecs::{Read, WorldExt},
    GameData, State, StateData, Trans,
};
use application_event::AppEvent;
use application_state::AutexState;
use application_ui::ThemeLoader;
use asset_model::loaded::AssetTypeMappings;
use character_loading::CharacterLoadingStatus;
use collision_audio_model::CollisionAudioLoadingStatus;
use derivative::Derivative;
use humantime;
use loading_model::loaded::{AssetLoadStatus, LoadStatus};
use log::{error, warn};
use object_loading::ObjectLoadingStatus;
use state_registry::StateId;
use ui_audio_model::UiAudioLoadingStatus;

use crate::MapLoadingStatus;

/// Time limit before outputting a warning message and transitioning to the next state.
const LOADING_TIME_LIMIT: Duration = Duration::from_secs(10);

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
    /// Tracks how long the `LoadingState` has run.
    ///
    /// Used to output a warning if loading takes too long.
    stopwatch: Stopwatch,
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
            stopwatch: Stopwatch::new(),
            phantom_data: PhantomData,
        }
    }
}

impl<'a, 'b, S> State<GameData<'a, 'b>, AppEvent> for LoadingState<'a, 'b, S>
where
    S: AutexState<'a, 'b> + 'static,
{
    fn on_start(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
        data.world.insert(StateId::Loading);
        self.stopwatch.start();

        if let Err(e) = ThemeLoader::load(&mut data.world) {
            let err_msg = format!("Failed to load theme: {}", e);
            error!("{}", &err_msg);
            panic!(err_msg);
        }
    }

    fn on_resume(&mut self, data: StateData<'_, GameData<'a, 'b>>) {
        data.world.insert(StateId::Loading);
        self.stopwatch.restart();
    }

    fn update(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
    ) -> Trans<GameData<'a, 'b>, AppEvent> {
        data.data.update(&data.world);

        let loading_statuses_complete = **data.world.read_resource::<CharacterLoadingStatus>()
            == ObjectLoadingStatus::Complete
            && *data.world.read_resource::<MapLoadingStatus>() == MapLoadingStatus::Complete
            && *data.world.read_resource::<CollisionAudioLoadingStatus>()
                == CollisionAudioLoadingStatus::Complete
            && *data.world.read_resource::<UiAudioLoadingStatus>()
                == UiAudioLoadingStatus::Complete;

        let asset_load_statuses_complete = {
            let (asset_type_mappings, asset_load_status) = data
                .world
                .system_data::<(Read<'_, AssetTypeMappings>, Read<'_, AssetLoadStatus>)>();

            let asset_loading_complete = asset_type_mappings
                .iter_ids_all()
                .flat_map(|(_asset_type, asset_ids)| asset_ids.iter())
                .try_fold((), |_, asset_id| {
                    let load_status =
                        asset_load_status
                            .get(*asset_id)
                            .copied()
                            .unwrap_or_else(|| {
                                panic!("Expected asset `{:?}` to have `LoadStatus`.", asset_id)
                            });

                    if load_status == LoadStatus::Complete {
                        Ok(())
                    } else {
                        if let Stopwatch::Ended(..) = &self.stopwatch {
                            warn!(
                                "Asset ID `{:?}` has not completed loading. {:?}",
                                asset_id, load_status
                            );
                        }
                        Err(())
                    }
                })
                .is_ok();

            asset_loading_complete
        };

        if loading_statuses_complete && asset_load_statuses_complete {
            Trans::Switch(Box::new(
                self.next_state
                    .take()
                    .expect("Expected `next_state` to be set"),
            ))
        } else {
            if let Stopwatch::Started(..) = &self.stopwatch {
                let elapsed = self.stopwatch.elapsed();
                if elapsed > LOADING_TIME_LIMIT {
                    self.stopwatch.stop();

                    let duration = humantime::Duration::from(elapsed);

                    warn!(
                        "Loading has not completed in {}, please ensure that you have registered \
                         the following bundles with the application dispatcher:\n\
                         \n\
                         * `SpriteLoadingBundle`\n\
                         * `CharacterLoadingBundle`\n\
                         * `CharacterPrefabBundle`\n\
                         * `MapLoadingBundle`\n\
                         * `amethyst::audio::AudioBundle`\n\
                         * `KinematicLoadingBundle`\n\
                         * `CollisionAudioLoadingBundle`\n\
                         * `UiAudioLoadingBundle`\n\
                         \n\
                         These provide the necessary `System`s to process the loaded assets.\n",
                        duration
                    );
                }
            }

            Trans::None
        }
    }
}
