use amethyst::{
    ecs::{Read, ReadStorage, System, World, WriteStorage},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use asset_selection_model::play::AssetSelection;
use asset_ui_model::play::{
    AssetSelectionHighlightMain, AssetSelectionParent, AssetSelectionStatus,
};
use chase_model::play::TargetObject;
use derivative::Derivative;
use derive_new::new;
use game_input_model::play::{AxisMoveEventData, ControlInputEvent, MoveDirection};
use ui_model_spi::play::{Siblings, SiblingsVertical};

/// Updates the `AssetSelectionHighlight` when receving a `ControlInputEvent`.
///
/// This simply updates the `TargetObject` to the appropriate sibling of the current
/// `AssetDisplayCell`.
#[derive(Debug, Default, new)]
pub struct AssetSelectionHighlightUpdateSystem {
    /// Reader ID for the `ControlInputEvent` channel.
    #[new(default)]
    control_input_event_rid: Option<ReaderId<ControlInputEvent>>,
}

/// `AssetSelectionHighlightUpdateSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct AssetSelectionHighlightUpdateSystemData<'s> {
    /// `ControlInputEvent` channel.
    #[derivative(Debug = "ignore")]
    pub control_input_ec: Read<'s, EventChannel<ControlInputEvent>>,
    /// `AssetSelectionHighlightUpdateResources`.
    pub asset_selection_highlight_update_resources: AssetSelectionHighlightUpdateResources<'s>,
}

/// `AssetSelectionHighlightUpdateResources`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct AssetSelectionHighlightUpdateResources<'s> {
    /// `TargetObject` components.
    #[derivative(Debug = "ignore")]
    pub target_objects: WriteStorage<'s, TargetObject>,
    /// `AssetSelectionParent` components.
    #[derivative(Debug = "ignore")]
    pub asset_selection_parents: WriteStorage<'s, AssetSelectionParent>,
    /// `AssetSelection` components.
    #[derivative(Debug = "ignore")]
    pub asset_selections: WriteStorage<'s, AssetSelection>,
    /// `AssetSelectionHighlightMain` components.
    #[derivative(Debug = "ignore")]
    pub asset_selection_highlight_mains: ReadStorage<'s, AssetSelectionHighlightMain>,
    /// `AssetSelectionStatus` components.
    #[derivative(Debug = "ignore")]
    pub asset_selection_statuses: ReadStorage<'s, AssetSelectionStatus>,
    /// `Siblings` components.
    #[derivative(Debug = "ignore")]
    pub siblingses: ReadStorage<'s, Siblings>,
    /// `SiblingsVertical` components.
    #[derivative(Debug = "ignore")]
    pub siblings_verticals: ReadStorage<'s, SiblingsVertical>,
}

impl AssetSelectionHighlightUpdateSystem {
    fn update_selection(
        asset_selection_highlight_update_resources: &mut AssetSelectionHighlightUpdateResources<'_>,
        axis_move_event_data: AxisMoveEventData,
    ) {
        let AssetSelectionHighlightUpdateResources {
            target_objects,
            asset_selection_parents,
            asset_selections,
            asset_selection_highlight_mains,
            asset_selection_statuses,
            siblingses,
            siblings_verticals,
        } = asset_selection_highlight_update_resources;

        let ash_entity = target_objects
            .get(axis_move_event_data.entity)
            .map(|target_object| target_object.entity);

        if let Some(ash_entity) = ash_entity {
            if asset_selection_highlight_mains.contains(ash_entity)
                && asset_selection_statuses.get(ash_entity).copied()
                    == Some(AssetSelectionStatus::InProgress)
            {
                let asset_display_cell_entity = target_objects
                    .get(ash_entity)
                    .map(|target_object| target_object.entity);

                if let Some(asset_display_cell_entity) = asset_display_cell_entity {
                    let move_direction = MoveDirection::from(axis_move_event_data);

                    let asset_display_cell_sibling = match move_direction {
                        MoveDirection::None => None,
                        // TODO: attach `SiblingsVertical` to entities. See:
                        //
                        // * `AssetSelector::augment_siblings`
                        // * `UiMenu::augment_siblings`
                        MoveDirection::Up => siblings_verticals
                            .get(asset_display_cell_entity)
                            .and_then(|siblings_vertical| siblings_vertical.up),
                        MoveDirection::Down => siblings_verticals
                            .get(asset_display_cell_entity)
                            .and_then(|siblings_vertical| siblings_vertical.down),
                        MoveDirection::Left => siblingses
                            .get(asset_display_cell_entity)
                            .and_then(|siblings| siblings.previous),
                        MoveDirection::Right => siblingses
                            .get(asset_display_cell_entity)
                            .and_then(|siblings| siblings.next),
                    };

                    if let Some(asset_display_cell_sibling) = asset_display_cell_sibling {
                        target_objects
                            .insert(ash_entity, TargetObject::new(asset_display_cell_sibling))
                            .expect("Failed to insert `TargetObject` component.");
                        asset_selection_parents
                            .insert(
                                ash_entity,
                                AssetSelectionParent::new(asset_display_cell_sibling),
                            )
                            .expect("Failed to insert `AssetSelectionParent` component.");
                        let asset_selection = asset_selections
                            .get(asset_display_cell_sibling)
                            .copied()
                            .expect(
                                "Expected `AssetSelectionCell` entity to have `AssetSelection` \
                                component.",
                            );
                        asset_selections
                            .insert(ash_entity, asset_selection)
                            .expect("Failed to insert `AssetSelection` component.");
                    }
                }
            }
        }
    }
}

impl<'s> System<'s> for AssetSelectionHighlightUpdateSystem {
    type SystemData = AssetSelectionHighlightUpdateSystemData<'s>;

    fn run(
        &mut self,
        AssetSelectionHighlightUpdateSystemData {
            control_input_ec,
            mut asset_selection_highlight_update_resources,
        }: Self::SystemData,
    ) {
        let control_input_event_rid = self
            .control_input_event_rid
            .as_mut()
            .expect("Expected `control_input_event_rid` field to be set.");
        control_input_ec
            .read(control_input_event_rid)
            .filter_map(|ev| {
                if let ControlInputEvent::AxisMoved(axis_move_event_data) = ev {
                    Some(axis_move_event_data)
                } else {
                    None
                }
            })
            .copied()
            .for_each(|axis_move_event_data| {
                Self::update_selection(
                    &mut asset_selection_highlight_update_resources,
                    axis_move_event_data,
                )
            });
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        self.control_input_event_rid = Some(
            world
                .fetch_mut::<EventChannel<ControlInputEvent>>()
                .register_reader(),
        );
    }
}
