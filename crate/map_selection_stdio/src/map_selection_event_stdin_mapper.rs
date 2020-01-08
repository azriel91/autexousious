use std::str::FromStr;

use amethyst::{
    ecs::{Read, World},
    shred::{ResourceId, SystemData},
    Error,
};
use asset_model::{
    config::{AssetSlug, AssetType},
    loaded::{AssetIdMappings, AssetTypeMappings},
};
use derivative::Derivative;
use map_selection_model::{MapSelection, MapSelectionEvent, MapSelectionEventArgs};
use stdio_spi::{MapperSystemData, StdinMapper, StdioError};

#[derive(Debug)]
pub struct MapSelectionEventStdinMapperData;

/// `MapSelectionEventStdinMapperSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct MapSelectionEventStdinMapperSystemData<'s> {
    /// `AssetTypeMappings` resource.
    #[derivative(Debug = "ignore")]
    pub asset_type_mappings: Read<'s, AssetTypeMappings>,
    /// `AssetIdMappings` resource.
    #[derivative(Debug = "ignore")]
    pub asset_id_mappings: Read<'s, AssetIdMappings>,
}

impl<'s> MapperSystemData<'s> for MapSelectionEventStdinMapperData {
    type SystemData = MapSelectionEventStdinMapperSystemData<'s>;
}

/// Builds a `MapSelectionEvent` from stdin tokens.
#[derive(Debug)]
pub struct MapSelectionEventStdinMapper;

impl MapSelectionEventStdinMapper {
    fn map_select_event(
        MapSelectionEventStdinMapperSystemData {
            asset_type_mappings,
            asset_id_mappings,
        }: &MapSelectionEventStdinMapperSystemData<'_>,
        selection: &str,
    ) -> Result<MapSelectionEvent, Error> {
        let map_selection = match selection {
            "random" => {
                let map_asset_id = asset_type_mappings
                    .iter_ids(&AssetType::Map)
                    .next()
                    .expect("Expected at least one map to be loaded.");
                MapSelection::Random(Some(*map_asset_id))
            }
            slug_str => {
                let slug = AssetSlug::from_str(slug_str)
                    .map_err(String::from)
                    .map_err(StdioError::Msg)?;
                let map_asset_id = asset_id_mappings
                    .id(&slug)
                    .copied()
                    .ok_or_else(|| format!("No map found with asset slug `{}`.", slug))
                    .map_err(StdioError::Msg)?;

                MapSelection::Id(map_asset_id)
            }
        };

        let map_selection_event = MapSelectionEvent::Select { map_selection };

        Ok(map_selection_event)
    }
}

impl StdinMapper for MapSelectionEventStdinMapper {
    type SystemData = MapSelectionEventStdinMapperData;
    type Event = MapSelectionEvent;
    type Args = MapSelectionEventArgs;

    fn map(
        map_selection_event_stdin_mapper_system_data: &MapSelectionEventStdinMapperSystemData<'_>,
        args: Self::Args,
    ) -> Result<Self::Event, Error> {
        match args {
            MapSelectionEventArgs::Return => Ok(MapSelectionEvent::Return),
            MapSelectionEventArgs::Select { selection } => {
                Self::map_select_event(map_selection_event_stdin_mapper_system_data, &selection)
            }
            MapSelectionEventArgs::Deselect => Ok(MapSelectionEvent::Deselect),
            MapSelectionEventArgs::Confirm => Ok(MapSelectionEvent::Confirm),
        } // kcov-ignore
    }
}
