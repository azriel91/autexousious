use asset_derive::Asset;
use derivative::Derivative;
use derive_deref::{Deref, DerefMut};
use derive_more::From;
use derive_new::new;
use sequence_model::loaded::ControlTransitions;

use crate::loaded::CharacterControlTransition;

/// Sequence ID to transition to when a `ControlAction` is pressed, held, or released.
#[derive(Asset, Clone, Debug, Derivative, Deref, DerefMut, From, PartialEq, new)]
#[derivative(Default(bound = ""))]
pub struct CharacterControlTransitions(pub ControlTransitions<CharacterControlTransition>);
