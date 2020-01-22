use std::any;

use amethyst::{
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, World, WorldExt},
    Error,
};
use asset_model::{config::asset_type::Map, play::AssetWorld};
use asset_selection_ui_model::{loaded::AssetPreviewWidget, play::ApwMain};
use asset_ui_model::{
    loaded::{
        AssetDisplayCellCharacter, AssetDisplayCellMap, AssetSelectionCell,
        AssetSelectionHighlight, AssetSelector,
    },
    play::{AssetSelectionHighlightMain, AssetSelectionStatus},
};
use audio_model::loaded::SourceSequenceHandles;
use character_model::loaded::CharacterIrsHandles;
use character_selection_ui_model::loaded::CswPortraits;
use chase_model::play::ChaseModeStick;
use collision_model::loaded::{BodySequenceHandles, InteractionsSequenceHandles};
use derive_new::new;
use game_input::{ButtonInputControlled, InputControlled, SharedInputControlled};
use game_mode_selection_model::GameModeIndex;
use input_reaction_model::loaded::InputReactionsSequenceHandles;
use kinematic_model::{
    config::{PositionInit, ScaleInit, VelocityInit},
    loaded::ObjectAccelerationSequenceHandles,
    play::PositionZAsY,
};
use map_selection_ui_model::loaded::MswPortraits;
use mirrored_model::play::Mirrored;
use object_model::play::Grounding;
use object_type::Character;
use sequence_model::loaded::{SequenceEndTransitions, SequenceId, WaitSequenceHandles};
use spawn_model::loaded::SpawnsSequenceHandles;
use sprite_model::loaded::{
    ScaleSequenceHandles, SpriteRenderSequenceHandles, TintSequenceHandles,
};
use ui_label_model::config::UiLabel;
use ui_menu_item_model::loaded::UiMenuItem;

use crate::ItemComponentComponentAugmentSystem;

