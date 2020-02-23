#[cfg(test)]
mod tests {
    use amethyst::{
        ecs::{Builder, Entity, World, WorldExt},
        Error,
    };
    use amethyst_test::AmethystApplication;
    use indexmap::IndexMap;
    use sequence_model::loaded::SequenceId;
    use ui_model_spi::{config::WidgetStatus, loaded::WidgetStatusSequences};

    use ui_play::WidgetSequenceUpdateSystem;

    #[test]
    fn attaches_handle_and_sends_event_for_widget_status_insertions() -> Result<(), Error> {
        run_test(
            |world| create_entity(world, None),
            |world| insert_widget_status(world, WidgetStatus::Idle),
            Some(SequenceId::new(1)),
        )
    }

    #[test]
    fn attaches_handle_and_sends_event_for_widget_status_modifications() -> Result<(), Error> {
        run_test(
            |world| create_entity(world, Some(WidgetStatus::Idle)),
            |world| update_widget_status(world, WidgetStatus::Active),
            Some(SequenceId::new(2)),
        )
    }

    fn run_test(
        entity_create_fn: fn(&mut World),
        widget_status_alter_fn: fn(&mut World),
        sequence_id_expected: Option<SequenceId>,
    ) -> Result<(), Error> {
        AmethystApplication::blank()
            .with_system(WidgetSequenceUpdateSystem::new(), "", &[])
            .with_effect(entity_create_fn)
            .with_effect(widget_status_alter_fn)
            .with_assertion(move |world| expect_sequence_id(world, sequence_id_expected))
            .run()
    }

    fn insert_widget_status(world: &mut World, widget_status: WidgetStatus) {
        let entity = *world.read_resource::<Entity>();

        let mut widget_statuss = world.write_storage::<WidgetStatus>();
        widget_statuss
            .insert(entity, widget_status)
            .expect("Failed to insert `WidgetStatus`.");
    }

    fn update_widget_status(world: &mut World, widget_status: WidgetStatus) {
        let entity = *world.read_resource::<Entity>();

        let mut widget_statuss = world.write_storage::<WidgetStatus>();
        let sid = widget_statuss
            .get_mut(entity)
            .expect("Expected entity to contain `WidgetStatus` component.");
        *sid = widget_status;
    }

    fn create_entity(world: &mut World, widget_status: Option<WidgetStatus>) {
        let widget_status_sequences = {
            let mut widget_status_sequences = IndexMap::new();
            widget_status_sequences.insert(WidgetStatus::Idle, SequenceId::new(1));
            widget_status_sequences.insert(WidgetStatus::Active, SequenceId::new(2));
            WidgetStatusSequences::new(widget_status_sequences)
        };

        let mut entity_builder = world.create_entity().with(widget_status_sequences);
        if let Some(widget_status) = widget_status {
            entity_builder = entity_builder.with(widget_status);
        }
        let entity = entity_builder.build();

        world.insert(entity);
    }

    fn expect_sequence_id(world: &mut World, sequence_id: Option<SequenceId>) {
        let entity = *world.read_resource::<Entity>();
        let sequence_ides = world.read_storage::<SequenceId>();
        let sequence_id_actual = sequence_ides.get(entity).copied();

        assert_eq!(sequence_id, sequence_id_actual);
    }
}
