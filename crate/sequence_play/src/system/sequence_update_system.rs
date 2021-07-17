use amethyst::{
    assets::AssetStorage,
    ecs::{Entities, Entity, Join, Read, ReadStorage, System, World, Write, WriteStorage},
    shred::{ResourceId, SystemData},
    shrev::EventChannel,
};
use derivative::Derivative;
use derive_new::new;
use sequence_model::{
    loaded::{WaitSequence, WaitSequenceHandle},
    play::{
        FrameFreezeClock, FrameIndexClock, FrameWaitClock, SequenceStatus, SequenceUpdateEvent,
    },
};

/// Ticks the logic clocks for sequences, and sends `SequenceUpdateEvent`s.
///
/// The logic clocks include:
///
/// * `FrameFreezeClock`
/// * `FrameWaitClock`
/// * `FrameIndexClock`
///
/// This system **must** be run before all systems that update the frame
/// components that are attached to entities, as the `SequenceUpdateEvent`s
/// include the new frame index, which is only guaranteed to be valid for the
/// current dispatcher run.
#[derive(Debug, Default, new)]
pub struct SequenceUpdateSystem;

/// `SequenceUpdateSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct SequenceUpdateSystemData<'s> {
    /// `Entities`.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// `WaitSequenceHandle` component storage.
    #[derivative(Debug = "ignore")]
    pub wait_sequence_handles: ReadStorage<'s, WaitSequenceHandle>,
    /// `WaitSequence` assets.
    #[derivative(Debug = "ignore")]
    pub wait_sequence_assets: Read<'s, AssetStorage<WaitSequence>>,
    /// `FrameIndexClock` component storage.
    #[derivative(Debug = "ignore")]
    pub frame_index_clocks: WriteStorage<'s, FrameIndexClock>,
    /// `FrameFreezeClock` component storage.
    #[derivative(Debug = "ignore")]
    pub frame_freeze_clocks: WriteStorage<'s, FrameFreezeClock>,
    /// `FrameWaitClock` component storage.
    #[derivative(Debug = "ignore")]
    pub frame_wait_clocks: WriteStorage<'s, FrameWaitClock>,
    /// `SequenceStatus` component storage.
    #[derivative(Debug = "ignore")]
    pub sequence_statuses: WriteStorage<'s, SequenceStatus>,
    /// Event channel for `SequenceUpdateEvent`s.
    #[derivative(Debug = "ignore")]
    pub sequence_update_ec: Write<'s, EventChannel<SequenceUpdateEvent>>,
}

#[derive(Debug)]
struct SequenceUpdateParams<'p> {
    entity: Entity,
    wait_sequence_handle: &'p WaitSequenceHandle,
    frame_index_clock: &'p mut FrameIndexClock,
    frame_wait_clock: &'p mut FrameWaitClock,
    sequence_status: &'p mut SequenceStatus,
}

impl SequenceUpdateSystem {
    fn start_sequence(
        wait_sequence_assets: &AssetStorage<WaitSequence>,
        SequenceUpdateParams {
            entity: _entity,
            wait_sequence_handle,
            frame_index_clock,
            frame_wait_clock,
            sequence_status,
        }: SequenceUpdateParams,
    ) {
        frame_index_clock.reset();
        frame_wait_clock.reset();

        // Set to ongoing, meaning we must be sure that this is the only system
        // that needs to read the `SequenceStatus::Begin` status.
        *sequence_status = SequenceStatus::Ongoing;

        // Update the frame_index_clock limit because we already hold a mutable
        // borrow of the component storage.
        let wait_sequence = wait_sequence_assets
            .get(wait_sequence_handle)
            .expect("Expected `WaitSequence` to be loaded.");
        (*frame_index_clock).limit = wait_sequence.len();

        Self::update_frame_wait_clock_limit(wait_sequence, frame_wait_clock, 0);
    }