/// Adds the following `System`s to the `World`:
///
/// * `Processor<BackgroundDefinition>`
#[derive(Debug, new)]
pub struct AssetPlayBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for AssetPlayBundle {
    fn build(
        self,
        world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        let mut asset_world = AssetWorld::default();
        asset_world.register::<InputControlled>();
        asset_world.register::<SharedInputControlled>();
        asset_world.register::<ButtonInputControlled>();
        asset_world.register::<PositionInit>();
        asset_world.register::<VelocityInit>();
        asset_world.register::<ScaleInit>();
        asset_world.register::<PositionZAsY>();
        asset_world.register::<Mirrored>();
        asset_world.register::<Grounding>();
        asset_world.register::<SequenceId>();
        asset_world.register::<SequenceEndTransitions>();
        asset_world.register::<WaitSequenceHandles>();
        asset_world.register::<SourceSequenceHandles>();
        asset_world.register::<ObjectAccelerationSequenceHandles>();
        asset_world.register::<SpriteRenderSequenceHandles>();
        asset_world.register::<BodySequenceHandles>();
        asset_world.register::<InteractionsSequenceHandles>();
        asset_world.register::<SpawnsSequenceHandles>();
        asset_world.register::<TintSequenceHandles>();
        asset_world.register::<ScaleSequenceHandles>();
        asset_world.register::<CharacterIrsHandles>();
        asset_world.register::<InputReactionsSequenceHandles>();
        asset_world.register::<UiLabel>();
        asset_world.register::<UiMenuItem<GameModeIndex>>();

        asset_world.register::<AssetPreviewWidget>();
        asset_world.register::<ApwMain>();

        asset_world.register::<CswPortraits>();
        asset_world.register::<AssetSelector<Character>>();
        asset_world.register::<AssetDisplayCellCharacter>();
        asset_world.register::<AssetSelectionCell<AssetDisplayCellCharacter>>();

        asset_world.register::<MswPortraits>();
        asset_world.register::<AssetSelector<Map>>();
        asset_world.register::<AssetDisplayCellMap>();
        asset_world.register::<AssetSelectionCell<AssetDisplayCellMap>>();

        asset_world.register::<AssetSelectionStatus>();
        asset_world.register::<AssetSelectionHighlight>();
        asset_world.register::<AssetSelectionHighlightMain>();
        asset_world.register::<ChaseModeStick>();

        world.insert(asset_world);

        builder.add_barrier();
        builder.add(
            ItemComponentComponentAugmentSystem::<InputControlled>::new(),
            &any::type_name::<ItemComponentComponentAugmentSystem<InputControlled>>(),
            &[],
        );
        builder.add(
            ItemComponentComponentAugmentSystem::<SharedInputControlled>::new(),
            any::type_name::<ItemComponentComponentAugmentSystem<SharedInputControlled>>(),
            &[],
        );
        builder.add(
            ItemComponentComponentAugmentSystem::<ButtonInputControlled>::new(),
            &any::type_name::<ItemComponentComponentAugmentSystem<ButtonInputControlled>>(),
            &[],
        );
        builder.add(
            ItemComponentComponentAugmentSystem::<PositionInit>::new(),
            any::type_name::<ItemComponentComponentAugmentSystem<PositionInit>>(),
            &[],
        );
        builder.add(
            ItemComponentComponentAugmentSystem::<VelocityInit>::new(),
            any::type_name::<ItemComponentComponentAugmentSystem<VelocityInit>>(),
            &[],
        );
        // Must strictly come after `PositionInit` as it modifies the `Transform` scale.
        builder.add(
            ItemComponentComponentAugmentSystem::<ScaleInit>::new(),
            any::type_name::<ItemComponentComponentAugmentSystem<ScaleInit>>(),
            &[any::type_name::<
                ItemComponentComponentAugmentSystem<PositionInit>,
            >()],
        );
        builder.add(
            ItemComponentComponentAugmentSystem::<PositionZAsY>::new(),
            any::type_name::<ItemComponentComponentAugmentSystem<PositionZAsY>>(),
            &[],
        );
        builder.add(
            ItemComponentComponentAugmentSystem::<Mirrored>::new(),
            any::type_name::<ItemComponentComponentAugmentSystem<Mirrored>>(),
            &[],
        );
        builder.add(
            ItemComponentComponentAugmentSystem::<Grounding>::new(),
            any::type_name::<ItemComponentComponentAugmentSystem<Grounding>>(),
            &[],
        );
        builder.add(
            ItemComponentComponentAugmentSystem::<SequenceId>::new(),
            any::type_name::<ItemComponentComponentAugmentSystem<SequenceId>>(),
            &[],
        );
        builder.add(
            ItemComponentComponentAugmentSystem::<SequenceEndTransitions>::new(),
            any::type_name::<ItemComponentComponentAugmentSystem<SequenceEndTransitions>>(),
            &[],
        );
        builder.add(
            ItemComponentComponentAugmentSystem::<WaitSequenceHandles>::new(),
            any::type_name::<ItemComponentComponentAugmentSystem<WaitSequenceHandles>>(),
            &[],
        );
        builder.add(
            ItemComponentComponentAugmentSystem::<SourceSequenceHandles>::new(),
            any::type_name::<ItemComponentComponentAugmentSystem<SourceSequenceHandles>>(),
            &[],
        );
        builder.add(
            ItemComponentComponentAugmentSystem::<ObjectAccelerationSequenceHandles>::new(),
            any::type_name::<ItemComponentComponentAugmentSystem::<ObjectAccelerationSequenceHandles>>(),
            &[],
        );
        builder.add(
            ItemComponentComponentAugmentSystem::<SpriteRenderSequenceHandles>::new(),
            any::type_name::<ItemComponentComponentAugmentSystem<SpriteRenderSequenceHandles>>(),
            &[],
        );
        builder.add(
            ItemComponentComponentAugmentSystem::<BodySequenceHandles>::new(),
            any::type_name::<ItemComponentComponentAugmentSystem<BodySequenceHandles>>(),
            &[],
        );
        builder.add(
            ItemComponentComponentAugmentSystem::<InteractionsSequenceHandles>::new(),
            any::type_name::<ItemComponentComponentAugmentSystem<InteractionsSequenceHandles>>(),
            &[],
        );
        builder.add(
            ItemComponentComponentAugmentSystem::<SpawnsSequenceHandles>::new(),
            any::type_name::<ItemComponentComponentAugmentSystem<SpawnsSequenceHandles>>(),
            &[],
        );
        builder.add(
            ItemComponentComponentAugmentSystem::<TintSequenceHandles>::new(),
            any::type_name::<ItemComponentComponentAugmentSystem<TintSequenceHandles>>(),
            &[],
        );
        builder.add(
            ItemComponentComponentAugmentSystem::<ScaleSequenceHandles>::new(),
            any::type_name::<ItemComponentComponentAugmentSystem<ScaleSequenceHandles>>(),
            &[],
        );
        builder.add(
            ItemComponentComponentAugmentSystem::<CharacterIrsHandles>::new(),
            any::type_name::<ItemComponentComponentAugmentSystem<CharacterIrsHandles>>(),
            &[],
        );
        builder.add(
            ItemComponentComponentAugmentSystem::<InputReactionsSequenceHandles>::new(),
            any::type_name::<ItemComponentComponentAugmentSystem<InputReactionsSequenceHandles>>(),
            &[],
        );
        builder.add(
            ItemComponentComponentAugmentSystem::<UiLabel>::new(),
            any::type_name::<ItemComponentComponentAugmentSystem<UiLabel>>(),
            &[],
        );
        builder.add(
            ItemComponentComponentAugmentSystem::<UiMenuItem<GameModeIndex>>::new(),
            any::type_name::<ItemComponentComponentAugmentSystem<UiMenuItem<GameModeIndex>>>(),
            &[],
        );

        // Asset selection UI common systems.
        builder.add(
            ItemComponentComponentAugmentSystem::<AssetPreviewWidget>::new(),
            &any::type_name::<ItemComponentComponentAugmentSystem<AssetPreviewWidget>>(),
            &[],
        );
        builder.add(
            ItemComponentComponentAugmentSystem::<ApwMain>::new(),
            &any::type_name::<ItemComponentComponentAugmentSystem<ApwMain>>(),
            &[&any::type_name::<
                ItemComponentComponentAugmentSystem<AssetPreviewWidget>,
            >()],
        );

        // Character Selection UI
        builder.add(
            ItemComponentComponentAugmentSystem::<CswPortraits>::new(),
            &any::type_name::<ItemComponentComponentAugmentSystem<CswPortraits>>(),
            &[&any::type_name::<
                ItemComponentComponentAugmentSystem<AssetPreviewWidget>,
            >()],
        );
        builder.add(
            ItemComponentComponentAugmentSystem::<AssetSelector<Character>>::new(),
            &any::type_name::<ItemComponentComponentAugmentSystem<AssetSelector<Character>>>(),
            &[],
        );
        builder.add(
            ItemComponentComponentAugmentSystem::<AssetDisplayCellCharacter>::new(),
            &any::type_name::<ItemComponentComponentAugmentSystem<AssetDisplayCellCharacter>>(),
            &[any::type_name::<
                ItemComponentComponentAugmentSystem<AssetSelector<Character>>,
            >()],
        );
        builder.add(
            ItemComponentComponentAugmentSystem::<AssetSelectionCell<AssetDisplayCellCharacter>>::new(),
            &any::type_name::<ItemComponentComponentAugmentSystem<AssetSelectionCell<AssetDisplayCellCharacter>>>(),
            &[any::type_name::<
                ItemComponentComponentAugmentSystem<AssetSelector<Character>>,
            >()],
        );

        // Map Selection UI
        builder.add(
            ItemComponentComponentAugmentSystem::<MswPortraits>::new(),
            &any::type_name::<ItemComponentComponentAugmentSystem<MswPortraits>>(),
            &[&any::type_name::<
                ItemComponentComponentAugmentSystem<AssetPreviewWidget>,
            >()],
        );
        builder.add(
            ItemComponentComponentAugmentSystem::<AssetSelector<Map>>::new(),
            &any::type_name::<ItemComponentComponentAugmentSystem<AssetSelector<Map>>>(),
            &[],
        );
        builder.add(
            ItemComponentComponentAugmentSystem::<AssetDisplayCellMap>::new(),
            &any::type_name::<ItemComponentComponentAugmentSystem<AssetDisplayCellMap>>(),
            &[any::type_name::<
                ItemComponentComponentAugmentSystem<AssetSelector<Map>>,
            >()],
        );
        builder.add(
            ItemComponentComponentAugmentSystem::<AssetSelectionCell<AssetDisplayCellMap>>::new(),
            &any::type_name::<
                ItemComponentComponentAugmentSystem<AssetSelectionCell<AssetDisplayCellMap>>,
            >(),
            &[any::type_name::<
                ItemComponentComponentAugmentSystem<AssetSelector<Map>>,
            >()],
        );

        // Common Asset selection systems
        builder.add(
            ItemComponentComponentAugmentSystem::<AssetSelectionStatus>::new(),
            &any::type_name::<ItemComponentComponentAugmentSystem<AssetSelectionStatus>>(),
            &[
                any::type_name::<ItemComponentComponentAugmentSystem<AssetSelector<Character>>>(),
                any::type_name::<ItemComponentComponentAugmentSystem<AssetSelector<Map>>>(),
            ],
        );
        builder.add(
            ItemComponentComponentAugmentSystem::<AssetSelectionHighlight>::new(),
            &any::type_name::<ItemComponentComponentAugmentSystem<AssetSelectionHighlight>>(),
            &[
                any::type_name::<ItemComponentComponentAugmentSystem<AssetSelector<Character>>>(),
                any::type_name::<ItemComponentComponentAugmentSystem<AssetSelector<Map>>>(),
            ],
        );
        builder.add(
            ItemComponentComponentAugmentSystem::<AssetSelectionHighlightMain>::new(),
            &any::type_name::<ItemComponentComponentAugmentSystem<AssetSelectionHighlightMain>>(),
            &[],
        );
        builder.add(
            ItemComponentComponentAugmentSystem::<ChaseModeStick>::new(),
            &any::type_name::<ItemComponentComponentAugmentSystem<ChaseModeStick>>(),
            &[any::type_name::<
                ItemComponentComponentAugmentSystem<AssetSelectionHighlight>,
            >()],
        );
        builder.add_barrier();
        Ok(())
    }
}
