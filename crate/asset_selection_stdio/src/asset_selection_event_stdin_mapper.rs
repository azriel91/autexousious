use std::str::FromStr;

use amethyst::{ecs::Read, Error};
use asset_model::{config::AssetSlug, loaded::AssetIdMappings};
use asset_selection_model::{
    config::AssetSelectionEventArgs,
    play::{AssetSelection, AssetSelectionEvent},
};
use game_input_model::config::ControllerId;
use stdio_spi::{MapperSystemData, StdinMapper, StdioError};

/// Magic string to indicate `random` selection.
const RANDOM_SELECTION: &str = "random";

#[derive(Debug)]
pub struct AssetSelectionEventStdinMapperData;

impl<'s> MapperSystemData<'s> for AssetSelectionEventStdinMapperData {
    type SystemData = Read<'s, AssetIdMappings>;
}

/// Builds a `AssetSelectionEvent` from stdin tokens.
#[derive(Debug)]
pub struct AssetSelectionEventStdinMapper;

impl AssetSelectionEventStdinMapper {
    fn map_switch_event(
        asset_id_mappings: &AssetIdMappings,
        controller_id: ControllerId,
        selection: &str,
    ) -> Result<AssetSelectionEvent, Error> {
        let asset_selection = Self::find_character(asset_id_mappings, selection)?;

        let asset_selection_event = AssetSelectionEvent::Switch {
            entity: None,
            controller_id,
            asset_selection,
        };

        Ok(asset_selection_event)
    }

    fn map_select_event(
        asset_id_mappings: &AssetIdMappings,
        controller_id: ControllerId,
        selection: &str,
    ) -> Result<AssetSelectionEvent, Error> {
        let asset_selection = Self::find_character(asset_id_mappings, selection)?;

        let asset_selection_event = AssetSelectionEvent::Select {
            entity: None,
            controller_id,
            asset_selection,
        };

        Ok(asset_selection_event)
    }

    fn find_character(
        asset_id_mappings: &AssetIdMappings,
        selection: &str,
    ) -> Result<AssetSelection, Error> {
        match selection {
            RANDOM_SELECTION => Ok(AssetSelection::Random),
            slug_str => {
                let slug = AssetSlug::from_str(slug_str)
                    .map_err(String::from)
                    .map_err(StdioError::Msg)?;

                let asset_id = asset_id_mappings
                    .id(&slug)
                    .copied()
                    .ok_or_else(|| format!("No character found with asset slug `{}`.", slug))
                    .map_err(StdioError::Msg)?;

                Ok(AssetSelection::Id(asset_id))
            }
        }
    }
}

impl StdinMapper for AssetSelectionEventStdinMapper {
    type SystemData = AssetSelectionEventStdinMapperData;
    type Event = AssetSelectionEvent;
    type Args = AssetSelectionEventArgs;

    fn map(
        asset_id_mappings: &Read<AssetIdMappings>,
        args: Self::Args,
    ) -> Result<Self::Event, Error> {
        match args {
            AssetSelectionEventArgs::Return => Ok(AssetSelectionEvent::Return),
            AssetSelectionEventArgs::Switch {
                controller_id,
                selection,
            } => Self::map_switch_event(asset_id_mappings, controller_id, &selection),
            AssetSelectionEventArgs::Select {
                controller_id,
                selection,
            } => Self::map_select_event(asset_id_mappings, controller_id, &selection),
            AssetSelectionEventArgs::Deselect { controller_id } => {
                Ok(AssetSelectionEvent::Deselect {
                    entity: None,
                    controller_id,
                })
            }
            AssetSelectionEventArgs::Join { controller_id } => Ok(AssetSelectionEvent::Join {
                entity: None,
                controller_id,
            }),
            AssetSelectionEventArgs::Leave { controller_id } => Ok(AssetSelectionEvent::Leave {
                entity: None,
                controller_id,
            }),
            AssetSelectionEventArgs::Confirm => Ok(AssetSelectionEvent::Confirm),
        }
    }
}
