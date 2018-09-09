use amethyst::{
    ecs::prelude::*,
    shrev::{EventChannel, ReaderId},
};
use game_model::loaded::CharacterAssets;

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
    Read<'s, CharacterAssets>,
    Write<'s, CharacterSelections>,
);

impl<'s> System<'s> for CharacterSelectionSystem {
    type SystemData = CharacterSelectionSystemData<'s>;

    fn run(
        &mut self,
        (character_selection_events, character_assets, mut character_selections): Self::SystemData,
    ) {
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
                    let character_slug = match character_selection {
                        // kcov-ignore-end
                        CharacterSelection::Random => {
                            // TODO: implement random
                            character_assets // kcov-ignore
                                .keys()
                                .next()
                                .expect("Expected at least one character to be loaded.")
                        }
                        CharacterSelection::Id(slug) => slug,
                    };
                    character_selections
                        .selections
                        .entry(*controller_id)
                        .or_insert(character_slug.clone());
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
    use asset_loading::ASSETS_TEST_DIR;
    use game_input::{PlayerActionControl, PlayerAxisControl};
    use game_model::config::{AssetSlug, AssetSlugBuilder};
    use typename::TypeName;

    use super::CharacterSelectionSystem;
    use CharacterSelection;
    use CharacterSelectionEvent;
    use CharacterSelections;
    use CharacterSelectionsState;

    const ASSETS_CHAR_BAT_NAME: &str = "bat";

    lazy_static! {
        /// Slug of the "bat" character asset.
        static ref ASSETS_CHAR_BAT_SLUG: AssetSlug = {
            AssetSlugBuilder::default()
                .namespace(ASSETS_TEST_DIR.to_string())
                .name(ASSETS_CHAR_BAT_NAME.to_string())
                .build()
                .expect(&format!(
                    "Expected `{}/{}` asset slug to build.",
                    ASSETS_TEST_DIR,
                    ASSETS_CHAR_BAT_NAME
                ))
        };
    }

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
                        character_selection: CharacterSelection::Id(ASSETS_CHAR_BAT_SLUG.clone())
                    }
                )).with_assertion(|world| {
                    let character_selections = world.read_resource::<CharacterSelections>();

                    assert_eq!(
                        Some(&*ASSETS_CHAR_BAT_SLUG),
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
                        .insert(123, ASSETS_CHAR_BAT_SLUG.clone()); // kcov-ignore
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
