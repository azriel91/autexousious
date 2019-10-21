#[cfg(test)]
mod tests {
    use amethyst::{
        ecs::{World, WorldExt},
        shrev::EventChannel,
        Error,
    };
    use application_test_support::AutexousiousApplication;
    use asset_model::{config::AssetType, loaded::AssetTypeMappings};
    use map_selection_model::{MapSelection, MapSelectionEvent};

    use map_selection_ui::MapSelectionSfxSystem;

    #[test]
    fn plays_sound_on_return_event() -> Result<(), Error> {
        run_test(|_world| MapSelectionEvent::Return)
    }

    #[test]
    fn plays_sound_on_switch_event() -> Result<(), Error> {
        run_test(|world| {
            let asset_id = world
                .read_resource::<AssetTypeMappings>()
                .iter_ids(&AssetType::Map)
                .next()
                .copied()
                .expect("Expected at least one map to be loaded.");
            let map_selection = MapSelection::Id(asset_id);
            MapSelectionEvent::Switch { map_selection }
        })
    }

    #[test]
    fn plays_sound_on_select_event() -> Result<(), Error> {
        run_test(|world| {
            let asset_id = world
                .read_resource::<AssetTypeMappings>()
                .iter_ids(&AssetType::Map)
                .next()
                .copied()
                .expect("Expected at least one map to be loaded.");
            let map_selection = MapSelection::Id(asset_id);
            MapSelectionEvent::Select { map_selection }
        })
    }

    #[test]
    fn plays_sound_on_deselect_event() -> Result<(), Error> {
        run_test(|_world| MapSelectionEvent::Deselect)
    }

    #[test]
    fn plays_sound_on_confirm_event() -> Result<(), Error> {
        run_test(|_world| MapSelectionEvent::Confirm)
    }

    fn run_test<F>(event_fn: F) -> Result<(), Error>
    where
        F: Fn(&mut World) -> MapSelectionEvent + Send + Sync + 'static,
    {
        AutexousiousApplication::config_base()
            .with_system(MapSelectionSfxSystem::new(), "", &[])
            .with_effect(move |world| {
                let event = event_fn(world);
                send_event(world, event);
            })
            .with_assertion(|_world| {})
            .run()
    }

    fn send_event(world: &mut World, event: MapSelectionEvent) {
        let mut ec = world.write_resource::<EventChannel<MapSelectionEvent>>();
        ec.single_write(event)
    } // kcov-ignore
}
