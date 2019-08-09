use amethyst::{
    assets::{Asset, Handle, ProcessingState},
    ecs::storage::VecStorage,
    Error,
};
use derivative::Derivative;
use derive_deref::{Deref, DerefMut};
use derive_more::From;
use derive_new::new;
use sequence_model::loaded::ControlTransitions;

use crate::{config::CharacterSequenceId, loaded::CharacterControlTransition};

/// Sequence ID to transition to when a `ControlAction` is pressed, held, or released.
#[derive(Clone, Debug, Derivative, Deref, DerefMut, From, PartialEq, new)]
#[derivative(Default(bound = ""))]
pub struct CharacterControlTransitions(
    pub ControlTransitions<CharacterSequenceId, CharacterControlTransition>,
);

impl Asset for CharacterControlTransitions {
    const NAME: &'static str = concat!(
        module_path!(),
        "::",
        stringify!(CharacterControlTransitions)
    );
    type Data = Self;
    type HandleStorage = VecStorage<Handle<Self>>;
}

impl From<CharacterControlTransitions>
    for Result<ProcessingState<CharacterControlTransitions>, Error>
{
    fn from(
        character: CharacterControlTransitions,
    ) -> Result<ProcessingState<CharacterControlTransitions>, Error> {
        Ok(ProcessingState::Loaded(character))
    }
}

/// Handle to a `CharacterControlTransitions` asset.
pub type CharacterControlTransitionsHandle = Handle<CharacterControlTransitions>;
