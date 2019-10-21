#[cfg(test)]
mod tests {
    use amethyst::{
        ecs::{World, WorldExt},
        shrev::EventChannel,
        Error,
    };
    use application_test_support::{AssetQueries, AutexousiousApplication};
    use asset_model::{config::AssetType, loaded::AssetId};
    use character_selection_model::{CharacterSelection, CharacterSelectionEvent};
    use object_type::ObjectType;

    use character_selection_ui::CharacterSelectionSfxSystem;

    #[test]
    fn plays_sound_on_return_event() -> Result<(), Error> {
        run_test(|_| CharacterSelectionEvent::Return)
    }

    #[test]
    fn plays_sound_on_join_event() -> Result<(), Error> {
        run_test(|_| CharacterSelectionEvent::Join { controller_id: 123 })
    }

    #[test]
    fn plays_sound_on_switch_event() -> Result<(), Error> {
        let character_selection_event_fn = |asset_id| {
            let character_selection = CharacterSelection::Id(asset_id);
            CharacterSelectionEvent::Switch {
                controller_id: 123,
                character_selection,
            }
        };
        run_test(character_selection_event_fn)
    }

    #[test]
    fn plays_sound_on_select_event() -> Result<(), Error> {
        let character_selection_event_fn = |asset_id| {
            let character_selection = CharacterSelection::Id(asset_id);
            CharacterSelectionEvent::Select {
                controller_id: 123,
                character_selection,
            }
        };
        run_test(character_selection_event_fn)
    }

    #[test]
    fn plays_sound_on_deselect_event() -> Result<(), Error> {
        run_test(|_| CharacterSelectionEvent::Deselect { controller_id: 123 })
    }

    #[test]
    fn plays_sound_on_leave_event() -> Result<(), Error> {
        run_test(|_| CharacterSelectionEvent::Leave { controller_id: 123 })
    }

    #[test]
    fn plays_sound_on_confirm_event() -> Result<(), Error> {
        run_test(|_| CharacterSelectionEvent::Confirm)
    }

    fn run_test(
        character_selection_event_fn: fn(AssetId) -> CharacterSelectionEvent,
    ) -> Result<(), Error> {
        AutexousiousApplication::config_base()
            .with_system(CharacterSelectionSfxSystem::new(), "", &[])
            .with_effect(move |world| {
                let asset_id =
                    AssetQueries::first_id(world, AssetType::Object(ObjectType::Character));
                let character_selection_event = character_selection_event_fn(asset_id);
                send_event(world, character_selection_event);
            })
            .with_assertion(|_world| {})
            .run_isolated()
    }

    fn send_event(world: &mut World, event: CharacterSelectionEvent) {
        let mut ec = world.write_resource::<EventChannel<CharacterSelectionEvent>>();
        ec.single_write(event)
    } // kcov-ignore
}
