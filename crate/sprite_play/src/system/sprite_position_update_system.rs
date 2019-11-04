use amethyst::{
    ecs::{Entities, Join, ReadStorage, System, World, WriteStorage},
    shred::{ResourceId, SystemData},
};
use derivative::Derivative;
use derive_new::new;
use kinematic_model::config::Position;
use sprite_model::config::SpritePosition;
use typename_derive::TypeName;

/// Updates the `Position<f32>` of entities that have a `SpritePosition`.
#[derive(Debug, Default, TypeName, new)]
pub struct SpritePositionUpdateSystem;

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct SpritePositionUpdateSystemData<'s> {
    /// `Entities` resource.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// `SpritePosition` components.
    #[derivative(Debug = "ignore")]
    pub sprite_positions: ReadStorage<'s, SpritePosition>,
    /// `Position<f32>` components.
    #[derivative(Debug = "ignore")]
    pub positions: WriteStorage<'s, Position<f32>>,
}

impl<'s> System<'s> for SpritePositionUpdateSystem {
    type SystemData = SpritePositionUpdateSystemData<'s>;

    fn run(
        &mut self,
        SpritePositionUpdateSystemData {
            entities,
            sprite_positions,
            mut positions,
        }: Self::SystemData,
    ) {
        (&entities, &sprite_positions)
            .join()
            .for_each(|(entity, sprite_position)| {
                let x = sprite_position.x as f32;
                let y = sprite_position.y as f32;
                let z = sprite_position.z as f32;
                positions
                    .insert(entity, Position::new(x, y, z))
                    .expect("Failed to insert `Position<f32>` component.");
            });
    } // kcov-ignore
}
