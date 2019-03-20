use amethyst::{
    assets::{AssetStorage, Handle},
    core::{math::Vector3, transform::Transform},
    ecs::{Entities, Join, Read, ReadStorage, System, Write},
    renderer::{Flipped, SpriteRender, SpriteSheet},
    shrev::EventChannel,
};
use collision_model::{
    config::{Body, Interaction, Interactions},
    play::CollisionEvent,
};
use derive_new::new;
use log::debug;
use shape_model::Volume;
use typename_derive::TypeName;

/// Detects collisions for all objects.
#[derive(Debug, Default, TypeName, new)]
pub(crate) struct ObjectCollisionDetectionSystem;

type ObjectCollisionDetectionSystemData<'s> = (
    Entities<'s>,
    ReadStorage<'s, Transform>,
    ReadStorage<'s, Handle<Interactions>>,
    Read<'s, AssetStorage<Interactions>>,
    ReadStorage<'s, Handle<Body>>,
    Read<'s, AssetStorage<Body>>,
    ReadStorage<'s, SpriteRender>,
    ReadStorage<'s, Flipped>,
    Read<'s, AssetStorage<SpriteSheet>>,
    Write<'s, EventChannel<CollisionEvent>>,
);

impl ObjectCollisionDetectionSystem {
    fn intersects(
        relative_pos: &Vector3<f32>,
        (interaction, interaction_offsets, interaction_mirrored): (&Interaction, [f32; 2], bool),
        (body, body_offsets, body_mirrored): (&Volume, [f32; 2], bool),
    ) -> bool {
        // TODO: Use a collision library.
        let Interaction::Physical { bounds, .. } = interaction;

        bounds.iter().any(|bound| match (*bound, *body) {
            (
                Volume::Box {
                    x: i_x,
                    y: i_y,
                    z: i_z,
                    w: i_w,
                    h: i_h,
                    d: i_d,
                },
                Volume::Box {
                    x: b_x,
                    y: b_y,
                    z: b_z,
                    w: b_w,
                    h: b_h,
                    d: b_d,
                },
            ) => {
                let (interaction_x, interaction_x_w) = Self::bound_coordinates(
                    i_x,
                    i_w,
                    interaction_offsets[0],
                    interaction_mirrored,
                    None,
                );
                let (body_x, body_x_w) = Self::bound_coordinates(
                    b_x,
                    b_w,
                    body_offsets[0],
                    body_mirrored,
                    Some(relative_pos[0]),
                );
                let x_intersects = (interaction_x >= body_x && interaction_x < body_x_w)
                    || (interaction_x_w >= body_x && interaction_x_w < body_x_w);

                let (interaction_y, interaction_y_h) =
                    Self::bound_coordinates(i_y, i_h, interaction_offsets[1], false, None);
                let (body_y, body_y_h) = Self::bound_coordinates(
                    b_y,
                    b_h,
                    body_offsets[1],
                    false,
                    Some(relative_pos[1]),
                );
                let y_intersects = (interaction_y >= body_y && interaction_y < body_y_h)
                    || (interaction_y_h >= body_y && interaction_y_h < body_y_h);

                let (interaction_z, interaction_z_d) =
                    Self::bound_coordinates(i_z, i_d, 0., false, None);
                let (body_z, body_z_d) =
                    Self::bound_coordinates(b_z, b_d, 0., false, Some(relative_pos[2]));
                let z_intersects = (interaction_z >= body_z && interaction_z < body_z_d)
                    || (interaction_z_d >= body_z && interaction_z_d < body_z_d);

                x_intersects && y_intersects && z_intersects
            }

            // Explicitly fail because we haven't finished this.
            _ => unimplemented!(),
        })
    }

