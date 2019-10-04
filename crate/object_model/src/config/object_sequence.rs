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
use sequence_model::config::{SequenceEndTransition, SequenceName};
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
    Frame: GameObjectFrame,
{
    /// Name of the sequence to switch to after this one has completed.
    ///
    /// Note: This may not be immediately after the last frame of the sequence. For example, a
    /// character that is in mid-air should remain in the last frame until it lands on the ground.
    pub next: SequenceEndTransition<SeqName>,
    /// Acceleration to apply to the object on this frame.
    pub acceleration: Option<ObjectAcceleration>,
    /// Key frames in the animation sequence.
    pub frames: Vec<Frame>,
}
