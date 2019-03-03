use amethyst::{
    assets::AssetStorage,
    ecs::{Read, ReadStorage, System, SystemData, WriteStorage},
    shred::Resources,
    shrev::{EventChannel, ReaderId},
};
use derivative::Derivative;
use derive_new::new;
use logic_clock::LogicClock;
use named_type::NamedType;
use named_type_derive::NamedType;
use object_loading::ObjectFrameComponentStorages;
use object_model::{
    entity::FrameIndexClock,
    loaded::{ComponentSequence, ComponentSequences, ComponentSequencesHandle},
};
use shred_derive::SystemData;

use crate::ObjectSequenceUpdateEvent;

/// Updates the logic clock and sequence ID for objects.
#[derive(Debug, Default, NamedType, new)]
pub struct ObjectFrameComponentUpdateSystem {
    /// Reader ID for the `ObjectSequenceUpdateEvent` event channel.
    #[new(default)]
    reader_id: Option<ReaderId<ObjectSequenceUpdateEvent>>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct ObjectFrameComponentUpdateSystemData<'s> {
    /// Event channel for `ObjectSequenceUpdateEvent`s.
    #[derivative(Debug = "ignore")]
    pub object_sequence_update_ec: Read<'s, EventChannel<ObjectSequenceUpdateEvent>>,
    /// `ComponentSequencesHandle` component storage.
    #[derivative(Debug = "ignore")]
    pub component_sequences_handles: ReadStorage<'s, ComponentSequencesHandle>,
    /// `ComponentSequences` assets.
    #[derivative(Debug = "ignore")]
    pub component_sequences_assets: Read<'s, AssetStorage<ComponentSequences>>,
    /// `FrameIndexClock` component storage.
    #[derivative(Debug = "ignore")]
    pub frame_index_clocks: ReadStorage<'s, FrameIndexClock>,
    /// `LogicClock` component storage.
    #[derivative(Debug = "ignore")]
    pub logic_clocks: WriteStorage<'s, LogicClock>,
    /// Game object `Component` storages.
    pub object_frame_component_storages: ObjectFrameComponentStorages<'s>,
}

impl<'s> System<'s> for ObjectFrameComponentUpdateSystem {
    type SystemData = ObjectFrameComponentUpdateSystemData<'s>;

    fn run(
        &mut self,
        ObjectFrameComponentUpdateSystemData {
            object_sequence_update_ec,
            component_sequences_handles,
            component_sequences_assets,
            frame_index_clocks,
            mut logic_clocks,
            object_frame_component_storages,
        }: Self::SystemData,
    ) {
        let ObjectFrameComponentStorages {
            mut waits,
            mut sprite_renders,
            mut bodies,
            mut interactionses,
        } = object_frame_component_storages;

        object_sequence_update_ec
            .read(
                self.reader_id
                    .as_mut()
                    .expect("Expected reader ID to exist for ObjectFrameComponentUpdateSystem."),
            )
            .for_each(|ev| match ev {
                ObjectSequenceUpdateEvent::SequenceBegin { entity }
                | ObjectSequenceUpdateEvent::FrameBegin { entity } => {
                    let component_sequences_handle = component_sequences_handles
                        .get(*entity)
                        .expect("Expected entity to have a `ComponentSequencesHandle` component.");
                    let frame_index_clock = frame_index_clocks
                        .get(*entity)
                        .expect("Expected entity to have a `FrameIndexClock` component.");
                    let logic_clock = logic_clocks
                        .get_mut(*entity)
                        .expect("Expected entity to have a `LogicClock` component.");

                    let component_sequences = component_sequences_assets
                        .get(component_sequences_handle)
                        .expect("Expected component_sequences to be loaded.");

                    let frame_index = (*frame_index_clock).value;

                    component_sequences.iter().for_each(|component_sequence| {
                        match component_sequence {
                            ComponentSequence::Wait(wait_sequence) => {
                                let wait = wait_sequence[frame_index];
                                waits
                                    .insert(*entity, wait)
                                    .expect("Failed to insert `Wait` component for object.");

                                logic_clock.limit = *wait as usize;
                            }
                            ComponentSequence::SpriteRender(sprite_render_sequence) => {
                                let sprite_render = sprite_render_sequence[frame_index].clone();
                                sprite_renders.insert(*entity, sprite_render).expect(
                                    "Failed to insert `SpriteRender` component for object.",
                                );
                            }
                            ComponentSequence::Body(body_sequence) => {
                                let body = body_sequence[frame_index].clone();
                                bodies
                                    .insert(*entity, body)
                                    .expect("Failed to insert `Body` component for object.");
                            }
                            ComponentSequence::Interactions(interactions_sequence) => {
                                let interactions = interactions_sequence[frame_index].clone();
                                interactionses.insert(*entity, interactions).expect(
                                    "Failed to insert `Interactions` component for object.",
                                );
                            }
                        }
                    });
                }
            });
    }

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        self.reader_id = Some(
            res.fetch_mut::<EventChannel<ObjectSequenceUpdateEvent>>()
                .register_reader(),
        );
    }
}

