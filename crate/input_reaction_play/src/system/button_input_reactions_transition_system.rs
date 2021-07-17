use std::{fmt::Debug, marker::PhantomData};

use amethyst::{
    assets::AssetStorage,
    core::SystemDesc,
    ecs::{BitSet, Entities, Entity, Join, Read, ReadStorage, System, World, WriteStorage},
    input::{Button, InputEvent},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
    ui::{UiEvent, UiEventType},
    winit::event::MouseButton,
};
use derivative::Derivative;
use derive_new::new;
use game_input_model::{config::ControlBindings, play::ButtonInputControlled};
use input_reaction_model::{
    config::{InputReactionAppEvents, InputReactionRequirement},
    loaded::{
        InputReaction, InputReactions, InputReactionsHandle, ReactionEffect, ReactionEffectButton,
    },
};
use sequence_model::loaded::SequenceId;

use crate::{IrAppEventSender, IrAppEventSenderSystemData};

/// Builds a `ButtonInputReactionsTransitionSystem`.
#[derive(Debug, Default)]
pub struct ButtonInputReactionsTransitionSystemDesc;

impl<'a, 'b, IRR> SystemDesc<'a, 'b, ButtonInputReactionsTransitionSystem<IRR>>
    for ButtonInputReactionsTransitionSystemDesc
where
    IRR: InputReactionRequirement<'a> + Send + Sync + 'static,
    IRR::SystemData: Debug,
{
    fn build(self, world: &mut World) -> ButtonInputReactionsTransitionSystem<IRR> {
        <ButtonInputReactionsTransitionSystem<IRR> as System<'_>>::SystemData::setup(world);

        let input_event_rid = world
            .fetch_mut::<EventChannel<InputEvent<ControlBindings>>>()
            .register_reader();
        let ui_event_rid = world.fetch_mut::<EventChannel<UiEvent>>().register_reader();

        ButtonInputReactionsTransitionSystem::new(input_event_rid, ui_event_rid)
    }
}

/// Updates `SequenceId` based on `InputEvent::ButtonPress`es.
///
/// # Type Parameters
///
/// * `IRR`: `InputReactionRequirement`.
#[derive(Debug, new)]
pub struct ButtonInputReactionsTransitionSystem<IRR> {
    /// Reader ID for the `InputEvent<ControlBindings>` channel.
    input_event_rid: ReaderId<InputEvent<ControlBindings>>,
    /// Reader ID for the `UiEvent` channel.
    ui_event_rid: ReaderId<UiEvent>,
    /// Pre-allocated bitset to track entities whose transitions have already
    /// been checked.
    #[new(default)]
    processed_entities: BitSet,
    /// Marker.
    marker: PhantomData<IRR>,
}

/// `ButtonInputReactionsTransitionSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct ButtonInputReactionsTransitionSystemData<'s, IRR>
where
    IRR: InputReactionRequirement<'s> + Send + Sync + 'static,
    IRR::SystemData: Debug,
{
    /// `InputEvent<ControlBindings>` channel.
    #[derivative(Debug = "ignore")]
    pub input_ec: Read<'s, EventChannel<InputEvent<ControlBindings>>>,
    /// `UiEvent` channel.
    #[derivative(Debug = "ignore")]
    pub ui_ec: Read<'s, EventChannel<UiEvent>>,
    /// `Entities` resource.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// `ButtonInputControlled` components.
    #[derivative(Debug = "ignore")]
    pub button_input_controlleds: ReadStorage<'s, ButtonInputControlled>,
    /// `ButtonInputReactionsTransitionResources`.
    pub input_reactions_transition_resources: ButtonInputReactionsTransitionResources<'s, IRR>,
    /// `InputReactionRequirement` system data.
    pub requirement_system_data: IRR::SystemData,
}

/// `ButtonInputReactionsTransitionResources`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct ButtonInputReactionsTransitionResources<'s, IRR>
where
    IRR: Send + Sync + 'static,
{
    /// `InputReactionsHandle` components.
    #[derivative(Debug = "ignore")]
    pub input_reactions_handles: ReadStorage<'s, InputReactionsHandle<InputReaction<IRR>>>,
    /// `InputReactions` assets.
    #[derivative(Debug = "ignore")]
    pub input_reactions_assets: Read<'s, AssetStorage<InputReactions<InputReaction<IRR>>>>,
    /// `SequenceId` components.
    #[derivative(Debug = "ignore")]
    pub sequence_ids: WriteStorage<'s, SequenceId>,
    /// `IrAppEventSenderSystemData`.
    #[derivative(Debug = "ignore")]
    pub ir_app_event_sender_system_data: IrAppEventSenderSystemData<'s>,
}

