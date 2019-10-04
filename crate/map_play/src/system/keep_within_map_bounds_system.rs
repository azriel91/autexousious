use amethyst::{
    ecs::{Read, ReadExpect, ReadStorage, System, World, WriteStorage},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use derivative::Derivative;
use derive_new::new;
use kinematic_model::config::Position;
use map_model::{
    loaded::AssetMargins,
    play::{BoundaryFace, MapBoundaryEvent, MapBoundaryEventData, MapBounded},
};
use map_selection_model::MapSelection;
use typename_derive::TypeName;

/// Keeps entities within map bounds.
#[derive(Debug, Default, TypeName, new)]
pub struct KeepWithinMapBoundsSystem {
    /// Reader ID for the `MapBoundaryEvent` channel.
    #[new(default)]
    map_boundary_event_rid: Option<ReaderId<MapBoundaryEvent>>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct KeepWithinMapBoundsSystemData<'s> {
    /// `MapBoundaryEvent` channel.
    #[derivative(Debug = "ignore")]
    pub map_boundary_ec: Read<'s, EventChannel<MapBoundaryEvent>>,
    /// `MapSelection` resource.
    #[derivative(Debug = "ignore")]
    pub map_selection: ReadExpect<'s, MapSelection>,
    /// `AssetMargins` resource.
    #[derivative(Debug = "ignore")]
    pub asset_margins: Read<'s, AssetMargins>,
    /// `MapBounded` components.
    #[derivative(Debug = "ignore")]
    pub map_boundeds: ReadStorage<'s, MapBounded>,
    /// `Position<f32>` components.
    #[derivative(Debug = "ignore")]
    pub positions: WriteStorage<'s, Position<f32>>,
}

impl<'s> System<'s> for KeepWithinMapBoundsSystem {
    type SystemData = KeepWithinMapBoundsSystemData<'s>;

    fn run(
        &mut self,
        KeepWithinMapBoundsSystemData {
            map_boundary_ec,
            map_selection,
            asset_margins,
            map_boundeds,
            mut positions,
        }: Self::SystemData,
    ) {
        let map_margins = asset_margins
            .get(
                map_selection
                    .asset_id()
                    .expect("Expected `MapSelection` asset ID to exist."),
            )
            .expect("Expected `Margins` to be loaded.");

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
                let entity = *entity;
                if let (Some(_), Some(position)) =
                    (map_boundeds.get(entity), positions.get_mut(entity))
                {
                    if boundary_faces.contains(BoundaryFace::Left) {
                        position[0] = map_margins.left;
                    } else if boundary_faces.contains(BoundaryFace::Right) {
                        position[0] = map_margins.right;
                    }

                    if boundary_faces.contains(BoundaryFace::Bottom) {
                        position[1] = map_margins.bottom;
                    } else if boundary_faces.contains(BoundaryFace::Top) {
                        position[1] = map_margins.top;
                    }

                    if boundary_faces.contains(BoundaryFace::Back) {
                        position[2] = map_margins.back;
                    } else if boundary_faces.contains(BoundaryFace::Front) {
                        position[2] = map_margins.front;
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
