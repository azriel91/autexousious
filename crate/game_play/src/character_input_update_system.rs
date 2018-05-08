use amethyst::animation::{get_animation_set, AnimationCommand, AnimationControlSet, EndControl};
use amethyst::assets::AssetStorage;
use amethyst::ecs::prelude::*;
use amethyst::input::InputHandler;
use amethyst::renderer::Material;
use character_selection::CharacterEntityControl;
use game_input::{Axis, PlayerActionControl, PlayerAxisControl};
use object_model::loaded::{Character, CharacterHandle};

/// Updates `Character` sequence based on input
#[derive(Debug, Default, new)]
pub(crate) struct CharacterInputUpdateSystem;

type CharacterInputUpdateSystemData<'s, 'c> = (
    Entities<'s>,
    ReadStorage<'s, CharacterHandle>,
    ReadStorage<'s, CharacterEntityControl>,
    Read<'s, AssetStorage<Character>>,
    Read<'s, InputHandler<PlayerAxisControl, PlayerActionControl>>,
    WriteStorage<'s, AnimationControlSet<u32, Material>>,
);

impl<'s> System<'s> for CharacterInputUpdateSystem {
    type SystemData = CharacterInputUpdateSystemData<'s, 's>;

    fn run(
        &mut self,
        (
            entities,
            handle_storage,
            control_storage,
            characters,
            control_input,
            mut animation_control_set_storage,
        ): Self::SystemData,
    ) {
        for (entity, character_handle, character_entity_control) in
            (&*entities, &handle_storage, &control_storage).join()
        {
            let player = character_entity_control.controller_id;

            let character = characters
                .get(character_handle)
                .expect("Expected character to be loaded.");

            let x_axis_value = control_input.axis_value(&PlayerAxisControl::new(player, Axis::X));
            let z_axis_value = control_input.axis_value(&PlayerAxisControl::new(player, Axis::Z));

            // TODO: Use Pushdown automata to handle state
            if x_axis_value != Some(0.) || z_axis_value != Some(0.) {
                let animation_handle = character.object.animations.get(1).unwrap().clone();

                // Start the animation
                let animation_set =
                    get_animation_set::<u32, Material>(&mut animation_control_set_storage, entity);
                animation_set.abort(0);
                let animation_id = 1;
                animation_set.add_animation(
                    animation_id,
                    &animation_handle,
                    EndControl::Loop(None),
                    30., // Rate at which the animation plays
                    AnimationCommand::Start,
                );
            } else if x_axis_value == Some(0.) && z_axis_value == Some(0.) {
                let animation_handle = character.object.animations.get(0).unwrap().clone();

                // Start the animation
                let animation_set =
                    get_animation_set::<u32, Material>(&mut animation_control_set_storage, entity);
                animation_set.abort(1);
                let animation_id = 0;
                animation_set.add_animation(
                    animation_id,
                    &animation_handle,
                    EndControl::Loop(None),
                    30., // Rate at which the animation plays
                    AnimationCommand::Start,
                );
            }
        }
    }

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
    }
}
