#[cfg(test)]
mod tests {
    use amethyst::{
        ecs::{World, WorldExt},
        shrev::EventChannel,
        Error,
    };
    use application_test_support::{AssetQueries, AutexousiousApplication};
    use asset_model::{config::AssetType, loaded::AssetId};
    use asset_selection_model::play::{AssetSelection, AssetSelectionEvent};
    use object_type::ObjectType;

    use asset_selection_ui_play::AssetSelectionSfxSystem;

    #[test]
    fn plays_sound_on_return_event() -> Result<(), Error> {
        run_test(|_| AssetSelectionEvent::Return)
    }

    #[test]
    fn plays_sound_on_join_event() -> Result<(), Error> {
        run_test(|_| AssetSelectionEvent::Join {
            entity: None,
            controller_id: 123,
        })
    }

    #[test]
    fn plays_sound_on_switch_event() -> Result<(), Error> {
        let asset_selection_event_fn = |asset_id| {
            let asset_selection = AssetSelection::Id(asset_id);
            AssetSelectionEvent::Switch {
                entity: None,
                controller_id: 123,
                asset_selection,
            }
        };
        run_test(asset_selection_event_fn)
    }

    #[test]
    fn plays_sound_on_select_event() -> Result<(), Error> {
        let asset_selection_event_fn = |asset_id| {
            let asset_selection = AssetSelection::Id(asset_id);
            AssetSelectionEvent::Select {
                entity: None,
                controller_id: 123,
                asset_selection,
            }
        };
        run_test(asset_selection_event_fn)
    }

    #[test]
    fn plays_sound_on_deselect_event() -> Result<(), Error> {
        run_test(|_| AssetSelectionEvent::Deselect {
            entity: None,
            controller_id: 123,
        })
    }

    #[test]
    fn plays_sound_on_leave_event() -> Result<(), Error> {
        run_test(|_| AssetSelectionEvent::Leave {
            entity: None,
            controller_id: 123,
        })
    }

    #[test]
    fn plays_sound_on_confirm_event() -> Result<(), Error> {
        run_test(|_| AssetSelectionEvent::Confirm)
    }

    fn run_test(asset_selection_event_fn: fn(AssetId) -> AssetSelectionEvent) -> Result<(), Error> {
        AutexousiousApplication::config_base()
            .with_system(AssetSelectionSfxSystem::new(), "", &[])
            .with_effect(move |world| {
                let asset_id =
                    AssetQueries::first_id(world, AssetType::Object(ObjectType::Character));
                let asset_selection_event = asset_selection_event_fn(asset_id);
                send_event(world, asset_selection_event);
            })
            .with_assertion(|_world| {})
            .run_isolated()
    }

    fn send_event(world: &mut World, event: AssetSelectionEvent) {
        let mut ec = world.write_resource::<EventChannel<AssetSelectionEvent>>();
        ec.single_write(event)
    } // kcov-ignore
}