    fn bound_coordinates(
        i_x: i32,
        i_w: u32,
        offset: f32,
        mirrored: bool,
        relative_pos: Option<f32>,
    ) -> (i32, i32) {
        let (mut coord, mut coord_w) = {
            let mut x1 = i_x as f32 - offset;
            if mirrored {
                let x2 = x1 + i_w as f32;
                x1 = -x1;
                (-x2, x1)
            } else {
                let x2 = x1 + i_w as f32;
                (x1, x2)
            }
        };
        if let Some(relative_pos) = relative_pos {
            coord += relative_pos;
            coord_w += relative_pos;
        }

        (coord as i32, coord_w as i32)
    }
}

impl<'s> System<'s> for ObjectCollisionDetectionSystem {
    type SystemData = ObjectCollisionDetectionSystemData<'s>;

    fn run(
        &mut self,
        (
            entities,
            transforms,
            interactions_handles,
            interactions_assets,
            body_handles,
            body_assets,
            sprite_renders,
            flippeds,
            sprite_sheet_assets,
            mut collision_ec,
        ): Self::SystemData,
    ) {
        // Naive collision detection.
        // TODO: Use broad sweep + narrow sweep for optimization.
        for (from, from_transform, interactions_handle, from_sprite_render, from_flipped) in (
            &entities,
            &transforms,
            &interactions_handles,
            &sprite_renders,
            &flippeds,
        )
            .join()
        {
            for (to, to_transform, body_handle, to_sprite_render, to_flipped) in (
                &entities,
                &transforms,
                &body_handles,
                &sprite_renders,
                &flippeds,
            )
                .join()
            {
                if from == to {
                    // Skip self
                    continue;
                }

                let interaction_offsets = {
                    let sprite_sheet = sprite_sheet_assets
                        .get(&from_sprite_render.sprite_sheet)
                        .expect("Expected sprite sheet for from_sprite_render to exist.");
                    let sprite = &sprite_sheet.sprites[from_sprite_render.sprite_number];

                    // Account for half width and height shift from Amethyst
                    [
                        sprite.offsets[0] + sprite.width / 2.,
                        sprite.offsets[1] + sprite.height / 2.,
                    ]
                };

                let body_offsets = {
                    let sprite_sheet = sprite_sheet_assets
                        .get(&to_sprite_render.sprite_sheet)
                        .expect("Expected sprite sheet for to_sprite_render to exist.");
                    let sprite = &sprite_sheet.sprites[to_sprite_render.sprite_number];

                    // Account for half width and height shift from Amethyst
                    [
                        sprite.offsets[0] + sprite.width / 2.,
                        sprite.offsets[1] + sprite.height / 2.,
                    ]
                };

                let mut relative_pos = to_transform.translation() - from_transform.translation();
                // Undo the Z shift from both entities, see `ObjectTransformUpdateSystem`
                relative_pos[1] += to_transform.translation()[2] - from_transform.translation()[2];

                let interactions = interactions_assets
                    .get(interactions_handle)
                    .expect("Expected `Interactions` from handle to exist.");
                let body = body_assets
                    .get(body_handle)
                    .expect("Expected `Body` from handle to exist.");

                let mut collision_events = {
                    interactions
                        .iter()
                        .flat_map(|interaction| {
                            // loop through each body, if it hits, generate a collision event.

                            body.iter().filter_map(move |volume| {
                                if Self::intersects(
                                    &relative_pos,
                                    (
                                        interaction,
                                        interaction_offsets,
                                        *from_flipped == Flipped::Horizontal,
                                    ),
                                    (volume, body_offsets, *to_flipped == Flipped::Horizontal),
                                ) {
                                    Some(CollisionEvent::new(
                                        from,
                                        to,
                                        interaction.clone(),
                                        *volume,
                                    ))
                                } else {
                                    None
                                }
                            })
                        })
                        .collect::<Vec<CollisionEvent>>()
                };

                if !collision_events.is_empty() {
                    debug!("Collisions: {:?}", collision_events);
                }

                collision_ec.drain_vec_write(&mut collision_events);
            }
        }
    }
}
