use amethyst::{
    ecs::{Entities, Entity, Join, Read, ReadStorage, System, World, WriteStorage},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use character_selection_model::{CharacterSelection, CharacterSelectionEvent};
use character_selection_ui_model::{loaded::CswPortraits, play::CharacterSelectionParent};
use derivative::Derivative;
use derive_new::new;
use game_input::InputControlled;
use game_input_model::ControllerId;
use sequence_model::loaded::SequenceId;

/// Deletes entities whose `CharacterSelectionParent` is dead.
#[derive(Debug, Default, new)]
pub struct CswPortraitUpdateSystem {
    /// Reader ID for the `CharacterSelectionEvent` channel.
    #[new(default)]
    character_selection_event_rid: Option<ReaderId<CharacterSelectionEvent>>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct CswPortraitUpdateSystemData<'s> {
    /// `CharacterSelectionEvent` channel.
    #[derivative(Debug = "ignore")]
    pub character_selection_ec: Read<'s, EventChannel<CharacterSelectionEvent>>,
    /// `Entities`.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// `InputControlled` components.
    #[derivative(Debug = "ignore")]
    pub input_controlleds: ReadStorage<'s, InputControlled>,
    /// `CharacterSelectionParent` components.
    #[derivative(Debug = "ignore")]
    pub character_selection_parents: ReadStorage<'s, CharacterSelectionParent>,
    /// `CswPortraits` components.
    #[derivative(Debug = "ignore")]
    pub csw_portraitses: ReadStorage<'s, CswPortraits>,
    /// `CharacterSelection` components.
    #[derivative(Debug = "ignore")]
    pub character_selections: ReadStorage<'s, CharacterSelection>,
    /// `SequenceId` components.
    #[derivative(Debug = "ignore")]
    pub sequence_ids: WriteStorage<'s, SequenceId>,
}

impl CswPortraitUpdateSystem {
    /// Finds the portrait `Entity` with the given controller ID.
    ///
    /// Returns the entity and its `CharacterSelectionParent` if found.
    fn find_csw_entities(
        (entities, input_controlleds, character_selection_parents): (
            &Entities<'_>,
            &ReadStorage<'_, InputControlled>,
            &ReadStorage<'_, CharacterSelectionParent>,
        ),
        controller_id: ControllerId,
    ) -> Option<(Entity, CharacterSelectionParent)> {
        (entities, input_controlleds, character_selection_parents)
            .join()
            .find_map(
                |(entity_portrait, input_controlled, character_selection_parent)| {
                    if input_controlled.controller_id == controller_id {
                        Some((entity_portrait, *character_selection_parent))
                    } else {
                        None
                    }
                },
            )
    }
}

impl<'s> System<'s> for CswPortraitUpdateSystem {
    type SystemData = CswPortraitUpdateSystemData<'s>;

    fn run(&mut self, mut csw_portrait_update_system_data: Self::SystemData) {
        let CswPortraitUpdateSystemData {
            ref character_selection_ec,
            ref entities,
            ref input_controlleds,
            ref character_selection_parents,
            ref csw_portraitses,
            ref character_selections,
            ref mut sequence_ids,
        } = csw_portrait_update_system_data;

        let find_csw_data = (entities, input_controlleds, character_selection_parents);

        let character_selection_event_rid = self
            .character_selection_event_rid
            .as_mut()
            .expect("Expected `character_selection_event_rid` field to be set.");

        character_selection_ec
            .read(character_selection_event_rid)
            .for_each(|ev| match ev {
                CharacterSelectionEvent::Return => {}
                CharacterSelectionEvent::Join { controller_id } => {
                    if let Some((entity_portrait, csw_portraits, character_selection)) =
                        Self::find_csw_entities(find_csw_data, *controller_id).and_then(
                            |(entity_portrait, character_selection_parent)| {
                                let csw_portraits = csw_portraitses.get(entity_portrait);
                                let character_selection =
                                    character_selections.get(character_selection_parent.0);

                                csw_portraits.and_then(|csw_portraits| {
                                    character_selection.map(|character_selection| {
                                        (entity_portrait, csw_portraits, character_selection)
                                    })
                                })
                            },
                        )
                    {
                        let sequence_id = match character_selection {
                            CharacterSelection::Random => csw_portraits.random,
                            // TODO: empty portrait background
                            CharacterSelection::Id(_asset_id) => csw_portraits.random,
                        };
                        sequence_ids
                            .insert(entity_portrait, sequence_id)
                            .expect("Failed to insert `SequenceId` component.");
                    }
                }
                CharacterSelectionEvent::Leave { controller_id } => {
                    if let Some((entity_portrait, csw_portraits)) =
                        Self::find_csw_entities(find_csw_data, *controller_id).and_then(
                            |(entity_portrait, _character_selection_parent)| {
                                csw_portraitses
                                    .get(entity_portrait)
                                    .map(|csw_portraits| (entity_portrait, csw_portraits))
                            },
                        )
                    {
                        let sequence_id = csw_portraits.join;
                        sequence_ids
                            .insert(entity_portrait, sequence_id)
                            .expect("Failed to insert `SequenceId` component.");
                    }
                }
                CharacterSelectionEvent::Switch {
                    controller_id,
                    character_selection,
                } => {}
                CharacterSelectionEvent::Select {
                    controller_id,
                    character_selection,
                } => {}
                CharacterSelectionEvent::Deselect { controller_id } => {}
                CharacterSelectionEvent::Confirm => {}
            });
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        self.character_selection_event_rid = Some(
            world
                .fetch_mut::<EventChannel<CharacterSelectionEvent>>()
                .register_reader(),
        );
    }
}
