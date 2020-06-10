mod map_ascl;
mod object_ascl;
mod ui_ascl;
mod ui_character_selection_ascl;
mod ui_components_ascl;
mod ui_control_settings_ascl;
mod ui_form_ascl;
mod ui_map_selection_ascl;
mod ui_menu_ascl;
mod ui_session_lobby_ascl;

use amethyst::ecs::WorldExt;
use asset_model::{config::AssetType, loaded::AssetId};
use audio_model::loaded::SourceSequenceHandles;
use character_model::loaded::CharacterIrsHandles;
use collision_model::loaded::{BodySequenceHandles, InteractionsSequenceHandles};
use kinematic_model::loaded::ObjectAccelerationSequenceHandles;
use loading_model::loaded::LoadStage;
use log::debug;
use sequence_model::loaded::WaitSequenceHandles;
use spawn_model::loaded::SpawnsSequenceHandles;
use sprite_model::loaded::{
    ScaleSequenceHandles, SpriteRenderSequenceHandles, TintSequenceHandles,
};

use crate::{
    AssetLoadingResources, AssetPartLoader, AssetPartLoadingSystem,
    SequenceComponentLoadingResources,
};

pub use self::{
    map_ascl::MapAscl, object_ascl::ObjectAscl, ui_ascl::UiAscl,
    ui_character_selection_ascl::UiCharacterSelectionAscl, ui_components_ascl::UiComponentsAscl,
    ui_control_settings_ascl::UiControlSettingsAscl, ui_form_ascl::UiFormAscl,
    ui_map_selection_ascl::UiMapSelectionAscl, ui_menu_ascl::UiMenuAscl,
    ui_session_lobby_ascl::UiSessionLobbyAscl,
};

/// Loads asset sequence components.
pub type AssetSequenceComponentLoadingSystem = AssetPartLoadingSystem<AssetSequenceComponentLoader>;

/// `AssetSequenceComponentLoader`.
#[derive(Debug)]
pub struct AssetSequenceComponentLoader;

impl<'s> AssetPartLoader<'s> for AssetSequenceComponentLoader {
    const LOAD_STAGE: LoadStage = LoadStage::SequenceComponentLoading;
    type SystemData = SequenceComponentLoadingResources<'s>;

    fn process(
        asset_loading_resources: &mut AssetLoadingResources<'_>,
        sequence_component_loading_resources: &mut SequenceComponentLoadingResources<'_>,
        asset_id: AssetId,
    ) {
        let AssetLoadingResources {
            asset_id_mappings,
            asset_type_mappings,
            ..
        } = asset_loading_resources;

        let asset_type = asset_type_mappings
            .get(asset_id)
            .copied()
            .expect("Expected `AssetType` mapping to exist.");

        let asset_slug = asset_id_mappings
            .slug(asset_id)
            .expect("Expected `AssetSlug` mapping to exist for `AssetId`.");

        debug!("Loading `{}` sequence components.", asset_slug);

        match asset_type {
            AssetType::Object(object_type) => {
                ObjectAscl::load(
                    asset_loading_resources,
                    sequence_component_loading_resources,
                    asset_id,
                    object_type,
                );
            }
            AssetType::Map => {
                MapAscl::load(
                    asset_loading_resources,
                    sequence_component_loading_resources,
                    asset_id,
                );
            }
            AssetType::Ui => {
                UiAscl::load(
                    asset_loading_resources,
                    sequence_component_loading_resources,
                    asset_id,
                );
            }
        }
    }

    /// Returns whether sequence components assets have been loaded.
    fn is_complete(
        _: &AssetLoadingResources<'_>,
        SequenceComponentLoadingResources {
            asset_world,
            asset_item_ids,
            wait_sequence_assets,
            source_sequence_assets,
            object_acceleration_sequence_assets,
            sprite_render_sequence_assets,
            body_sequence_assets,
            interactions_sequence_assets,
            spawns_sequence_assets,
            character_irs_assets,
            tint_sequence_assets,
            scale_sequence_assets,
            ..
        }: &SequenceComponentLoadingResources<'_>,
        asset_id: AssetId,
    ) -> bool {
        macro_rules! sequence_component_loaded {
            ($item_component:path, $assets:ident) => {{
                if let Some(item_ids) = asset_item_ids.get(asset_id) {
                    item_ids
                        .iter()
                        .copied()
                        .try_fold((), |_, item_id| {
                            let handleses = asset_world.read_storage::<$item_component>();
                            if let Some(handles) = handleses.get(item_id.0) {
                                if handles.iter().all(|handle| $assets.get(handle).is_some()) {
                                    Ok(())
                                } else {
                                    Err(())
                                }
                            } else {
                                Ok(())
                            }
                        })
                        .is_ok()
                } else {
                    true
                }
            }};
        };

        // sequence_component_loaded!(PositionInit) &&
        // sequence_component_loaded!(VelocityInit) &&
        // sequence_component_loaded!(PositionZAsY) &&
        // sequence_component_loaded!(Mirrored) &&
        // sequence_component_loaded!(Grounding) &&
        // sequence_component_loaded!(SequenceId) &&
        // sequence_component_loaded!(SequenceEndTransitions) &&
        // sequence_component_loaded!(UiLabel) &&
        // sequence_component_loaded!(UiMenuItem<MenuIndex>)

        sequence_component_loaded!(WaitSequenceHandles, wait_sequence_assets)
            && sequence_component_loaded!(SourceSequenceHandles, source_sequence_assets)
            && sequence_component_loaded!(
                ObjectAccelerationSequenceHandles,
                object_acceleration_sequence_assets
            )
            && sequence_component_loaded!(
                SpriteRenderSequenceHandles,
                sprite_render_sequence_assets
            )
            && sequence_component_loaded!(BodySequenceHandles, body_sequence_assets)
            && sequence_component_loaded!(InteractionsSequenceHandles, interactions_sequence_assets)
            && sequence_component_loaded!(SpawnsSequenceHandles, spawns_sequence_assets)
            && sequence_component_loaded!(CharacterIrsHandles, character_irs_assets)
            && sequence_component_loaded!(TintSequenceHandles, tint_sequence_assets)
            && sequence_component_loaded!(ScaleSequenceHandles, scale_sequence_assets)
    }
}
