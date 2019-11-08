use amethyst::{
    ecs::{Entities, Entity, Join, Read, ReadStorage, System, World, Write},
    shred::{ResourceId, SystemData},
    shrev::EventChannel,
};
use derivative::Derivative;
use derive_new::new;
use enumflags2::BitFlags;
use kinematic_model::config::Position;
use map_model::{
    loaded::{AssetMargins, Margins},
    play::{BoundaryFace, MapBoundaryEvent, MapBoundaryEventData},
};
use map_selection_model::MapSelection;
use tracker::Last;
use typename_derive::TypeName;

use crate::{Comparative, MapBoundsChecks};

/// Sends a `MapBoundaryEvent` when an entity's `Position` has entered or exited map bounds.
#[derive(Debug, Default, TypeName, new)]
pub struct MapEnterExitDetectionSystem;

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct MapEnterExitDetectionSystemData<'s> {
    /// `Entities`.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// `MapSelection` resource.
    #[derivative(Debug = "ignore")]
    pub map_selection: Read<'s, MapSelection>,
    /// `AssetMargins` resource.
    #[derivative(Debug = "ignore")]
    pub asset_margins: Read<'s, AssetMargins>,
    /// `Last<Position<f32>>` components.
    #[derivative(Debug = "ignore")]
    pub positions_last: ReadStorage<'s, Last<Position<f32>>>,
    /// `Position<f32>` components.
    #[derivative(Debug = "ignore")]
    pub positions: ReadStorage<'s, Position<f32>>,
    /// `MapBoundaryEvent` channel.
    #[derivative(Debug = "ignore")]
    pub map_boundary_ec: Write<'s, EventChannel<MapBoundaryEvent>>,
}

impl<'s> System<'s> for MapEnterExitDetectionSystem {
    type SystemData = MapEnterExitDetectionSystemData<'s>;

    fn run(
        &mut self,
        MapEnterExitDetectionSystemData {
            entities,
            map_selection,
            asset_margins,
            positions_last,
            positions,
            mut map_boundary_ec,
        }: Self::SystemData,
    ) {
        let map_margins = asset_margins
            .get(
                map_selection
                    .asset_id()
                    .expect("Expected `MapSelection` asset ID to exist."),
            )
            .expect("Expected `Margins` to be loaded.");

        // Send event when the entity was in bounds previously, but not in bounds now.
        let map_boundary_events = (&entities, &positions_last, &positions)
            .join()
            .filter_map(|(entity, position_last, position)| {
                Self::detect_enter_exit(map_margins, entity, **position_last, *position)
            })
            .collect::<Vec<MapBoundaryEvent>>();
        map_boundary_ec.iter_write(map_boundary_events);
    }
}

impl MapEnterExitDetectionSystem {
    /// Returns a `MapBoundaryEvent` if the entity has crossed the map boundary.
    fn detect_enter_exit(
        map_margins: &Margins,
        entity: Entity,
        position_last: Position<f32>,
        position: Position<f32>,
    ) -> Option<MapBoundaryEvent> {
        let (within_x_last, within_y_last, within_z_last) =
            MapBoundsChecks::position_comparative(map_margins, position_last);
        let within_bounds_last =
            MapBoundsChecks::is_within_bounds(within_x_last, within_y_last, within_z_last);

        let (within_x, within_y, within_z) =
            MapBoundsChecks::position_comparative(map_margins, position);
        let within_bounds = MapBoundsChecks::is_within_bounds(within_x, within_y, within_z);

        let mut boundary_faces = BitFlags::<BoundaryFace>::default();

        if within_bounds_last && !within_bounds {
            match within_x {
                Comparative::Below => boundary_faces |= BoundaryFace::Left,
                Comparative::Above => boundary_faces |= BoundaryFace::Right,
                Comparative::Within => {}
            }
            match within_y {
                Comparative::Below => boundary_faces |= BoundaryFace::Bottom,
                Comparative::Above => boundary_faces |= BoundaryFace::Top,
                Comparative::Within => {}
            }
            match within_z {
                Comparative::Below => boundary_faces |= BoundaryFace::Back,
                Comparative::Above => boundary_faces |= BoundaryFace::Front,
                Comparative::Within => {}
            }
            Some(MapBoundaryEvent::Exit(MapBoundaryEventData {
                entity,
                boundary_faces,
            }))
        } else if !within_bounds_last && within_bounds {
            match within_x_last {
                Comparative::Below => boundary_faces |= BoundaryFace::Left,
                Comparative::Above => boundary_faces |= BoundaryFace::Right,
                Comparative::Within => {}
            }
            match within_y_last {
                Comparative::Below => boundary_faces |= BoundaryFace::Bottom,
                Comparative::Above => boundary_faces |= BoundaryFace::Top,
                Comparative::Within => {}
            }
            match within_z_last {
                Comparative::Below => boundary_faces |= BoundaryFace::Back,
                Comparative::Above => boundary_faces |= BoundaryFace::Front,
                Comparative::Within => {}
            }

            Some(MapBoundaryEvent::Enter(MapBoundaryEventData {
                entity,
                boundary_faces,
            }))
        } else {
            None
        }
    }
}
