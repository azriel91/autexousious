use amethyst::{
    ecs::{Read, ReadStorage, System, World, WriteStorage},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use asset_ui_model::play::AssetSelectionHighlightMain;
use chase_model::play::TargetObject;
use derivative::Derivative;
use derive_new::new;
use game_input_model::{play::MoveDirection, AxisMoveEventData, ControlInputEvent};
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
    /// `AssetSelectionHighlightMain` components.
    #[derivative(Debug = "ignore")]
    pub asset_selection_highlight_mains: ReadStorage<'s, AssetSelectionHighlightMain>,
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
            siblingses,
            siblings_verticals,
        } = asset_selection_highlight_update_resources;
        let ash_entity = axis_move_event_data.entity;
        let asset_display_cell_entity = target_objects
            .get(ash_entity)
            .map(|target_object| target_object.entity);

        if let Some(asset_display_cell_entity) = asset_display_cell_entity {
            let move_direction = MoveDirection::from(axis_move_event_data);

            let asset_display_cell_sibling = match move_direction {
                MoveDirection::None => None,
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
            asset_selection_highlight_mains,
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
                    if asset_selection_highlight_mains.contains(axis_move_event_data.entity) {
                        Some(axis_move_event_data)
                    } else {
                        None
                    }
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
