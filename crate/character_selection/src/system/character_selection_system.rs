use amethyst::{
    ecs::prelude::*,
    shrev::{EventChannel, ReaderId},
};

use CharacterSelection;
use CharacterSelectionEvent;
use CharacterSelections;

/// Populates the `CharacterSelections` based on user input.
#[derive(Debug, Default, TypeName, new)]
pub struct CharacterSelectionSystem {
    /// Reader ID for the `CharacterSelectionEvent` event channel.
    #[new(default)]
    reader_id: Option<ReaderId<CharacterSelectionEvent>>,
}

type CharacterSelectionSystemData<'s> = (
    Read<'s, EventChannel<CharacterSelectionEvent>>,
    Write<'s, CharacterSelections>,
);

impl<'s> System<'s> for CharacterSelectionSystem {
    type SystemData = CharacterSelectionSystemData<'s>;

    fn run(&mut self, (character_selection_events, mut character_selections): Self::SystemData) {
        character_selection_events
            .read(
                self.reader_id
                    .as_mut()
                    .expect("Expected to read `CharacterSelectionEvent`s."),
            ).for_each(|ev| match ev {
                CharacterSelectionEvent::Select {
                    controller_id,
                    character_selection,
                } => {
                    let character_id = match character_selection {
                        CharacterSelection::Random => 0, // TODO: implement random
                        CharacterSelection::Id(id) => *id,
                    };
                    character_selections
                        .entry(*controller_id)
                        .or_insert(character_id);
                }
                CharacterSelectionEvent::Deselect { controller_id } => {
                    character_selections.remove(&controller_id);
                }
                CharacterSelectionEvent::Confirmed => {
                    // TODO: Change CharacterSelections into a richer struct.
                }
            });
    }

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        self.reader_id = Some(
            res.fetch_mut::<EventChannel<CharacterSelectionEvent>>()
                .register_reader(),
        );
    }
}
