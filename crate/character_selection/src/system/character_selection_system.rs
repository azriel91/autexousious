use amethyst::{
    ecs::prelude::*,
    shrev::{EventChannel, ReaderId},
};

use CharacterSelection;
use CharacterSelectionEvent;
use CharacterSelections;
use CharacterSelectionsState;

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
                    // kcov-ignore-start
                    let character_id = match character_selection {
                        // kcov-ignore-end
                        CharacterSelection::Random => 0, // TODO: implement random
                        CharacterSelection::Id(id) => *id,
                    };
                    character_selections
                        .selections
                        .entry(*controller_id)
                        .or_insert(character_id);
                }
                CharacterSelectionEvent::Deselect { controller_id } => {
                    character_selections.selections.remove(&controller_id);
                }
                CharacterSelectionEvent::Confirm => {
                    character_selections.state = CharacterSelectionsState::Ready;
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

#[cfg(test)]
mod tests {
    use amethyst::{ecs::prelude::*, shrev::EventChannel};
    use amethyst_test_support::prelude::*;
    use game_input::{PlayerActionControl, PlayerAxisControl};
    use typename::TypeName;

    use super::CharacterSelectionSystem;
    use CharacterSelection;
    use CharacterSelectionEvent;
    use CharacterSelections;
    use CharacterSelectionsState;

    #[test]
    fn inserts_character_selection_on_select_event() {
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::ui_base::<PlayerAxisControl, PlayerActionControl>()
                .with_system(
                    CharacterSelectionSystem::new(),
                    CharacterSelectionSystem::type_name(),
                    &[]
                ).with_setup(|world| send_event(
                    world,
                    CharacterSelectionEvent::Select {
                        controller_id: 123,
                        character_selection: CharacterSelection::Id(321)
                    }
                )).with_assertion(|world| {
                    let character_selections = world.read_resource::<CharacterSelections>();

                    assert_eq!(
                        Some(&(321 as usize)),
                        character_selections.selections.get(&123)
                    );
                }).run()
                .is_ok()
        );
    }

    #[test]
    fn removes_character_selection_on_deselect_event() {
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::ui_base::<PlayerAxisControl, PlayerActionControl>()
                .with_system(
                    CharacterSelectionSystem::new(),
                    CharacterSelectionSystem::type_name(),
                    &[]
                ).with_setup(|world| {
                    world
                        .write_resource::<CharacterSelections>()
                        .selections
                        .insert(123, 321); // kcov-ignore
                }).with_setup(|world| send_event(
                    world,
                    CharacterSelectionEvent::Deselect { controller_id: 123 }
                )).with_assertion(|world| {
                    let character_selections = world.read_resource::<CharacterSelections>();

                    assert_eq!(None, character_selections.selections.get(&123));
                }).run()
                .is_ok()
        );
    }

    #[test]
    fn sets_character_selections_state_to_ready_on_confirm_event() {
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::ui_base::<PlayerAxisControl, PlayerActionControl>()
                .with_system(
                    CharacterSelectionSystem::new(),
                    CharacterSelectionSystem::type_name(),
                    &[]
                ).with_setup(|world| send_event(world, CharacterSelectionEvent::Confirm))
                .with_assertion(|world| {
                    let character_selections = world.read_resource::<CharacterSelections>();

                    assert_eq!(CharacterSelectionsState::Ready, character_selections.state);
                }).run()
                .is_ok()
        );
    }

    fn send_event(world: &mut World, event: CharacterSelectionEvent) {
        world
            .write_resource::<EventChannel<CharacterSelectionEvent>>()
            .single_write(event);
    }
}
