use amethyst::{
    ecs::{Read, ReadStorage, World, Write, WriteStorage},
    shred::{ResourceId, SystemData},
    shrev::EventChannel,
};
use asset_model::{
    loaded::{AssetId, AssetIdMappings, AssetTypeMappings},
    play::{AssetSelection, AssetSelectionEvent},
};
use asset_ui_model::play::AshStatus;
use character_selection_model::{CharacterSelection, CharacterSelectionEvent};
use character_selection_ui_model::play::CswStatus;
use chase_model::play::TargetObject;
use control_settings_model::ControlSettingsEvent;
use derivative::Derivative;
use game_input::InputControlled;
use game_mode_selection_model::GameModeSelectionEvent;
use game_play_model::GamePlayEvent;
use map_selection_model::{MapSelection, MapSelectionEvent};

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

    /// `InputControlled` components.
    #[derivative(Debug = "ignore")]
    pub input_controlleds: ReadStorage<'s, InputControlled>,

    /// `AssetSelectionEvent` channel.
    #[derivative(Debug = "ignore")]
    pub asset_selection_ec: Write<'s, EventChannel<AssetSelectionEvent>>,
    /// `AshStatus` components.
    #[derivative(Debug = "ignore")]
    pub ash_statuses: WriteStorage<'s, AshStatus>,
    /// `TargetObject` components.
    #[derivative(Debug = "ignore")]
    pub target_objects: ReadStorage<'s, TargetObject>,
    /// `AssetSelection` components.
    #[derivative(Debug = "ignore")]
    pub asset_selections: ReadStorage<'s, AssetSelection>,

    /// `CharacterSelectionEvent` channel.
    #[derivative(Debug = "ignore")]
    pub character_selection_ec: Write<'s, EventChannel<CharacterSelectionEvent>>,
    /// `CharacterSelection` components.
    #[derivative(Debug = "ignore")]
    pub character_selections: WriteStorage<'s, CharacterSelection>,
    /// `CswStatus` components.
    #[derivative(Debug = "ignore")]
    pub csw_statuses: WriteStorage<'s, CswStatus>,

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
