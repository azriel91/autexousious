use amethyst::{
    ecs::prelude::*,
    shrev::{EventChannel, ReaderId},
};

use CharacterSelection;
use CharacterSelectionEvent;
use CharacterSelections;
use CharacterSelectionsStatus;

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
                    let slug_and_handle = match character_selection {
                        CharacterSelection::Random(slug_and_handle)
                        | CharacterSelection::Id(slug_and_handle) => slug_and_handle,
                    };
                    character_selections
                        .selections
                        .entry(*controller_id)
                        .or_insert_with(|| slug_and_handle.clone());
                }
                CharacterSelectionEvent::Deselect { controller_id } => {
                    character_selections.selections.remove(&controller_id);
                }
                CharacterSelectionEvent::Confirm => {
                    character_selections.state = CharacterSelectionsStatus::Ready;
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
    use std::env;

    use amethyst::{ecs::prelude::*, shrev::EventChannel};
    use amethyst_test_support::prelude::*;
    use application_event::AppEvent;
    use assets_test::{ASSETS_CHAR_BAT_SLUG, ASSETS_PATH};
    use game_input::{PlayerActionControl, PlayerAxisControl};
    use game_model::loaded::SlugAndHandle;
    use loading::LoadingState;
    use map_loading::MapLoadingBundle;
    use object_loading::ObjectLoadingBundle;
    use typename::TypeName;

    use super::CharacterSelectionSystem;
    use CharacterSelection;
    use CharacterSelectionEvent;
    use CharacterSelections;
    use CharacterSelectionsStatus;

    #[test]
    fn inserts_character_selection_on_select_event() {
        env::set_var("APP_DIR", env!("CARGO_MANIFEST_DIR"));

        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::render_base("inserts_character_selection_on_select_event", false)
                .with_custom_event_type::<AppEvent>()
                .with_bundle(MapLoadingBundle::new())
                .with_bundle(ObjectLoadingBundle::new())
                .with_state(|| LoadingState::new(ASSETS_PATH.clone(), EmptyState))
                .with_system(
                    CharacterSelectionSystem::new(),
                    CharacterSelectionSystem::type_name(),
                    &[]
                ).with_setup(|world| {
                    let slug_and_handle =
                        SlugAndHandle::from((&*world, ASSETS_CHAR_BAT_SLUG.clone()));

                    send_event(
                        world,
                        CharacterSelectionEvent::Select {
                            controller_id: 123,
                            character_selection: CharacterSelection::Id(slug_and_handle),
                        },
                    )
                }).with_assertion(|world| {
                    let character_selections = world.read_resource::<CharacterSelections>();

                    assert_eq!(
                        Some(&SlugAndHandle::from((
                            &*world,
                            ASSETS_CHAR_BAT_SLUG.clone()
                        ))),
                        character_selections.selections.get(&123)
                    );
                }).run()
                .is_ok()
        );
    }

    #[test]
    fn removes_character_selection_on_deselect_event() {
        env::set_var("APP_DIR", env!("CARGO_MANIFEST_DIR"));

        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::render_base(
                "removes_character_selection_on_deselect_event",
                false
            ).with_custom_event_type::<AppEvent>()
            .with_bundle(MapLoadingBundle::new())
            .with_bundle(ObjectLoadingBundle::new())
            .with_state(|| LoadingState::new(ASSETS_PATH.clone(), EmptyState))
            .with_system(
                CharacterSelectionSystem::new(),
                CharacterSelectionSystem::type_name(),
                &[]
            ).with_setup(|world| {
                world
                    .write_resource::<CharacterSelections>()
                    .selections
                    .insert(
                        123,
                        SlugAndHandle::from((&*world, ASSETS_CHAR_BAT_SLUG.clone())),
                    );
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
    fn sets_character_selections_status_to_ready_on_confirm_event() {
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

                    assert_eq!(CharacterSelectionsStatus::Ready, character_selections.state);
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
