use amethyst::{
    assets::AssetStorage,
    ecs::{Entity, Read, ReadStorage, System, SystemData, WriteStorage},
    shred::Resources,
    shrev::{EventChannel, ReaderId},
};
use derivative::Derivative;
use derive_new::new;
use named_type::NamedType;
use named_type_derive::NamedType;
use object_loading::FrameComponentStorages;
use sequence_model::{
    entity::{FrameIndexClock, FrameWaitClock},
    loaded::{ComponentSequence, ComponentSequences, ComponentSequencesHandle},
};
use shred_derive::SystemData;

use crate::SequenceUpdateEvent;

/// Updates frame components.
#[derive(Debug, Default, NamedType, new)]
pub struct FrameComponentUpdateSystem {
    /// Reader ID for the `SequenceUpdateEvent` event channel.
    #[new(default)]
    reader_id: Option<ReaderId<SequenceUpdateEvent>>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct FrameComponentUpdateSystemData<'s> {
    /// Event channel for `SequenceUpdateEvent`s.
    #[derivative(Debug = "ignore")]
    pub sequence_update_ec: Read<'s, EventChannel<SequenceUpdateEvent>>,
    /// `ComponentSequencesHandle` component storage.
    #[derivative(Debug = "ignore")]
    pub component_sequences_handles: ReadStorage<'s, ComponentSequencesHandle>,
    /// `ComponentSequences` assets.
    #[derivative(Debug = "ignore")]
    pub component_sequences_assets: Read<'s, AssetStorage<ComponentSequences>>,
    /// `FrameIndexClock` component storage.
    #[derivative(Debug = "ignore")]
    pub frame_index_clocks: ReadStorage<'s, FrameIndexClock>,
    /// `FrameWaitClock` component storage.
    #[derivative(Debug = "ignore")]
    pub frame_wait_clocks: WriteStorage<'s, FrameWaitClock>,
    /// Frame `Component` storages.
    pub frame_component_storages: FrameComponentStorages<'s>,
}

impl FrameComponentUpdateSystem {
    fn update_frame_components(
        frame_wait_clocks: &mut WriteStorage<'_, FrameWaitClock>,
        frame_component_storages: &mut FrameComponentStorages,
        component_sequences: &ComponentSequences,
        entity: Entity,
        frame_index: usize,
    ) {
        let FrameComponentStorages {
            ref mut waits,
            ref mut sprite_renders,
            ref mut bodies,
            ref mut interactionses,
        } = frame_component_storages;

        let frame_wait_clock = frame_wait_clocks
            .get_mut(entity)
            .expect("Expected entity to have a `FrameWaitClock` component.");

        component_sequences
            .iter()
            .for_each(|component_sequence| match component_sequence {
                ComponentSequence::Wait(wait_sequence) => {
                    let wait = wait_sequence[frame_index];
                    waits
                        .insert(entity, wait)
                        .expect("Failed to insert `Wait` component.");

                    (*frame_wait_clock).limit = *wait as usize;
                }
                ComponentSequence::SpriteRender(sprite_render_sequence) => {
                    let sprite_render = sprite_render_sequence[frame_index].clone();
                    sprite_renders
                        .insert(entity, sprite_render)
                        .expect("Failed to insert `SpriteRender` component.");
                }
                ComponentSequence::Body(body_sequence) => {
                    let body = body_sequence[frame_index].clone();
                    bodies
                        .insert(entity, body)
                        .expect("Failed to insert `Body` component.");
                }
                ComponentSequence::Interactions(interactions_sequence) => {
                    let interactions = interactions_sequence[frame_index].clone();
                    interactionses
                        .insert(entity, interactions)
                        .expect("Failed to insert `Interactions` component.");
                }
            });
    }
}

impl<'s> System<'s> for FrameComponentUpdateSystem {
    type SystemData = FrameComponentUpdateSystemData<'s>;

