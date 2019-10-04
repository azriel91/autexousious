use amethyst::ecs::{Join, System, WriteStorage};
use derive_new::new;
use object_status_model::config::StunPoints;

use typename_derive::TypeName;

/// Decrements `StunPoints`.
#[derive(Debug, Default, TypeName, new)]
pub struct StunPointsReductionSystem;

type StunPointsReductionSystemData<'s> = WriteStorage<'s, StunPoints>;

impl<'s> System<'s> for StunPointsReductionSystem {
    type SystemData = StunPointsReductionSystemData<'s>;

    fn run(&mut self, mut stun_pointses: Self::SystemData) {
        (&mut stun_pointses).join().for_each(|stun_points| {
            if *stun_points > 0 {
                *stun_points -= 1;
            }
        });
    } // kcov-ignore
}
