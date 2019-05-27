use amethyst::{
    ecs::prelude::*,
    shrev::{EventChannel, ReaderId},
};
use character_selection_model::{CharacterSelection, CharacterSelectionEvent, CharacterSelections};
use derive_new::new;
use typename_derive::TypeName;

/// Populates the `CharacterSelections` based on user input.
#[derive(Debug, Default, TypeName, new)]
pub struct CharacterSelectionSystem {
    /// Reader ID for the `CharacterSelectionEvent` event channel.
    #[new(default)]
    character_selection_event_rid: Option<ReaderId<CharacterSelectionEvent>>,
}

type CharacterSelectionSystemData<'s> = (
    Read<'s, EventChannel<CharacterSelectionEvent>>,
    Write<'s, CharacterSelections>,
);

impl<'s> System<'s> for CharacterSelectionSystem {
    type SystemData = CharacterSelectionSystemData<'s>;

    fn run(&mut self, (character_selection_ec, mut character_selections): Self::SystemData) {
        character_selection_ec
            .read(
                self.character_selection_event_rid
                    .as_mut()
                    .expect("Expected `character_selection_event_rid` to be set."),
            )
            .for_each(|ev| match ev {
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
                _ => {}
            });
    }

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        self.character_selection_event_rid = Some(
            res.fetch_mut::<EventChannel<CharacterSelectionEvent>>()
                .register_reader(),
        );
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use amethyst::{
        assets::Processor, audio::Source, core::TransformBundle, ecs::World,
        renderer::RenderEmptyBundle, shrev::EventChannel, Error,
    };
    use amethyst_test::{AmethystApplication, PopState};
    use application_event::{AppEvent, AppEventReader};
    use asset_model::loaded::SlugAndHandle;
    use assets_test::{ASSETS_CHAR_BAT_SLUG, ASSETS_PATH};
    use character_loading::CharacterLoadingBundle;
    use character_selection_model::{
        CharacterSelection, CharacterSelectionEvent, CharacterSelections,
    };
    use collision_audio_loading::CollisionAudioLoadingBundle;
    use collision_loading::CollisionLoadingBundle;
    use game_input_model::ControlBindings;
    use loading::{LoadingBundle, LoadingState};
    use map_loading::MapLoadingBundle;
    use sequence_loading::SequenceLoadingBundle;
    use sprite_loading::SpriteLoadingBundle;
    use typename::TypeName;
    use ui_audio_loading::UiAudioLoadingBundle;

    use super::CharacterSelectionSystem;

    #[test]
    fn inserts_character_selection_on_select_event() -> Result<(), Error> {
        env::set_var("APP_DIR", env!("CARGO_MANIFEST_DIR"));

        AmethystApplication::blank()
            .with_bundle(TransformBundle::new())
            .with_bundle(RenderEmptyBundle::new())
            .with_custom_event_type::<AppEvent, AppEventReader>()
            .with_ui_bundles::<ControlBindings>()
            .with_system(Processor::<Source>::new(), "source_processor", &[])
            .with_bundle(SpriteLoadingBundle::new())
            .with_bundle(SequenceLoadingBundle::new())
            .with_bundle(LoadingBundle::new(ASSETS_PATH.clone()))
            .with_bundle(CollisionLoadingBundle::new())
            .with_bundle(MapLoadingBundle::new())
            .with_bundle(CharacterLoadingBundle::new())
            .with_bundle(CollisionAudioLoadingBundle::new(ASSETS_PATH.clone()))
            .with_bundle(UiAudioLoadingBundle::new(ASSETS_PATH.clone()))
            .with_state(|| LoadingState::new(PopState))
            .with_system(
                CharacterSelectionSystem::new(),
                CharacterSelectionSystem::type_name(),
                &[],
            ) // kcov-ignore
            .with_setup(|world| {
                let slug_and_handle = SlugAndHandle::from((&*world, ASSETS_CHAR_BAT_SLUG.clone()));

                send_event(
                    world,
                    CharacterSelectionEvent::Select {
                        controller_id: 123,
                        character_selection: CharacterSelection::Id(slug_and_handle),
                    },
                )
            })
            .with_assertion(|world| {
                let character_selections = world.read_resource::<CharacterSelections>();

                assert_eq!(
                    Some(&SlugAndHandle::from((
                        &*world,
                        ASSETS_CHAR_BAT_SLUG.clone()
                    ))),
                    character_selections.selections.get(&123)
                );
            })
            .run()
    }

    #[test]
    fn removes_character_selection_on_deselect_event() -> Result<(), Error> {
        env::set_var("APP_DIR", env!("CARGO_MANIFEST_DIR"));

        AmethystApplication::blank()
            .with_bundle(TransformBundle::new())
            .with_bundle(RenderEmptyBundle::new())
            .with_custom_event_type::<AppEvent, AppEventReader>()
            .with_ui_bundles::<ControlBindings>()
            .with_system(Processor::<Source>::new(), "source_processor", &[])
            .with_bundle(SpriteLoadingBundle::new())
            .with_bundle(SequenceLoadingBundle::new())
            .with_bundle(LoadingBundle::new(ASSETS_PATH.clone()))
            .with_bundle(CollisionLoadingBundle::new())
            .with_bundle(MapLoadingBundle::new())
            .with_bundle(CharacterLoadingBundle::new())
            .with_bundle(CollisionAudioLoadingBundle::new(ASSETS_PATH.clone()))
            .with_bundle(UiAudioLoadingBundle::new(ASSETS_PATH.clone()))
            .with_state(|| LoadingState::new(PopState))
            .with_system(
                CharacterSelectionSystem::new(),
                CharacterSelectionSystem::type_name(),
                &[],
            )
            .with_setup(|world| {
                world
                    .write_resource::<CharacterSelections>()
                    .selections
                    .insert(
                        123,
                        SlugAndHandle::from((&*world, ASSETS_CHAR_BAT_SLUG.clone())),
                    );
            })
            .with_setup(|world| {
                send_event(
                    world,
                    CharacterSelectionEvent::Deselect { controller_id: 123 },
                )
            })
            .with_assertion(|world| {
                let character_selections = world.read_resource::<CharacterSelections>();

                assert_eq!(None, character_selections.selections.get(&123));
            })
            .run()
    }

    fn send_event(world: &mut World, event: CharacterSelectionEvent) {
        world
            .write_resource::<EventChannel<CharacterSelectionEvent>>()
            .single_write(event);
    }
}
