use amethyst::{
    ecs::{Join, ReadStorage, System, World, WriteStorage},
    renderer::camera::Camera,
    shred::{ResourceId, SystemData},
};
use camera_model::play::CameraTargetCoordinates;
use derivative::Derivative;
use derive_new::new;
use kinematic_model::config::{Position, Velocity};
use typename_derive::TypeName;

/// How much to divide the target velocity by, to smoothen the acceleration.
const SMOOTHING_FACTOR_DEFAULT: f32 = 3.;

/// Updates camera velocity to smoothen camera movement between its current and target position.
#[derive(Debug, Derivative, TypeName, new)]
#[derivative(Default)]
pub struct CameraVelocitySystem {
    /// How much to divide the target velocity by, to smoothen the acceleration.
    #[derivative(Default(value = "SMOOTHING_FACTOR_DEFAULT"))]
    pub smoothing_factor: f32,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct CameraVelocitySystemData<'s> {
    /// `Camera` components.
    #[derivative(Debug = "ignore")]
    pub cameras: ReadStorage<'s, Camera>,
    /// `CameraTargetCoordinates` components.
    #[derivative(Debug = "ignore")]
    pub camera_target_coordinateses: ReadStorage<'s, CameraTargetCoordinates>,
    /// `Position<f32>` components.
    #[derivative(Debug = "ignore")]
    pub positions: ReadStorage<'s, Position<f32>>,
    /// `Velocity<f32>` components.
    #[derivative(Debug = "ignore")]
    pub velocities: WriteStorage<'s, Velocity<f32>>,
}

impl<'s> System<'s> for CameraVelocitySystem {
    type SystemData = CameraVelocitySystemData<'s>;

    fn run(
        &mut self,
        CameraVelocitySystemData {
            cameras,
            camera_target_coordinateses,
            positions,
            mut velocities,
        }: Self::SystemData,
    ) {
        (
            &cameras,
            &camera_target_coordinateses,
            &positions,
            &mut velocities,
        )
            .join()
            .for_each(|(_, camera_target_coordinates, position, velocity)| {
                **velocity = {
                    // 1. Get distance between current position and target position.
                    //    Divide that by 10, this is the max velocity we will reach.
                    //
                    //     e.g. if we have to move 1000 pixels, at most we will move 100 per tick.
                    //
                    // 2. Calculate an average between the current velocity and the target velocity.
                    //
                    //     If our current velocity is 0, then we will increase to 33.
                    //     Next frame will be 44: (33 + 100) / 3
                    let velocity_limit = (**camera_target_coordinates - **position) / 10.;
                    (**velocity + velocity_limit) / self.smoothing_factor
                };
            });
    }
}
