use amethyst::ecs::{storage::VecStorage, Component};
use derive_new::new;

use crate::{config::BasicIrr, loaded::ReactionEffect};

/// Sequence to transition to on control input with requirements.
#[derive(Clone, Component, Debug, PartialEq, new)]
#[storage(VecStorage)]
pub struct InputReaction<IRR = BasicIrr>
where
    IRR: Send + Sync + 'static,
{
    /// Effects of the reaction `ReactionEffect`.
    pub effect: ReactionEffect,
    /// Requirement for the input reaction to happen.
    pub requirement: IRR,
}

impl<IRR> AsRef<InputReaction<IRR>> for InputReaction<IRR>
where
    IRR: Send + Sync + 'static,
{
    fn as_ref(&self) -> &InputReaction<IRR> {
        self
    }
}

impl<IRR> AsRef<IRR> for InputReaction<IRR>
where
    IRR: Send + Sync + 'static,
{
    fn as_ref(&self) -> &IRR {
        &self.requirement
    }
}
