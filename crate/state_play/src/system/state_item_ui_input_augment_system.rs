use amethyst::{
    ecs::{Entities, Entity, Read, ReadExpect, ReadStorage, System, World, Write, WriteStorage},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use derivative::Derivative;
use derive_new::new;
use game_input_model::{
    config::{ControllerId, PlayerInputConfigs},
    play::{ControllerInput, InputControlled},
};
use shrev_support::EventChannelExt;
use state_registry::{StateIdUpdateEvent, StateItemEntities};
use ui_model_spi::config::WidgetStatus;

/// Adds the `InputControlled` and `ControllerInput` components to `UiMenuItem` item entities.
#[derive(Debug, Default, new)]
pub struct StateItemUiInputAugmentSystem {
    /// Reader ID for the `StateIdUpdateEvent` channel.
    #[new(default)]
    state_id_update_event_rid: Option<ReaderId<StateIdUpdateEvent>>,
}

/// `StateItemUiInputAugmentSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct StateItemUiInputAugmentSystemData<'s> {
    /// `Entities`.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// `StateIdUpdateEvent` channel.
    #[derivative(Debug = "ignore")]
    pub state_id_update_ec: Read<'s, EventChannel<StateIdUpdateEvent>>,
    /// `StateItemEntities` resource.
    #[derivative(Debug = "ignore")]
    pub state_item_entities: Write<'s, StateItemEntities>,
    /// `WidgetStatus` components.
    #[derivative(Debug = "ignore")]
    pub widget_statuses: ReadStorage<'s, WidgetStatus>,
    /// `PlayerInputConfigs` resource.
    #[derivative(Debug = "ignore")]
    pub player_input_configs: ReadExpect<'s, PlayerInputConfigs>,
    /// `InputControlled` components.
    #[derivative(Debug = "ignore")]
    pub input_controlleds: WriteStorage<'s, InputControlled>,
    /// `ControllerInput` components.
    #[derivative(Debug = "ignore")]
    pub controller_inputs: WriteStorage<'s, ControllerInput>,
}

impl<'s> System<'s> for StateItemUiInputAugmentSystem {
    type SystemData = StateItemUiInputAugmentSystemData<'s>;

    fn run(
        &mut self,
        StateItemUiInputAugmentSystemData {
            entities,
            state_id_update_ec,
            mut state_item_entities,
            widget_statuses,
            player_input_configs,
            mut input_controlleds,
            mut controller_inputs,
        }: Self::SystemData,
    ) {
        let state_id_update_event_rid = self
            .state_id_update_event_rid
            .as_mut()
            .expect("Expected `state_id_update_event_rid` field to be set.");

        if let Some(_ev) = state_id_update_ec.last_event(state_id_update_event_rid) {
            let menu_items_exist = state_item_entities
                .entities
                .iter()
                .any(|entity| widget_statuses.get(*entity).is_some());

            // This creates another entity for each controller, which is an odd implementation.
            //
            // TODO: Perhaps instead of creating an entity for each controller, add a
            // `SharedInputControlled` component to each of the menu item entities, and:
            //
            // * Change the `SharedControllerInputUpdateSystem` to also write `ControlInputEvent`s
            //   for changes in the merged `ControllerInput` values.
            // * Change the `MenuItemWidgetInputSystem` to get the menu item entity based off the
            //   `ControlInputEvent` instead of joining and filtering.
            if menu_items_exist {
                let mut controller_entities = (0..player_input_configs.controller_configs.len())
                    .map(|index| {
                        let controller_id = index as ControllerId;
                        entities
                            .build_entity()
                            .with(InputControlled::new(controller_id), &mut input_controlleds)
                            .with(ControllerInput::default(), &mut controller_inputs)
                            .build()
                    })
                    .collect::<Vec<Entity>>();

                state_item_entities
                    .entities
                    .append(&mut controller_entities);
            }
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        self.state_id_update_event_rid = Some(
            world
                .fetch_mut::<EventChannel<StateIdUpdateEvent>>()
                .register_reader(),
        );
    }
}
