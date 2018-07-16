use amethyst::{assets::AssetStorage, ecs::prelude::*};
use map_model::loaded::{Map, MapHandle, Margins};
use object_model::entity::{CharacterStatus, Grounding, Kinematics};

/// Updates `Character` kinematics based on sequence.
#[derive(Debug, Default, new)]
pub(crate) struct CharacterGroundingSystem;

type CharacterGroundingSystemData<'s> = (
    Read<'s, AssetStorage<Map>>,
    ReadStorage<'s, MapHandle>, // TODO: Create a `MapSelection` like `CharacterSelection`.
    WriteStorage<'s, Kinematics<f32>>,
    WriteStorage<'s, CharacterStatus>,
);

impl<'s> System<'s> for CharacterGroundingSystem {
    type SystemData = CharacterGroundingSystemData<'s>;

    fn run(
        &mut self,
        (maps, map_handle_storage, mut kinematics_storage, mut status_storage): Self::SystemData,
    ) {
        let map_handle = &map_handle_storage.join().next();
        if map_handle.is_none() {
            // Game is not running.
            // TODO: Use custom `GameData`
            return;
        }

        let map_handle = map_handle.unwrap().clone();
        let map_margins = {
            // TODO: Use custom `GameData`, which allows use to use
            // `.expect("Expected map to be loaded.")`
            maps.get(&map_handle).map_or_else(
                || Margins::new(0., 800., 0., 600., 0., 200.),
                |map| map.margins,
            )
        };

        for (mut kinematics, mut status) in (&mut kinematics_storage, &mut status_storage).join() {
            if kinematics.position[1] > map_margins.bottom {
                kinematics.velocity[1] += -1.7;
                status.object_status.grounding = Grounding::Airborne;
            } else {
                kinematics.position[1] = map_margins.bottom;
                kinematics.velocity[1] = 0.;
                status.object_status.grounding = Grounding::OnGround;
            }
        }
    }
}
