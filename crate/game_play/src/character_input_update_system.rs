use amethyst::ecs::prelude::*;
use amethyst::input::InputHandler;
use game_input::{Axis, PlayerActionControl, PlayerAxisControl};
use object_model::loaded::CharacterHandle;

/// Updates `Character` sequence based on input
#[derive(Debug, Default, new)]
pub(crate) struct CharacterInputUpdateSystem;

type CharacterInputUpdateSystemData<'s, 'c> = (
    ReadStorage<'s, CharacterHandle>,
    Read<'s, InputHandler<PlayerAxisControl, PlayerActionControl>>,
);

impl<'s> System<'s> for CharacterInputUpdateSystem {
    type SystemData = CharacterInputUpdateSystemData<'s, 's>;

    fn run(&mut self, (_characters, control_input): Self::SystemData) {
        // TODO: Somehow need to add a component to the entity based on the CharacterSelection.
        //       See `CharacterSelectionSystem`.

        // TODO: Update character active sequence if there is input and a transition.
        let player = 0;
        let x_axis_value = control_input.axis_value(&PlayerAxisControl::new(player, Axis::X));
        let z_axis_value = control_input.axis_value(&PlayerAxisControl::new(player, Axis::Z));
        if x_axis_value != Some(0.) || z_axis_value != Some(0.) {
            // TODO: implement
        }
    }

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
    }
}
