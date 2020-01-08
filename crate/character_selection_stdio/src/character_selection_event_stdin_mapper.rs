use std::str::FromStr;

use amethyst::{ecs::Read, Error};
use asset_model::{config::AssetSlug, loaded::AssetIdMappings};
use character_selection_model::{
    CharacterSelection, CharacterSelectionEvent, CharacterSelectionEventArgs,
};
use game_input_model::ControllerId;
use stdio_spi::{MapperSystemData, StdinMapper, StdioError};

/// Magic string to indicate `random` selection.
const RANDOM_SELECTION: &str = "random";

#[derive(Debug)]
pub struct CharacterSelectionEventStdinMapperData;

impl<'s> MapperSystemData<'s> for CharacterSelectionEventStdinMapperData {
    type SystemData = Read<'s, AssetIdMappings>;
}

/// Builds a `CharacterSelectionEvent` from stdin tokens.
#[derive(Debug)]
pub struct CharacterSelectionEventStdinMapper;

impl CharacterSelectionEventStdinMapper {
    fn map_switch_event(
        asset_id_mappings: &AssetIdMappings,
        controller_id: ControllerId,
        selection: &str,
    ) -> Result<CharacterSelectionEvent, Error> {
        let character_selection = Self::find_character(asset_id_mappings, selection)?;

        let character_selection_event = CharacterSelectionEvent::Switch {
            controller_id,
            character_selection,
        };

        Ok(character_selection_event)
    }

    fn map_select_event(
        asset_id_mappings: &AssetIdMappings,
        controller_id: ControllerId,
        selection: &str,
    ) -> Result<CharacterSelectionEvent, Error> {
        let character_selection = Self::find_character(asset_id_mappings, selection)?;

        let character_selection_event = CharacterSelectionEvent::Select {
            controller_id,
            character_selection,
        };

        Ok(character_selection_event)
    }

    fn find_character(
        asset_id_mappings: &AssetIdMappings,
        selection: &str,
    ) -> Result<CharacterSelection, Error> {
        match selection {
            RANDOM_SELECTION => Ok(CharacterSelection::Random),
            slug_str => {
                let slug = AssetSlug::from_str(slug_str)
                    .map_err(String::from)
                    .map_err(StdioError::Msg)?;

                let asset_id = asset_id_mappings
                    .id(&slug)
                    .copied()
                    .ok_or_else(|| format!("No character found with asset slug `{}`.", slug))
                    .map_err(StdioError::Msg)?;

                Ok(CharacterSelection::Id(asset_id))
            }
        }
    }
}

impl StdinMapper for CharacterSelectionEventStdinMapper {
    type SystemData = CharacterSelectionEventStdinMapperData;
    type Event = CharacterSelectionEvent;
    type Args = CharacterSelectionEventArgs;

    fn map(
        asset_id_mappings: &Read<AssetIdMappings>,
        args: Self::Args,
    ) -> Result<Self::Event, Error> {
        match args {
            CharacterSelectionEventArgs::Return => Ok(CharacterSelectionEvent::Return),
            CharacterSelectionEventArgs::Switch {
                controller_id,
                selection,
            } => Self::map_switch_event(asset_id_mappings, controller_id, &selection),
            CharacterSelectionEventArgs::Select {
                controller_id,
                selection,
            } => Self::map_select_event(asset_id_mappings, controller_id, &selection),
            CharacterSelectionEventArgs::Deselect { controller_id } => {
                Ok(CharacterSelectionEvent::Deselect { controller_id })
            }
            CharacterSelectionEventArgs::Join { controller_id } => {
                Ok(CharacterSelectionEvent::Join { controller_id })
            }
            CharacterSelectionEventArgs::Leave { controller_id } => {
                Ok(CharacterSelectionEvent::Leave { controller_id })
            }
            CharacterSelectionEventArgs::Confirm => Ok(CharacterSelectionEvent::Confirm),
        }
    }
}