#[cfg(test)]
mod tests {
    use amethyst::{
        assets::{AssetStorage, Prefab},
        ecs::{Entities, Join, ReadStorage, World, WriteStorage},
        shrev::EventChannel,
        Error,
    };
    use application_test_support::AutexousiousApplication;
    use asset_model::loaded::SlugAndHandle;
    use assets_test::ASSETS_CHAR_BAT_SLUG;
    use character_loading::CharacterPrefab;
    use character_model::{config::CharacterSequenceId, loaded::CharacterObjectWrapper};
    use collision_model::config::{Body, Interaction, Interactions};
    use logic_clock::LogicClock;
    use object_loading::{ObjectFrameComponentStorages, ObjectPrefab};
    use object_model::{
        config::object::Wait,
        entity::FrameIndexClock,
        loaded::{ComponentSequencesHandle, ObjectWrapper},
    };
    use shape_model::Volume;

    use super::ObjectFrameComponentUpdateSystem;
    use crate::ObjectSequenceUpdateEvent;

    #[test]
    fn updates_all_frame_components_on_sequence_begin_event() -> Result<(), Error> {
        let test_name = "updates_all_frame_components_on_sequence_begin_event";
        AutexousiousApplication::game_base(test_name, false)
            .with_system(ObjectFrameComponentUpdateSystem::new(), "", &[])
            .with_setup(|world| {
                let component_sequences_handle =
                    component_sequences_handle(world, CharacterSequenceId::StandAttack);
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
            .with_system(ObjectFrameComponentUpdateSystem::new(), "", &[])
            .with_setup(|world| {
                let component_sequences_handle =
                    component_sequences_handle(world, CharacterSequenceId::StandAttack);
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
        logic_clock_value: usize,
        logic_clock_limit: usize,
        component_sequences_handle_initial: ComponentSequencesHandle,
    ) {
        let (
            _entities,
            mut frame_index_clocks,
            mut logic_clocks,
            mut component_sequences_handles,
            ..
        ) = world.system_data::<TestSystemData>();

        (
            &mut frame_index_clocks,
            &mut logic_clocks,
            &mut component_sequences_handles,
        )
            .join()
            .for_each(
                |(frame_index_clock, logic_clock, component_sequences_handle)| {
                    (*frame_index_clock).value = frame_index_clock_value;
                    (*frame_index_clock).limit = frame_index_clock_limit;

                    (*logic_clock).value = logic_clock_value;
                    (*logic_clock).limit = logic_clock_limit;

                    *component_sequences_handle = component_sequences_handle_initial.clone();
                },
            );
    }

    // Quite unergonomic =/
    fn component_sequences_handle(
        world: &mut World,
        sequence_id: CharacterSequenceId,
    ) -> ComponentSequencesHandle {
        let snh = SlugAndHandle::from((&*world, ASSETS_CHAR_BAT_SLUG.clone()));
        let character_prefab_assets =
            world.read_resource::<AssetStorage<Prefab<CharacterPrefab>>>();
        let bat_char_prefab = character_prefab_assets
            .get(&snh.handle)
            .expect("Expected bat character prefab to be loaded.");
        let object_wrapper_handle = {
            let object_prefab = &bat_char_prefab
                .entities()
                .next()
                .expect("Expected bat character main entity to exist.")
                .data()
                .expect("Expected bat character prefab to contain data.")
                .object_prefab;
            if let ObjectPrefab::Handle(handle) = object_prefab {
                handle.clone()
            } else {
                panic!("Expected bat object prefab to be loaded.")
            }
        };

        let object_wrapper_assets = world.read_resource::<AssetStorage<CharacterObjectWrapper>>();
        let object_wrapper = object_wrapper_assets
            .get(&object_wrapper_handle)
            .expect("Expected bat object wrapper to be loaded.");
        object_wrapper
            .inner()
            .component_sequences_handles
            .get(&sequence_id)
            .expect("Expected `RunStop` sequence to exist.")
            .clone()
    }

    fn expect_values(world: &mut World, logic_clock_value: usize, logic_clock_limit: usize) {
        let (logic_clocks, sequence_statuses) =
            world.system_data::<(WriteStorage<LogicClock>, ReadStorage<CharacterSequenceId>)>();

        (&logic_clocks, &sequence_statuses)
            .join()
            .for_each(|(logic_clock, _sequence_status)| {
                assert_eq!(logic_clock_value, (*logic_clock).value);
                assert_eq!(logic_clock_limit, (*logic_clock).limit);
            });
    }

    fn expect_component_values(
        world: &mut World,
        expected_wait: Wait,
        expected_sprite_number: usize,
        expected_body: Body,
        expected_interactions: Interactions,
    ) {
        let (object_frame_component_storages, sequence_statuses) = world.system_data::<(
            ObjectFrameComponentStorages,
            ReadStorage<'_, CharacterSequenceId>,
        )>();
        let body_assets = world.read_resource::<AssetStorage<Body>>();
        let interactions_assets = world.read_resource::<AssetStorage<Interactions>>();

        let ObjectFrameComponentStorages {
            waits,
            sprite_renders,
            bodies,
            interactionses,
        } = object_frame_component_storages;

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
        Interactions::new(vec![Interaction::Physical {
            bounds: vec![Volume::Box {
                x: 81,
                y: 0,
                z: 0,
                w: 14,
                h: 12,
                d: 26,
            }],
            hp_damage: 20,
            sp_damage: 0,
            multiple: false,
        }])
    }

    fn send_events(world: &mut World, events: Vec<ObjectSequenceUpdateEvent>) {
        let mut ec = world.write_resource::<EventChannel<ObjectSequenceUpdateEvent>>();
        ec.iter_write(events.into_iter())
    }

    fn sequence_begin_events(world: &mut World) -> Vec<ObjectSequenceUpdateEvent> {
        let (
            entities,
            frame_index_clocks,
            logic_clocks,
            component_sequences_handles,
            _object_frame_component_storages,
        ) = world.system_data::<TestSystemData>();

        (
            &entities,
            &frame_index_clocks,
            &logic_clocks,
            &component_sequences_handles,
        )
            .join()
            .map(|(entity, _, _, _)| ObjectSequenceUpdateEvent::SequenceBegin { entity })
            .collect::<Vec<_>>()
    }

    fn frame_begin_events(world: &mut World) -> Vec<ObjectSequenceUpdateEvent> {
        let (
            entities,
            frame_index_clocks,
            logic_clocks,
            component_sequences_handles,
            _object_frame_component_storages,
        ) = world.system_data::<TestSystemData>();

        (
            &entities,
            &frame_index_clocks,
            &logic_clocks,
            &component_sequences_handles,
        )
            .join()
            .map(|(entity, _, _, _)| ObjectSequenceUpdateEvent::FrameBegin { entity })
            .collect::<Vec<_>>()
    }

    type TestSystemData<'s> = (
        Entities<'s>,
        WriteStorage<'s, FrameIndexClock>,
        WriteStorage<'s, LogicClock>,
        WriteStorage<'s, ComponentSequencesHandle>,
        ObjectFrameComponentStorages<'s>,
    );
}
