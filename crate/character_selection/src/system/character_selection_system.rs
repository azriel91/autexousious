use amethyst::ecs::prelude::*;
use object_model::loaded;

use CharacterSelection;

/// Populates the `CharacterSelection` based on user input.
#[derive(Debug, Default, TypeName, new)]
pub(crate) struct CharacterSelectionSystem;

type CharacterSelectionSystemData<'s, 'c> = (
    Read<'s, Vec<loaded::Character>>,
    Write<'s, CharacterSelection>,
);

impl<'s> System<'s> for CharacterSelectionSystem {
    type SystemData = CharacterSelectionSystemData<'s, 's>;

    // kcov-ignore-start
    fn run(&mut self, (_characters, mut character_selection): Self::SystemData) {
        // TODO: Update `CharacterSelection` with the user selected `character_object_index`.
        let controller_id = 0;
        let character_object_index = 0; // First loaded `Character`
        character_selection
            .entry(controller_id)
            .or_insert(character_object_index);
    }
    // kcov-ignore-end

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
    }
}
