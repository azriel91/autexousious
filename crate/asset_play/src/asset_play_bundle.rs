use amethyst::{
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, World, WorldExt},
    Error,
};
use asset_model::play::AssetWorld;
use audio_model::loaded::SourceSequenceHandles;
use character_model::loaded::CharacterCtsHandles;
use collision_model::loaded::{BodySequenceHandles, InteractionsSequenceHandles};
use derive_new::new;
use game_mode_selection_model::GameModeIndex;
use kinematic_model::{config::PositionInit, loaded::ObjectAccelerationSequenceHandles};
use sequence_model::loaded::{SequenceEndTransitions, WaitSequenceHandles};
use spawn_model::loaded::SpawnsSequenceHandles;
use sprite_model::loaded::{
    ScaleSequenceHandles, SpriteRenderSequenceHandles, TintSequenceHandles,
};
use typename::TypeName;
use ui_label_model::{config::UiLabel, loaded::UiSpriteLabel};
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
        asset_world.register::<SequenceEndTransitions>();
        asset_world.register::<WaitSequenceHandles>();
        asset_world.register::<SourceSequenceHandles>();
        asset_world.register::<ObjectAccelerationSequenceHandles>();
        asset_world.register::<SpriteRenderSequenceHandles>();
        asset_world.register::<BodySequenceHandles>();
        asset_world.register::<InteractionsSequenceHandles>();
        asset_world.register::<SpawnsSequenceHandles>();
        asset_world.register::<CharacterCtsHandles>();
        asset_world.register::<PositionInit>();
        asset_world.register::<TintSequenceHandles>();
        asset_world.register::<ScaleSequenceHandles>();
        asset_world.register::<UiLabel>();
        asset_world.register::<UiSpriteLabel>();
        asset_world.register::<UiMenuItem<GameModeIndex>>();

        world.insert(asset_world);

        builder.add_barrier();
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
            ItemComponentComponentAugmentSystem::<CharacterCtsHandles>::new(),
            &ItemComponentComponentAugmentSystem::<CharacterCtsHandles>::type_name(),
            &[],
        );
        builder.add(
            ItemComponentComponentAugmentSystem::<PositionInit>::new(),
            &ItemComponentComponentAugmentSystem::<PositionInit>::type_name(),
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
            ItemComponentComponentAugmentSystem::<UiLabel>::new(),
            &ItemComponentComponentAugmentSystem::<UiLabel>::type_name(),
            &[],
        );
        builder.add(
            ItemComponentComponentAugmentSystem::<UiSpriteLabel>::new(),
            &ItemComponentComponentAugmentSystem::<UiSpriteLabel>::type_name(),
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