    fn run(
        &mut self,
        FrameComponentUpdateSystemData {
            sequence_update_ec,
            component_sequences_handles,
            component_sequences_assets,
            frame_index_clocks,
            mut frame_wait_clocks,
            mut frame_component_storages,
        }: Self::SystemData,
    ) {
        sequence_update_ec
            .read(
                self.reader_id
                    .as_mut()
                    .expect("Expected reader ID to exist for FrameComponentUpdateSystem."),
            )
            .for_each(|ev| {
                let (entity, frame_index) = match ev {
                    SequenceUpdateEvent::SequenceBegin { entity }
                    | SequenceUpdateEvent::FrameBegin { entity } => {
                        let frame_index_clock = frame_index_clocks
                            .get(*entity)
                            .expect("Expected entity to have a `FrameIndexClock` component.");
                        let frame_index = (*frame_index_clock).value;

                        (entity, frame_index)
                    }
                    SequenceUpdateEvent::SequenceEnd { entity } => (entity, 0),
                };

                let component_sequences_handle = component_sequences_handles
                    .get(*entity)
                    .expect("Expected entity to have a `ComponentSequencesHandle` component.");
                let component_sequences = component_sequences_assets
                    .get(component_sequences_handle)
                    .expect("Expected component_sequences to be loaded.");

                Self::update_frame_components(
                    &mut frame_wait_clocks,
                    &mut frame_component_storages,
                    component_sequences,
                    *entity,
                    frame_index,
                );
            });
    }

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        self.reader_id = Some(
            res.fetch_mut::<EventChannel<SequenceUpdateEvent>>()
                .register_reader(),
        );
    }
}

#[cfg(test)]
mod tests {
    use amethyst::{
        assets::AssetStorage,
        ecs::{Entities, Join, ReadStorage, World, WriteStorage},
        shrev::EventChannel,
        Error,
    };
    use application_test_support::{AutexousiousApplication, SequenceQueries};
    use assets_test::ASSETS_CHAR_BAT_SLUG;
    use character_model::config::CharacterSequenceId;
    use collision_model::config::{
        Body, Hit, HitRepeatDelay, Interaction, InteractionKind, Interactions,
    };
    use object_loading::FrameComponentStorages;
    use sequence_model::{
        config::Wait,
        entity::{FrameIndexClock, FrameWaitClock},
        loaded::ComponentSequencesHandle,
    };
    use shape_model::Volume;

    use super::FrameComponentUpdateSystem;
    use crate::SequenceUpdateEvent;

    #[test]
    fn updates_all_frame_components_on_sequence_begin_event() -> Result<(), Error> {
        let test_name = "updates_all_frame_components_on_sequence_begin_event";
        AutexousiousApplication::game_base(test_name, false)
            .with_system(FrameComponentUpdateSystem::new(), "", &[])
            .with_setup(|world| {
                let component_sequences_handle = SequenceQueries::component_sequences_handle(
                    world,
                    &ASSETS_CHAR_BAT_SLUG.clone(),
                    CharacterSequenceId::StandAttack,
                );
                initial_values(
                    world,
                    // third frame in the sequence, though it doesn't make sense for sequence begin
                    2,
                    5,
                    0,
                    5,
                    component_sequences_handle,
                )
            })
            .with_setup(|world| {
                let events = sequence_begin_events(world);
                send_events(world, events);
            })
            .with_assertion(|world| expect_values(world, 0, 2))
            .with_assertion(|world| {
                expect_component_values(world, Wait::new(2), 2, body(), interactions())
            })
            .run()
    }

    #[test]
    fn updates_all_frame_components_on_frame_begin_event() -> Result<(), Error> {
        let test_name = "updates_all_frame_components_on_frame_begin_event";
        AutexousiousApplication::game_base(test_name, false)
            .with_system(FrameComponentUpdateSystem::new(), "", &[])
            .with_setup(|world| {
                let component_sequences_handle = SequenceQueries::component_sequences_handle(
                    world,
                    &ASSETS_CHAR_BAT_SLUG.clone(),
                    CharacterSequenceId::StandAttack,
                );
                initial_values(
                    world,
                    2, // third frame in the sequence
                    5,
                    0,
                    5,
                    component_sequences_handle,
                )
            })
            .with_setup(|world| {
                let events = frame_begin_events(world);
                send_events(world, events);
            })
            .with_assertion(|world| expect_values(world, 0, 2))
            .with_assertion(|world| {
                expect_component_values(world, Wait::new(2), 2, body(), interactions())
            })
            .run()
    }

    fn initial_values(
        world: &mut World,
        frame_index_clock_value: usize,
        frame_index_clock_limit: usize,
        frame_wait_clock_value: usize,
        frame_wait_clock_limit: usize,
        component_sequences_handle_initial: ComponentSequencesHandle,
    ) {
        let (
            _entities,
            mut frame_index_clocks,
            mut frame_wait_clocks,
            mut component_sequences_handles,
            ..
        ) = world.system_data::<TestSystemData>();

        (
            &mut frame_index_clocks,
            &mut frame_wait_clocks,
            &mut component_sequences_handles,
        )
            .join()
            .for_each(
                |(frame_index_clock, frame_wait_clock, component_sequences_handle)| {
                    (*frame_index_clock).value = frame_index_clock_value;
                    (*frame_index_clock).limit = frame_index_clock_limit;

                    (*frame_wait_clock).value = frame_wait_clock_value;
                    (*frame_wait_clock).limit = frame_wait_clock_limit;

                    *component_sequences_handle = component_sequences_handle_initial.clone();
                },
            );
    }

