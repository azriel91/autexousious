use amethyst::{
    ecs::{Read, ReadStorage, World, Write, WriteStorage},
    shred::{ResourceId, SystemData},
    shrev::EventChannel,
};
use asset_model::{
    loaded::{AssetId, AssetIdMappings, AssetTypeMappings},
    play::{AssetSelection, AssetSelectionEvent},
};
use asset_ui_model::play::{AssetSelectionHighlightMain, AssetSelectionStatus};
use chase_model::play::TargetObject;
use control_settings_model::ControlSettingsEvent;
use derivative::Derivative;
use game_input::InputControlled;
use game_mode_selection_model::GameModeSelectionEvent;
use game_play_model::GamePlayEvent;
use map_selection_model::{MapSelection, MapSelectionEvent};
use state_registry::StateId;

/// `IrAppEventSenderSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct IrAppEventSenderSystemData<'s> {
    /// `AssetId` components.
    #[derivative(Debug = "ignore")]
    pub asset_ids: ReadStorage<'s, AssetId>,
    /// `AssetIdMappings` resource.
    #[derivative(Debug = "ignore")]
    pub asset_id_mappings: Read<'s, AssetIdMappings>,
    /// `AssetTypeMappings` resource.
    #[derivative(Debug = "ignore")]
    pub asset_type_mappings: Read<'s, AssetTypeMappings>,

    /// `StateId` resource.
    #[derivative(Debug = "ignore")]
    pub state_id: Read<'s, StateId>,

    /// `InputControlled` components.
    #[derivative(Debug = "ignore")]
    pub input_controlleds: ReadStorage<'s, InputControlled>,

    /// `AssetSelectionEvent` channel.
    #[derivative(Debug = "ignore")]
    pub asset_selection_ec: Write<'s, EventChannel<AssetSelectionEvent>>,
    /// `AssetSelectionStatus` components.
    #[derivative(Debug = "ignore")]
    pub asset_selection_statuses: WriteStorage<'s, AssetSelectionStatus>,
    /// `AssetSelectionHighlightMain` components.
    #[derivative(Debug = "ignore")]
    pub asset_selection_highlight_mains: ReadStorage<'s, AssetSelectionHighlightMain>,
    /// `TargetObject` components.
    #[derivative(Debug = "ignore")]
    pub target_objects: ReadStorage<'s, TargetObject>,
    /// `AssetSelection` components.
    #[derivative(Debug = "ignore")]
    pub asset_selections: ReadStorage<'s, AssetSelection>,

    /// `ControlSettingsEvent` channel.
    #[derivative(Debug = "ignore")]
    pub control_settings_ec: Write<'s, EventChannel<ControlSettingsEvent>>,
    /// `GameModeSelectionEvent` channel.
    #[derivative(Debug = "ignore")]
    pub game_mode_selection_ec: Write<'s, EventChannel<GameModeSelectionEvent>>,
    /// `GamePlayEvent` channel.
    #[derivative(Debug = "ignore")]
    pub game_play_ec: Write<'s, EventChannel<GamePlayEvent>>,

    /// `MapSelectionEvent` channel.
    #[derivative(Debug = "ignore")]
    pub map_selection_ec: Write<'s, EventChannel<MapSelectionEvent>>,
    /// `MapSelection` components.
    #[derivative(Debug = "ignore")]
    pub map_selections: ReadStorage<'s, MapSelection>,
}
