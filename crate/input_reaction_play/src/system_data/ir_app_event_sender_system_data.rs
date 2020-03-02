use amethyst::{
    ecs::{Read, ReadStorage, World, Write, WriteStorage},
    shred::{ResourceId, SystemData},
    shrev::EventChannel,
    ui::UiText,
};
use asset_model::loaded::{AssetId, AssetIdMappings, AssetTypeMappings};
use asset_selection_model::play::{AssetSelection, AssetSelectionEvent};
use asset_ui_model::play::{AssetSelectionHighlightMain, AssetSelectionStatus};
use chase_model::play::TargetObject;
use control_settings_model::ControlSettingsEvent;
use derivative::Derivative;
use game_input_model::play::InputControlled;
use game_mode_selection_model::GameModeSelectionEvent;
use game_play_model::GamePlayEvent;
use network_mode_selection_model::NetworkModeSelectionEvent;
use session_host_model::SessionHostEvent;
use session_join_model::SessionJoinEvent;
use state_registry::StateId;
use ui_form_model::play::UiFormInputEntities;

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

    /// `UiFormInputEntities` resource.
    #[derivative(Debug = "ignore")]
    pub ui_form_input_entities: Read<'s, UiFormInputEntities>,
    /// `UiText` components.
    #[derivative(Debug = "ignore")]
    pub ui_texts: ReadStorage<'s, UiText>,

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
    /// `NetworkModeSelectionEvent` channel.
    #[derivative(Debug = "ignore")]
    pub network_mode_selection_ec: Write<'s, EventChannel<NetworkModeSelectionEvent>>,
    /// `SessionHostEvent` channel.
    #[derivative(Debug = "ignore")]
    pub session_host_ec: Write<'s, EventChannel<SessionHostEvent>>,
    /// `SessionJoinEvent` channel.
    #[derivative(Debug = "ignore")]
    pub session_join_ec: Write<'s, EventChannel<SessionJoinEvent>>,
}
