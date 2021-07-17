use amethyst::{
    derive::SystemDesc,
    ecs::{Entities, Entity, Read, ReadStorage, System, World, Write, WriteStorage},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use derivative::Derivative;
use derive_new::new;
use game_input_model::{
    config::ControllerId,
    loaded::PlayerControllers,
    play::{ControllerInput, InputControlled},
};
use shrev_support::EventChannelExt;
use state_registry::{StateIdUpdateEvent, StateItemEntities};
use ui_model_spi::config::WidgetStatus;

/// Adds the `InputControlled` and `ControllerInput` components to `UiMenuItem`
/// item entities.
#[derive(Debug, SystemDesc, new)]
#[system_desc(name(StateItemUiInputAugmentSystemDesc))]
pub struct StateItemUiInputAugmentSystem {
    /// Reader ID for the `StateIdUpdateEvent` channel.
    #[system_desc(event_channel_reader)]
    state_id_update_event_rid: ReaderId<StateIdUpdateEvent>,
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
    /// `PlayerControllers` resource.
    #[derivative(Debug = "ignore")]
    pub player_controllers: Read<'s, PlayerControllers>,
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
            player_controllers,
            mut input_controlleds,
            mut controller_inputs,
        }: Self::SystemData,
    ) {
        if state_id_update_ec
            .last_event(&mut self.state_id_update_event_rid)
            .is_some()
        {
            let menu_items_exist = state_item_entities
                .entities
                .iter()
                .any(|entity| widget_statuses.get(*entity).is_some());

            // This creates another entity for each controller, which is an odd
            // implementation.
            //
            // TODO: Perhaps instead of creating an entity for each controller, add a
            // `SharedInputControlled` component to each of the menu item entities, and:
            //
            // * Change the `SharedControllerInputUpdateSystem` to also write
            //   `ControlInputEvent`s for changes in the merged `ControllerInput` values.
            // * Change the `MenuItemWidgetInputSystem` to get the menu item entity based
            //   off the `ControlInputEvent` instead of joining and filtering.
            if menu_items_exist {
                let mut controller_entities = (0..player_controllers.len())
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
}
