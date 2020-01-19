use amethyst::{
    ecs::{storage::DenseVecStorage, Component, Entity, World, WriteStorage},
    shred::{ResourceId, SystemData},
};
use asset_model::ItemComponent;
use derivative::Derivative;
use derive_new::new;
use kinematic_model::config::Position;

/// Component indicating the chaser should stick to the target object.
#[derive(Clone, Component, Copy, Debug, Default, PartialEq, new)]
pub struct ChaseModeStick {
    /// Fixed offset from the target object.
    pub offset: Option<Position<f32>>,
}

/// `ChaseModeStickSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct ChaseModeStickSystemData<'s> {
    /// `ChaseModeStick` components.
    #[derivative(Debug = "ignore")]
    pub chase_mode_sticks: WriteStorage<'s, ChaseModeStick>,
}

impl<'s> ItemComponent<'s> for ChaseModeStick {
    type SystemData = ChaseModeStickSystemData<'s>;

    fn augment(&self, system_data: &mut Self::SystemData, entity: Entity) {
        let ChaseModeStickSystemData { chase_mode_sticks } = system_data;

        if chase_mode_sticks.get(entity).is_none() {
            chase_mode_sticks
                .insert(entity, *self)
                .expect("Failed to insert `ChaseModeStick` component.");
        }
    }
}
