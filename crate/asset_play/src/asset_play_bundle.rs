use std::any;

use amethyst::{
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, World, WorldExt},
    Error,
};
use asset_model::play::AssetWorld;
use audio_model::loaded::SourceSequenceHandles;
use character_model::loaded::CharacterIrsHandles;
use character_selection_ui_model::{
    loaded::{CharacterSelectionWidget, CswPortraits},
    play::CswMain,
};
use collision_model::loaded::{BodySequenceHandles, InteractionsSequenceHandles};
use derive_new::new;
use game_input::{ButtonInputControlled, InputControlled, SharedInputControlled};
use game_mode_selection_model::GameModeIndex;
use input_reaction_model::loaded::InputReactionsSequenceHandles;
use kinematic_model::{
    config::{PositionInit, VelocityInit},
    loaded::ObjectAccelerationSequenceHandles,
    play::PositionZAsY,
};
use mirrored_model::play::Mirrored;
use object_model::play::Grounding;
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
        asset_world.register::<CharacterSelectionWidget>();
        asset_world.register::<CswPortraits>();
        asset_world.register::<CswMain>();

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
        builder.add(
            ItemComponentComponentAugmentSystem::<CharacterSelectionWidget>::new(),
            &any::type_name::<ItemComponentComponentAugmentSystem<CharacterSelectionWidget>>(),
            &[],
        );
        builder.add(
            ItemComponentComponentAugmentSystem::<CswPortraits>::new(),
            &any::type_name::<ItemComponentComponentAugmentSystem<CswPortraits>>(),
            &[&any::type_name::<
                ItemComponentComponentAugmentSystem<CharacterSelectionWidget>,
            >()],
        );
        builder.add(
            ItemComponentComponentAugmentSystem::<CswMain>::new(),
            &any::type_name::<ItemComponentComponentAugmentSystem<CswMain>>(),
            &[&any::type_name::<
                ItemComponentComponentAugmentSystem<CharacterSelectionWidget>,
            >()],
        );
        builder.add_barrier();
        Ok(())
    }
}