    fn expect_values(
        world: &mut World,
        frame_wait_clock_value: usize,
        frame_wait_clock_limit: usize,
    ) {
        let (frame_wait_clocks, sequence_statuses) = world.system_data::<(
            WriteStorage<FrameWaitClock>,
            ReadStorage<CharacterSequenceId>,
        )>();

        (&frame_wait_clocks, &sequence_statuses).join().for_each(
            |(frame_wait_clock, _sequence_status)| {
                assert_eq!(frame_wait_clock_value, (*frame_wait_clock).value);
                assert_eq!(frame_wait_clock_limit, (*frame_wait_clock).limit);
            },
        );
    }

    fn expect_component_values(
        world: &mut World,
        expected_wait: Wait,
        expected_sprite_number: usize,
        expected_body: Body,
        expected_interactions: Interactions,
    ) {
        let (frame_component_storages, sequence_statuses) =
            world.system_data::<(FrameComponentStorages, ReadStorage<'_, CharacterSequenceId>)>();
        let body_assets = world.read_resource::<AssetStorage<Body>>();
        let interactions_assets = world.read_resource::<AssetStorage<Interactions>>();

        let FrameComponentStorages {
            waits,
            sprite_renders,
            bodies,
            interactionses,
        } = frame_component_storages;

        (
            &waits,
            &sprite_renders,
            &bodies,
            &interactionses,
            &sequence_statuses,
        )
            .join()
            .for_each(
                |(wait, sprite_render, body_handle, interactions_handle, _sequence_status)| {
                    let body = body_assets
                        .get(body_handle)
                        .expect("Expected `Body` to be loaded.");
                    let interactions = interactions_assets
                        .get(interactions_handle)
                        .expect("Expected `Interactions` to be loaded.");

                    assert_eq!(&expected_wait, wait);
                    assert_eq!(expected_sprite_number, sprite_render.sprite_number);
                    assert_eq!(&expected_body, body);
                    assert_eq!(&expected_interactions, interactions);
                },
            );
    }

    fn body() -> Body {
        Body::new(vec![Volume::Box {
            x: 81,
            y: 0,
            z: 0,
            w: 14,
            h: 12,
            d: 26,
        }])
    }

    fn interactions() -> Interactions {
        Interactions::new(vec![Interaction {
            kind: InteractionKind::Hit(Hit {
                repeat_delay: HitRepeatDelay::default(),
                hp_damage: 20,
                sp_damage: 0,
            }),
            bounds: vec![Volume::Box {
                x: 81,
                y: 0,
                z: 0,
                w: 14,
                h: 12,
                d: 26,
            }],
            multiple: false,
        }])
    }

    fn send_events(world: &mut World, events: Vec<SequenceUpdateEvent>) {
        let mut ec = world.write_resource::<EventChannel<SequenceUpdateEvent>>();
        ec.iter_write(events.into_iter())
    }

    fn sequence_begin_events(world: &mut World) -> Vec<SequenceUpdateEvent> {
        let (
            entities,
            frame_index_clocks,
            frame_wait_clocks,
            component_sequences_handles,
            _frame_component_storages,
        ) = world.system_data::<TestSystemData>();

        (
            &entities,
            &frame_index_clocks,
            &frame_wait_clocks,
            &component_sequences_handles,
        )
            .join()
            .map(|(entity, _, _, _)| SequenceUpdateEvent::SequenceBegin { entity })
            .collect::<Vec<_>>()
    }

    fn frame_begin_events(world: &mut World) -> Vec<SequenceUpdateEvent> {
        let (
            entities,
            frame_index_clocks,
            frame_wait_clocks,
            component_sequences_handles,
            _frame_component_storages,
        ) = world.system_data::<TestSystemData>();

        (
            &entities,
            &frame_index_clocks,
            &frame_wait_clocks,
            &component_sequences_handles,
        )
            .join()
            .map(|(entity, _, _, _)| SequenceUpdateEvent::FrameBegin { entity })
            .collect::<Vec<_>>()
    }

    type TestSystemData<'s> = (
        Entities<'s>,
        WriteStorage<'s, FrameIndexClock>,
        WriteStorage<'s, FrameWaitClock>,
        WriteStorage<'s, ComponentSequencesHandle>,
        FrameComponentStorages<'s>,
    );
}
