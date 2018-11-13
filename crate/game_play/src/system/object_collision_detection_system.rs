use amethyst::{
    assets::AssetStorage,
    core::{nalgebra::Vector3, transform::Transform},
    ecs::{Entities, Join, Read, ReadStorage, System, Write},
    shrev::EventChannel,
};
use collision_model::{
    animation::{BodyFrameActiveHandle, InteractionFrameActiveHandle},
    config::{BodyFrame, Interaction, InteractionFrame},
    play::CollisionEvent,
};
use shape_model::Volume;
use typename::TypeName;

/// Detects collisions for all objects.
#[derive(Debug, Default, TypeName, new)]
pub(crate) struct ObjectCollisionDetectionSystem;

type ObjectCollisionDetectionSystemData<'s> = (
    Entities<'s>,
    ReadStorage<'s, Transform>,
    ReadStorage<'s, InteractionFrameActiveHandle>,
    Read<'s, AssetStorage<InteractionFrame>>,
    ReadStorage<'s, BodyFrameActiveHandle>,
    Read<'s, AssetStorage<BodyFrame>>,
    Write<'s, EventChannel<CollisionEvent>>,
);

impl ObjectCollisionDetectionSystem {
    fn intersects(relative_pos: &Vector3<f32>, interaction: &Interaction, body: &Volume) -> bool {
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
                let bound_x = relative_pos[0] as i32 + i_x;
                let bound_x_w = bound_x + i_w as i32;
                let x_intersects = (bound_x >= b_x && bound_x < b_x + b_w as i32)
                    || (bound_x_w >= b_x && bound_x_w < b_x + b_w as i32);

                let bound_y = relative_pos[1] as i32 + i_y;
                let bound_y_h = bound_y + i_h as i32;
                let y_intersects = (bound_y >= b_y && bound_y < b_y + b_h as i32)
                    || (bound_y_h >= b_y && bound_y_h < b_y + b_h as i32);

                let bound_z = relative_pos[2] as i32 + i_z;
                let bound_z_d = bound_z + i_d as i32;
                let z_intersects = (bound_z >= b_z && bound_z < b_z + b_d as i32)
                    || (bound_z_d >= b_z && bound_z_d < b_z + b_d as i32);

                x_intersects && y_intersects && z_intersects
            }

            // Explicitly fail because we haven't finished this.
            _ => unimplemented!(),
        })
    }
}

impl<'s> System<'s> for ObjectCollisionDetectionSystem {
    type SystemData = ObjectCollisionDetectionSystemData<'s>;

    fn run(
        &mut self,
        (
            entities,
            transforms,
            ifahs,
            interaction_frame_assets,
            bfahs,
            body_frame_assets,
            mut collision_ec,
        ): Self::SystemData,
    ) {
        // Naive collision detection.
        // TODO: Use broad sweep + narrow sweep for optimization.
        for (from, from_transform, ifah) in (&entities, &transforms, &ifahs).join() {
            for (to, to_transform, bfah) in (&entities, &transforms, &bfahs).join() {
                if from == to {
                    // Skip self
                    continue;
                }

                let relative_pos = to_transform.translation() - from_transform.translation();
                let interaction_frame = interaction_frame_assets
                    .get(ifah.current())
                    .expect("Expected `InteractionFrame` from handle to exist.");
                let body_frame = body_frame_assets
                    .get(bfah.current())
                    .expect("Expected `BodyFrame` from handle to exist.");

                let mut collision_events = match (&interaction_frame.interactions, &body_frame.body)
                {
                    (Some(ref interactions), Some(ref body_volumes)) => {
                        interactions
                            .iter()
                            .flat_map(|interaction| {
                                // loop through each body, if it hits, generate a collision event.

                                body_volumes.iter().filter_map(move |volume| {
                                    if Self::intersects(&relative_pos, interaction, volume) {
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
                    }
                    _ => Vec::new(),
                };

                if !collision_events.is_empty() {
                    debug!("Collisions: {:?}", collision_events);
                }

                collision_ec.drain_vec_write(&mut collision_events);
            }
        }
    }
}
