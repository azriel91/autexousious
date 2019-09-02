use amethyst::{
    ecs::{Read, ReadStorage, System, World, WriteStorage},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use derivative::Derivative;
use derive_new::new;
use map_model::play::{BoundaryFace, MapBoundaryEvent, MapBoundaryEventData, MapBounded};
use object_model::play::Grounding;
use typename_derive::TypeName;

/// Updates the `Grounding` for objects that are `MapBounded` and exit the map bottom boundary.
#[derive(Debug, Default, TypeName, new)]
pub struct ObjectGroundingSystem {
    /// Reader ID for the `MapBoundaryEvent` channel.
    #[new(default)]
    map_boundary_event_rid: Option<ReaderId<MapBoundaryEvent>>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct ObjectGroundingSystemData<'s> {
    /// `MapBoundaryEvent` channel.
    #[derivative(Debug = "ignore")]
    pub map_boundary_ec: Read<'s, EventChannel<MapBoundaryEvent>>,
    /// `MapBounded` components.
    #[derivative(Debug = "ignore")]
    pub map_boundeds: ReadStorage<'s, MapBounded>,
    /// `Grounding` components.
    #[derivative(Debug = "ignore")]
    pub groundings: WriteStorage<'s, Grounding>,
}

impl<'s> System<'s> for ObjectGroundingSystem {
    type SystemData = ObjectGroundingSystemData<'s>;

    fn run(
        &mut self,
        ObjectGroundingSystemData {
            map_boundary_ec,
            map_boundeds,
            mut groundings,
        }: Self::SystemData,
    ) {
        let map_boundary_event_rid = self
            .map_boundary_event_rid
            .as_mut()
            .expect("Expected `map_boundary_event_rid` field to be set.");

        map_boundary_ec.read(map_boundary_event_rid).for_each(|ev| {
            if let MapBoundaryEvent::Exit(MapBoundaryEventData {
                entity,
                boundary_faces,
            }) = ev
            {
                if boundary_faces.contains(BoundaryFace::Bottom) {
                    let entity = *entity;
                    if let (Some(_), Some(grounding)) =
                        (map_boundeds.get(entity), groundings.get_mut(entity))
                    {
                        *grounding = Grounding::OnGround;
                    }
                }
            }
        });
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        self.map_boundary_event_rid = Some(
            world
                .fetch_mut::<EventChannel<MapBoundaryEvent>>()
                .register_reader(),
        );
    }
}

#[cfg(test)]
mod tests {
    use amethyst::{
        ecs::{Builder, Entity, System, SystemData, World, WorldExt},
        shrev::EventChannel,
        Error,
    };
    use amethyst_test::AmethystApplication;
    use enumflags2::BitFlags;
    use map_model::play::{BoundaryFace, MapBoundaryEvent, MapBoundaryEventData, MapBounded};
    use object_model::play::Grounding;
    use typename::TypeName;

    use super::ObjectGroundingSystem;

    #[test]
    fn does_not_change_grounding_when_no_map_boundary_event() -> Result<(), Error> {
        run_test(
            SetupParams {
                grounding: Grounding::Airborne,
                map_boundary_event_fn: None,
            },
            ExpectedParams {
                grounding: Grounding::Airborne,
            },
        )
    }

    #[test]
    fn does_not_change_grounding_on_enter_event() -> Result<(), Error> {
        run_test(
            SetupParams {
                grounding: Grounding::Airborne,
                map_boundary_event_fn: Some(|entity| {
                    let boundary_faces =
                        BoundaryFace::Left | BoundaryFace::Bottom | BoundaryFace::Back;
                    MapBoundaryEvent::Enter(MapBoundaryEventData {
                        entity,
                        boundary_faces,
                    })
                }),
            },
            ExpectedParams {
                grounding: Grounding::Airborne,
            },
        )
    }

    #[test]
    fn sets_grounding_to_on_ground_on_exit_event_bottom() -> Result<(), Error> {
        run_test(
            SetupParams {
                grounding: Grounding::Airborne,
                map_boundary_event_fn: Some(|entity| {
                    let boundary_faces = BitFlags::from(BoundaryFace::Bottom);
                    MapBoundaryEvent::Exit(MapBoundaryEventData {
                        entity,
                        boundary_faces,
                    })
                }),
            },
            ExpectedParams {
                grounding: Grounding::OnGround,
            },
        )
    }

    fn run_test(
        SetupParams {
            grounding,
            map_boundary_event_fn,
        }: SetupParams,
        ExpectedParams {
            grounding: grounding_expected,
        }: ExpectedParams,
    ) -> Result<(), Error> {
        AmethystApplication::blank()
            .with_system(
                ObjectGroundingSystem::new(),
                ObjectGroundingSystem::type_name(),
                &[],
            ) // kcov-ignore
            .with_setup(setup_system_data)
            .with_effect(move |world| {
                let entity = world
                    .create_entity()
                    .with(grounding)
                    .with(MapBounded)
                    .build();

                if let Some(map_boundary_event_fn) = map_boundary_event_fn {
                    let map_boundary_event = map_boundary_event_fn(entity);
                    let mut map_boundary_ec =
                        world.write_resource::<EventChannel<MapBoundaryEvent>>();

                    map_boundary_ec.single_write(map_boundary_event);
                }

                world.insert(entity);
            })
            .with_assertion(move |world| {
                let entity = *world.read_resource::<Entity>();
                let groundings = world.read_storage::<Grounding>();
                let grounding_actual = groundings
                    .get(entity)
                    .copied()
                    .expect("Expected entity to have `Grounding` component.");

                assert_eq!(grounding_expected, grounding_actual);
            })
            .run()
    }

    fn setup_system_data(world: &mut World) {
        <ObjectGroundingSystem as System<'_>>::SystemData::setup(world);
    }

    struct SetupParams {
        grounding: Grounding,
        map_boundary_event_fn: Option<fn(Entity) -> MapBoundaryEvent>,
    }

    struct ExpectedParams {
        grounding: Grounding,
    }
}
