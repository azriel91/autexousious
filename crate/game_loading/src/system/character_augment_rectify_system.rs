use amethyst::{
    assets::PrefabData,
    ecs::{
        Entities, Entity, Join, LazyUpdate, Read, ReadStorage, System, World, Write, WriteStorage,
    },
    shred::{ResourceId, SystemData},
};
use camera_model::play::CameraTracked;
use derivative::Derivative;
use derive_new::new;
use game_input::InputControlled;
use game_play_hud::{CpBarPrefab, HpBarPrefab};
use game_play_model::GamePlayEntity;
use kinematic_model::config::Position;
use map_model::loaded::AssetMapBounds;
use map_selection_model::MapSelection;

use crate::{CharacterAugmentStatus, GameLoadingStatus};

/// Spawns character entities based on the character selection.
#[derive(Debug, Default, new)]
pub struct CharacterAugmentRectifySystem;

/// `CharacterAugmentRectifySystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct CharacterAugmentRectifySystemData<'s> {
    /// `Entities` resource.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// `GameLoadingStatus` resource.
    #[derivative(Debug = "ignore")]
    pub game_loading_status: Write<'s, GameLoadingStatus>,
    /// `MapSelection` resource.
    #[derivative(Debug = "ignore")]
    pub map_selection: Read<'s, MapSelection>,
    /// `AssetMapBounds` resource.
    #[derivative(Debug = "ignore")]
    pub asset_map_bounds: Read<'s, AssetMapBounds>,
    /// `InputControlled` components.
    #[derivative(Debug = "ignore")]
    pub input_controlleds: ReadStorage<'s, InputControlled>,
    /// `CameraTracked` components.
    #[derivative(Debug = "ignore")]
    pub camera_trackeds: WriteStorage<'s, CameraTracked>,
    /// `Position<f32>` components.
    #[derivative(Debug = "ignore")]
    pub positions: WriteStorage<'s, Position<f32>>,
    /// `LazyUpdate` resource.
    ///
    /// This is used because both the `HpBarPrefab` and `CpBarPrefab` request `Write` access to the
    /// same resources.
    #[derivative(Debug = "ignore")]
    pub lazy_update: Read<'s, LazyUpdate>,
}

impl CharacterAugmentRectifySystem {
    fn hp_bar_augment(world: &World, game_object_entity: Entity) {
        let (entities, mut hp_bar_prefab_system_data, mut game_play_entities) = world
            .system_data::<(
                Entities<'_>,
                <HpBarPrefab as PrefabData<'_>>::SystemData,
                WriteStorage<'_, GamePlayEntity>,
            )>();

        let hp_bar_entity = entities.create();
        let hp_bar_prefab = HpBarPrefab::new(game_object_entity);
        hp_bar_prefab
            .add_to_entity(hp_bar_entity, &mut hp_bar_prefab_system_data, &[], &[])
            .expect("`HpBarPrefab` failed to augment entity.");
        game_play_entities
            .insert(hp_bar_entity, GamePlayEntity)
            .expect("Failed to insert `GamePlayEntity` component.");
    }

    fn cp_bar_augment(world: &World, game_object_entity: Entity) {
        let (entities, mut cp_bar_prefab_system_data, mut game_play_entities) = world
            .system_data::<(
                Entities<'_>,
                <CpBarPrefab as PrefabData<'_>>::SystemData,
                WriteStorage<'_, GamePlayEntity>,
            )>();

        let cp_bar_entity = entities.create();
        let cp_bar_prefab = CpBarPrefab::new(game_object_entity);
        cp_bar_prefab
            .add_to_entity(cp_bar_entity, &mut cp_bar_prefab_system_data, &[], &[])
            .expect("`CpBarPrefab` failed to augment entity.");
        game_play_entities
            .insert(cp_bar_entity, GamePlayEntity)
            .expect("Failed to insert `GamePlayEntity` component.");
    }
}

impl<'s> System<'s> for CharacterAugmentRectifySystem {
    type SystemData = CharacterAugmentRectifySystemData<'s>;

    fn run(
        &mut self,
        CharacterAugmentRectifySystemData {
            entities,
            mut game_loading_status,
            map_selection,
            asset_map_bounds,
            input_controlleds,
            mut camera_trackeds,
            mut positions,
            lazy_update,
        }: Self::SystemData,
    ) {
        // TODO: Entities may not have health_points component -- see the second join()

        // TODO: We may actually want this system to run during gameplay, e.g. when changing which
        // game object is controlled.

        if game_loading_status.character_augment_status != CharacterAugmentStatus::Rectify
            || map_selection.asset_id().is_none()
        {
            return;
        }

        // Read map to determine bounds where the characters can be spawned.
        let (width, height, depth) = {
            asset_map_bounds
                .get(
                    map_selection
                        .asset_id()
                        .expect("Expected map selection to have an `AssetId`."),
                )
                .map(|bounds| {
                    (
                        bounds.width as f32,
                        bounds.height as f32,
                        bounds.depth as f32,
                    )
                })
                .expect("Expected map selection to have `MapBounds`.")
        };

        // This `Position` moves the entity to the middle of a screen wide map.
        let position = Position::<f32>::new(width / 2., height / 2., depth / 2.);

        (&entities, &input_controlleds)
            .join()
            .for_each(|(entity, _)| {
                // Set character `position` based on the map.
                positions
                    .insert(entity, position)
                    .expect("Failed to insert `Position<f32>` component.");

                // Track player with camera.
                camera_trackeds
                    .insert(entity, CameraTracked)
                    .expect("Failed to insert `CameraTracked` component.");

                lazy_update.exec(move |world| Self::hp_bar_augment(world, entity));
                lazy_update.exec(move |world| Self::cp_bar_augment(world, entity));
            });

        game_loading_status.character_augment_status = CharacterAugmentStatus::Complete;
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        <HpBarPrefab as PrefabData<'_>>::SystemData::setup(world);
        <CpBarPrefab as PrefabData<'_>>::SystemData::setup(world);
        <WriteStorage<'_, GamePlayEntity>>::setup(world);
    }
}
