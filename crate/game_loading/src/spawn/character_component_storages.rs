use amethyst::ecs::prelude::*;
use derivative::Derivative;
use game_input::{ControllerInput, InputControlled};
use object_model::{
    config::object::CharacterSequenceId,
    entity::{Grounding, HealthPoints, Mirrored, RunCounter, SequenceStatus},
    loaded::{CharacterHandle, ObjectHandle, SequenceEndTransitions},
};
use shred_derive::SystemData;

/// Character specific `Component` storages.
///
/// These are the storages for the components specific to character objects. See also
/// `ObjectComponentStorages`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct CharacterComponentStorages<'s> {
    /// `InputControlled` component storage.
    #[derivative(Debug = "ignore")]
    pub input_controlleds: WriteStorage<'s, InputControlled>,
    /// `ControllerInput` component storage.
    #[derivative(Debug = "ignore")]
    pub controller_inputs: WriteStorage<'s, ControllerInput>,
    /// `CharacterHandle` component storage.
    #[derivative(Debug = "ignore")]
    pub character_handles: WriteStorage<'s, CharacterHandle>,
    /// `ObjectHandle` component storage.
    #[derivative(Debug = "ignore")]
    pub object_handles: WriteStorage<'s, ObjectHandle<CharacterSequenceId>>,
    /// `SequenceEndTransitions` component storage.
    #[derivative(Debug = "ignore")]
    pub sequence_end_transitionses: WriteStorage<'s, SequenceEndTransitions<CharacterSequenceId>>,
    /// `HealthPoints` component storage.
    #[derivative(Debug = "ignore")]
    pub health_pointses: WriteStorage<'s, HealthPoints>,
    /// `CharacterSequenceId` component storage.
    #[derivative(Debug = "ignore")]
    pub character_sequence_ids: WriteStorage<'s, CharacterSequenceId>,
    /// `SequenceStatus` component storage.
    #[derivative(Debug = "ignore")]
    pub sequence_statuses: WriteStorage<'s, SequenceStatus>,
    /// `RunCounter` component storage.
    #[derivative(Debug = "ignore")]
    pub run_counters: WriteStorage<'s, RunCounter>,
    /// `Mirrored` component storage.
    #[derivative(Debug = "ignore")]
    pub mirroreds: WriteStorage<'s, Mirrored>,
    /// `Grounding` component storage.
    #[derivative(Debug = "ignore")]
    pub groundings: WriteStorage<'s, Grounding>,
}