impl<'s, IRR> ButtonInputReactionsTransitionSystem<IRR>
where
    IRR: InputReactionRequirement<'s> + Send + Sync + 'static,
{
    fn handle_button_event(
        &mut self,
        ButtonInputReactionsTransitionResources {
            ref input_reactions_handles,
            ref input_reactions_assets,
            ref mut sequence_ids,
            ref mut ir_app_event_sender_system_data,
        }: &mut ButtonInputReactionsTransitionResources<IRR>,
        requirement_system_data: &mut IRR::SystemData,
        entity: Entity,
        button: Button,
        value: bool,
    ) {
        self.processed_entities.add(entity.id());

        if let Some(input_reactions_handle) = input_reactions_handles.get(entity) {
            let input_reactions = input_reactions_assets
                .get(input_reactions_handle)
                .expect("Expected `InputReactions` to be loaded.");

            let transition_sequence_id = input_reactions
                .iter()
                .filter_map(|input_reaction| {
                    let input_reaction_requirement = &input_reaction.requirement;

                    if let ReactionEffect::ButtonPress(ReactionEffectButton {
                        button: reaction_button,
                        sequence_id,
                        events,
                    }) = &input_reaction.effect
                    {
                        if value && button == *reaction_button {
                            Some((*sequence_id, events, input_reaction_requirement))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .filter_map(|(sequence_id, events, input_reaction_requirement)| {
                    Self::process_transition(
                        requirement_system_data,
                        entity,
                        sequence_id,
                        events,
                        input_reaction_requirement,
                    )
                })
                .next();

            if let Some((transition_sequence_id, events)) = transition_sequence_id {
                events.iter().copied().for_each(|event| {
                    IrAppEventSender::send(ir_app_event_sender_system_data, None, entity, event)
                });

                sequence_ids
                    .insert(entity, transition_sequence_id)
                    .expect("Failed to insert `SequenceId` component.");
            }
        }
    }

    fn process_transition<'f>(
        requirement_system_data: &mut IRR::SystemData,
        entity: Entity,
        sequence_id: SequenceId,
        events: &'f InputReactionAppEvents,
        input_reaction_requirement: &IRR,
    ) -> Option<(SequenceId, &'f InputReactionAppEvents)> {
        if input_reaction_requirement.requirement_met(requirement_system_data, entity) {
            Some((sequence_id, events))
        } else {
            None
        }
    }
}

impl<'s, IRR> System<'s> for ButtonInputReactionsTransitionSystem<IRR>
where
    IRR: InputReactionRequirement<'s> + Send + Sync + 'static,
    IRR::SystemData: Debug,
{
    type SystemData = ButtonInputReactionsTransitionSystemData<'s, IRR>;

    fn run(
        &mut self,
        ButtonInputReactionsTransitionSystemData {
            input_ec,
            ui_ec,
            entities,
            button_input_controlleds,
            mut input_reactions_transition_resources,
            mut requirement_system_data,
        }: Self::SystemData,
    ) {
        self.processed_entities.clear();

        let button_controlled_entities = (&entities, &button_input_controlleds)
            .join()
            .map(|(entity, _)| entity)
            .collect::<Vec<Entity>>();

        // We use `InputEvent::KeyPressed` for entities that should react regardless of
        // whether or not the entity is a UI widget, and whether or not it is
        // focused.
        //
        // For mouse events, we need to make sure the click happened on the UI widget,
        // so we use `UiEvent`s. Note that the entity must have the
        // `Selectable<()>` and `Interactable` components.
        //
        // If we did not care about the coordinates of the mouse click, then we could
        // use `InputEvent::ButtonPressed` for both kinds of tests.
        input_ec.read(&mut self.input_event_rid).for_each(|ev| {
            if let InputEvent::KeyPressed { key_code, .. } = ev {
                let button = Button::Key(*key_code);

                button_controlled_entities
                    .iter()
                    .copied()
                    .for_each(|entity| {
                        self.handle_button_event(
                            &mut input_reactions_transition_resources,
                            &mut requirement_system_data,
                            entity,
                            button,
                            true,
                        );
                    });
            }
        });
        ui_ec.read(&mut self.ui_event_rid).for_each(|ev| {
            if let UiEvent {
                event_type: UiEventType::Click,
                target,
            } = ev
            {
                // TODO: What about right / middle click?
                let button = Button::Mouse(MouseButton::Left);

                button_controlled_entities
                    .iter()
                    .copied()
                    .filter(|entity| entity == target)
                    .for_each(|entity| {
                        self.handle_button_event(
                            &mut input_reactions_transition_resources,
                            &mut requirement_system_data,
                            entity,
                            button,
                            true,
                        );
                    });
            }
        });
    }
}