    /// Returns true if the entity is **not frozen**, ticks the clock otherwise.
    fn entity_unfrozen_tick(
        frame_freeze_clocks: &mut WriteStorage<'_, FrameFreezeClock>,
        entity: Entity,
    ) -> bool {
        frame_freeze_clocks
            .get_mut(entity)
            .map(|frame_freeze_clock| {
                if frame_freeze_clock.is_complete() {
                    true
                } else {
                    frame_freeze_clock.tick();
                    false
                }
            })
            .unwrap_or(true)
    }

    fn entity_frame_wait_tick(
        wait_sequence_assets: &AssetStorage<WaitSequence>,
        sequence_update_ec: &mut EventChannel<SequenceUpdateEvent>,
        sequence_update_params: SequenceUpdateParams,
    ) {
        let SequenceUpdateParams {
            entity,
            wait_sequence_handle,
            frame_index_clock,
            frame_wait_clock,
            sequence_status,
        } = sequence_update_params;

        frame_wait_clock.tick();

        if frame_wait_clock.is_complete() {
            // Switch to next frame, or if there is no next frame, switch
            // `SequenceStatus` to `End`.

            frame_index_clock.tick();

            if frame_index_clock.is_complete() {
                *sequence_status = SequenceStatus::End;

                let frame_index = (*frame_index_clock).value.saturating_sub(1);
                sequence_update_ec.single_write(SequenceUpdateEvent::SequenceEnd {
                    entity,
                    frame_index,
                });
            } else {
                frame_wait_clock.reset();

                let frame_index = (*frame_index_clock).value;

                // Update limit for `FrameWaitClock`.
                let wait_sequence = wait_sequence_assets
                    .get(wait_sequence_handle)
                    .expect("Expected `WaitSequence` to be loaded.");

                Self::update_frame_wait_clock_limit(wait_sequence, frame_wait_clock, frame_index);

                sequence_update_ec.single_write(SequenceUpdateEvent::FrameBegin {
                    entity,
                    frame_index,
                });
            }
        }
    }

    fn update_frame_wait_clock_limit(
        wait_sequence: &WaitSequence,
        frame_wait_clock: &mut FrameWaitClock,
        frame_index: usize,
    ) {
        let wait = wait_sequence.get(frame_index).unwrap_or_else(|| {
            panic!(
                "Expected wait sequence to have frame index: `{}`. `WaitSequence`: {:?}",
                frame_index, wait_sequence
            )
        });
        (*frame_wait_clock).limit = **wait as usize;
    }
}

impl<'s> System<'s> for SequenceUpdateSystem {
    type SystemData = SequenceUpdateSystemData<'s>;

    fn run(
        &mut self,
        SequenceUpdateSystemData {
            entities,
            wait_sequence_handles,
            wait_sequence_assets,
            mut frame_index_clocks,
            mut frame_freeze_clocks,
            mut frame_wait_clocks,
            mut sequence_statuses,
            mut sequence_update_ec,
        }: Self::SystemData,
    ) {
        (
            &entities,
            &wait_sequence_handles,
            &mut frame_index_clocks,
            &mut frame_wait_clocks,
            &mut sequence_statuses,
        )
            .join()
            .for_each(
                |(
                    entity,
                    wait_sequence_handle,
                    mut frame_index_clock,
                    mut frame_wait_clock,
                    mut sequence_status,
                )| {
                    let sequence_update_params = SequenceUpdateParams {
                        entity,
                        wait_sequence_handle,
                        frame_index_clock: &mut frame_index_clock,
                        frame_wait_clock: &mut frame_wait_clock,
                        sequence_status: &mut sequence_status,
                    };
                    match sequence_update_params.sequence_status {
                        SequenceStatus::Begin => {
                            Self::start_sequence(&wait_sequence_assets, sequence_update_params);
                        }
                        SequenceStatus::Ongoing => {
                            if Self::entity_unfrozen_tick(&mut frame_freeze_clocks, entity) {
                                Self::entity_frame_wait_tick(
                                    &wait_sequence_assets,
                                    &mut sequence_update_ec,
                                    sequence_update_params,
                                );
                            }
                        }
                        SequenceStatus::End => {
                            Self::entity_unfrozen_tick(&mut frame_freeze_clocks, entity);
                        }
                    }
                },
            );
    } // kcov-ignore
}
