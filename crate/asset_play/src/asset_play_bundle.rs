use amethyst::{
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, World, WorldExt},
    Error,
};
use asset_model::play::AssetWorld;
use audio_model::loaded::SourceSequenceHandles;
use character_model::loaded::CharacterIrsHandles;
use collision_model::loaded::{BodySequenceHandles, InteractionsSequenceHandles};
use derive_new::new;
use game_input::SharedInputControlled;
use game_mode_selection_model::GameModeIndex;
use input_reaction_model::loaded::InputReactionsSequenceHandles;
use kinematic_model::{
    config::{PositionInit, VelocityInit},
    loaded::ObjectAccelerationSequenceHandles,
    play::PositionZAsY,
};
use object_model::play::{Grounding, Mirrored};
use sequence_model::loaded::{SequenceEndTransitions, SequenceId, WaitSequenceHandles};
use spawn_model::loaded::SpawnsSequenceHandles;
use sprite_model::loaded::{
    ScaleSequenceHandles, SpriteRenderSequenceHandles, TintSequenceHandles,
};
use typename::TypeName;
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
        asset_world.register::<SharedInputControlled>();
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

        world.insert(asset_world);

        builder.add_barrier();
        builder.add(
            ItemComponentComponentAugmentSystem::<SharedInputControlled>::new(),
            &ItemComponentComponentAugmentSystem::<SharedInputControlled>::type_name(),
            &[],
        );
        builder.add(
            ItemComponentComponentAugmentSystem::<PositionInit>::new(),
            &ItemComponentComponentAugmentSystem::<PositionInit>::type_name(),
            &[],
        );
        builder.add(
            ItemComponentComponentAugmentSystem::<VelocityInit>::new(),
            &ItemComponentComponentAugmentSystem::<VelocityInit>::type_name(),
            &[],
        );
        builder.add(
            ItemComponentComponentAugmentSystem::<PositionZAsY>::new(),
            &ItemComponentComponentAugmentSystem::<PositionZAsY>::type_name(),
            &[],
        );
        builder.add(
            ItemComponentComponentAugmentSystem::<Mirrored>::new(),
            &ItemComponentComponentAugmentSystem::<Mirrored>::type_name(),
            &[],
        );
        builder.add(
            ItemComponentComponentAugmentSystem::<Grounding>::new(),
            &ItemComponentComponentAugmentSystem::<Grounding>::type_name(),
            &[],
        );
        builder.add(
            ItemComponentComponentAugmentSystem::<SequenceId>::new(),
            &ItemComponentComponentAugmentSystem::<SequenceId>::type_name(),
            &[],
        );
        builder.add(
            ItemComponentComponentAugmentSystem::<SequenceEndTransitions>::new(),
            &ItemComponentComponentAugmentSystem::<SequenceEndTransitions>::type_name(),
            &[],
        );
        builder.add(
            ItemComponentComponentAugmentSystem::<WaitSequenceHandles>::new(),
            &ItemComponentComponentAugmentSystem::<WaitSequenceHandles>::type_name(),
            &[],
        );
        builder.add(
            ItemComponentComponentAugmentSystem::<SourceSequenceHandles>::new(),
            &ItemComponentComponentAugmentSystem::<SourceSequenceHandles>::type_name(),
            &[],
        );
        builder.add(
            ItemComponentComponentAugmentSystem::<ObjectAccelerationSequenceHandles>::new(),
            &ItemComponentComponentAugmentSystem::<ObjectAccelerationSequenceHandles>::type_name(),
            &[],
        );
        builder.add(
            ItemComponentComponentAugmentSystem::<SpriteRenderSequenceHandles>::new(),
            &ItemComponentComponentAugmentSystem::<SpriteRenderSequenceHandles>::type_name(),
            &[],
        );
        builder.add(
            ItemComponentComponentAugmentSystem::<BodySequenceHandles>::new(),
            &ItemComponentComponentAugmentSystem::<BodySequenceHandles>::type_name(),
            &[],
        );
        builder.add(
            ItemComponentComponentAugmentSystem::<InteractionsSequenceHandles>::new(),
            &ItemComponentComponentAugmentSystem::<InteractionsSequenceHandles>::type_name(),
            &[],
        );
        builder.add(
            ItemComponentComponentAugmentSystem::<SpawnsSequenceHandles>::new(),
            &ItemComponentComponentAugmentSystem::<SpawnsSequenceHandles>::type_name(),
            &[],
        );
        builder.add(
            ItemComponentComponentAugmentSystem::<TintSequenceHandles>::new(),
            &ItemComponentComponentAugmentSystem::<TintSequenceHandles>::type_name(),
            &[],
        );
        builder.add(
            ItemComponentComponentAugmentSystem::<ScaleSequenceHandles>::new(),
            &ItemComponentComponentAugmentSystem::<ScaleSequenceHandles>::type_name(),
            &[],
        );
        builder.add(
            ItemComponentComponentAugmentSystem::<CharacterIrsHandles>::new(),
            &ItemComponentComponentAugmentSystem::<CharacterIrsHandles>::type_name(),
            &[],
        );
        builder.add(
            ItemComponentComponentAugmentSystem::<InputReactionsSequenceHandles>::new(),
            &ItemComponentComponentAugmentSystem::<InputReactionsSequenceHandles>::type_name(),
            &[],
        );
        builder.add(
            ItemComponentComponentAugmentSystem::<UiLabel>::new(),
            &ItemComponentComponentAugmentSystem::<UiLabel>::type_name(),
            &[],
        );
        builder.add(
            ItemComponentComponentAugmentSystem::<UiMenuItem<GameModeIndex>>::new(),
            &ItemComponentComponentAugmentSystem::<UiMenuItem<GameModeIndex>>::type_name(),
            &[],
        );
        builder.add_barrier();
        Ok(())
    }
}
