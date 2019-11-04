use amethyst::{
    ecs::{Join, Read, ReadStorage, System, World, WriteStorage},
    shred::{ResourceId, SystemData},
};
use derivative::Derivative;
use derive_new::new;
use kinematic_model::config::Position;
use map_model::loaded::AssetMargins;
use map_selection_model::MapSelection;
use object_model::play::Grounding;
use typename_derive::TypeName;

/// Updates `Grounding` to `Airborne` for objects above the map bottom boundary.
#[derive(Debug, Default, TypeName, new)]
pub struct ObjectGroundingSystem;

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct ObjectGroundingSystemData<'s> {
    /// `MapSelection` resource.
    #[derivative(Debug = "ignore")]
    pub map_selection: Read<'s, MapSelection>,
    /// `AssetMargins` resource.
    #[derivative(Debug = "ignore")]
    pub asset_margins: Read<'s, AssetMargins>,
    /// `Position<f32>` components.
    #[derivative(Debug = "ignore")]
    pub positions: ReadStorage<'s, Position<f32>>,
    /// `Grounding` components.
    #[derivative(Debug = "ignore")]
    pub groundings: WriteStorage<'s, Grounding>,
}

impl<'s> System<'s> for ObjectGroundingSystem {
    type SystemData = ObjectGroundingSystemData<'s>;

    fn run(
        &mut self,
        ObjectGroundingSystemData {
            map_selection,
            asset_margins,
            positions,
            mut groundings,
        }: Self::SystemData,
    ) {
        let map_margins = asset_margins
            .get(
                map_selection
                    .asset_id()
                    .expect("Expected `MapSelection` asset ID to exist."),
            )
            .expect("Expected `Margins` to be loaded.");

        (&positions, &mut groundings)
            .join()
            .for_each(|(position, grounding)| {
                if position[1] > map_margins.bottom {
                    *grounding = Grounding::Airborne;
                } else if position[1] < map_margins.bottom {
                    *grounding = Grounding::Underground;
                } else {
                    *grounding = Grounding::OnGround;
                }
            });
    }
}
