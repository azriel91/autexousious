//! Configuration types for object sequences.
//!
//! A sequence is an independent grouping of frames, which contains not only animation information,
//! but also the collision zones, interaction, and effects.
//!
//! Sequences are shared by different object types, and are genericized by the sequence name. This is
//! because different object types have different valid sequence names, and we want to be able to
//! define this at compile time rather than needing to process this at run time.

use derive_new::new;
use kinematic_model::config::ObjectAcceleration;
use sequence_model::config::{Sequence, SequenceName, Wait};
use serde::{Deserialize, Serialize};

use crate::config::{GameObjectFrame, ObjectFrame};

/// Represents an independent action sequence of an object.
///
/// This carries the information necessary for an `Animation`, as well as the effects and
/// interactions that happen during each frame of that animation.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, new)]
#[serde(default, deny_unknown_fields)]
pub struct ObjectSequence<SeqName, Frame = ObjectFrame>
where
    SeqName: SequenceName,
    Frame: AsRef<Wait> + Default + GameObjectFrame,
{
    /// Common sequence information.
    #[serde(flatten)]
    pub sequence: Sequence<SeqName, Frame>,
    /// Acceleration to apply to the object on this frame.
    pub acceleration: Option<ObjectAcceleration>,
}
