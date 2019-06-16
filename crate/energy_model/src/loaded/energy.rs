use amethyst::assets::Handle;
use asset_derive::Asset;
use derive_new::new;
use object_model::game_object;
use typename_derive::TypeName;

use crate::config::{EnergyDefinition, EnergySequence, EnergySequenceId};

/// Represents an in-game energy that has been loaded.
///
/// Each of these fields should be a component that is attached to the energy entity.
#[game_object(EnergySequenceId)]
#[derive(Asset, Clone, Debug, PartialEq, TypeName, new)]
pub struct Energy;

/// Handle to an `Energy`.
pub type EnergyHandle = Handle<Energy>;
