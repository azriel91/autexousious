use amethyst::{
    animation::get_animation_set,
    assets::AssetStorage,
    ecs::{Read, ReadStorage, Resources, System, SystemData, WriteStorage},
    shrev::{EventChannel, ReaderId},
};
use collision_model::play::CollisionEvent;
use game_loading::{AnimationRunner, ObjectAnimationStorages};
use object_model::{
    config::object::CharacterSequenceId,
    entity::CharacterStatus,
    loaded::{AnimatedComponentAnimation, Character, CharacterHandle},
};
use typename::TypeName;

/// Determines collision effects for characters.
#[derive(Debug, Default, TypeName, new)]
pub(crate) struct CharacterCollisionEffectSystem {
    /// Reader ID for the `CollisionEvent` event channel.
    #[new(default)]
    reader_id: Option<ReaderId<CollisionEvent>>,
}

type CharacterCollisionEffectSystemData<'s> = (
    Read<'s, EventChannel<CollisionEvent>>,
    ReadStorage<'s, CharacterHandle>,
    Read<'s, AssetStorage<Character>>,
    WriteStorage<'s, CharacterStatus>,
    ObjectAnimationStorages<'s, CharacterSequenceId>,
);

impl<'s> System<'s> for CharacterCollisionEffectSystem {
    type SystemData = CharacterCollisionEffectSystemData<'s>;

    fn run(
        &mut self,
        (
            collision_ec,
            character_handles,
            character_assets,
            mut character_statuses,
            (mut sprite_acs, mut body_frame_acs, mut interaction_acs),
        ): Self::SystemData,
    ) {
        // Read from channel
        collision_ec
            .read(
                self.reader_id
                    .as_mut()
                    .expect("Expected reader ID to exist for CharacterCollisionEffectSystem."),
            )
            .for_each(|ev| {
                // Fetch CharacterStatus for entity
                let character_handle = character_handles.get(ev.to);
                let character_status = character_statuses.get_mut(ev.to);

                if let (Some(character_handle), Some(character_status)) =
                    (character_handle, character_status)
                {
                    let character = character_assets
                        .get(character_handle)
                        .expect("Expected character to be loaded.");
                    let mut sprite_animation_set = get_animation_set(&mut sprite_acs, ev.to)
                        .expect("Sprite animation should exist as entity should be valid.");
                    let mut body_animation_set = get_animation_set(&mut body_frame_acs, ev.to)
                        .expect("Body animation should exist as entity should be valid.");
                    let mut interaction_animation_set =
                        get_animation_set(&mut interaction_acs, ev.to).expect(
                            "Interaction animation should exist as entity should be valid.",
                        );

                    // TODO: Select damage sequence based on status.
                    let next_sequence_id = CharacterSequenceId::Jump;

                    // Swap animations
                    let animations = &character
                        .object
                        .animations
                        .get(&next_sequence_id)
                        .unwrap_or_else(|| {
                            panic!(
                                "Failed to get animations for sequence: `{:?}`",
                                next_sequence_id
                            )
                        });

                    animations
                        .iter()
                        .for_each(|animated_component| match animated_component {
                            AnimatedComponentAnimation::SpriteRender(ref handle) => {
                                AnimationRunner::swap(
                                    character_status.object_status.sequence_id,
                                    next_sequence_id,
                                    &mut sprite_animation_set,
                                    handle,
                                );
                            }
                            AnimatedComponentAnimation::BodyFrame(ref handle) => {
                                AnimationRunner::swap(
                                    character_status.object_status.sequence_id,
                                    next_sequence_id,
                                    &mut body_animation_set,
                                    handle,
                                );
                            }
                            AnimatedComponentAnimation::InteractionFrame(ref handle) => {
                                AnimationRunner::swap(
                                    character_status.object_status.sequence_id,
                                    next_sequence_id,
                                    &mut interaction_animation_set,
                                    handle,
                                );
                            }
                        });

                    // Set sequence id
                    character_status.object_status.sequence_id = next_sequence_id;
                }
            });
    }

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        self.reader_id = Some(
            res.fetch_mut::<EventChannel<CollisionEvent>>()
                .register_reader(),
        );
    }
}
