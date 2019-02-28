//! Configuration types for object sequences.
//!
//! A sequence is an independent grouping of frames, which contains not only animation information,
//! but also the collision zones, interaction, and effects.
//!
//! Sequences are shared by different object types, and are genericized by the sequence ID. This is
//! because different object types have different valid sequence IDs, and we want to be able to
//! define this at compile time rather than needing to process this at run time.

pub use self::{object_frame::ObjectFrame, sequence_id::SequenceId, wait::Wait};

use collision_loading::InteractionAnimationSequence;
use collision_model::animation::BodyAnimationSequence;
use derive_new::new;
use serde::{Deserialize, Serialize};
use sprite_loading::AnimationSequence;

mod object_frame;
mod sequence_id;
mod wait;

/// Represents an independent action sequence of an object.
///
/// This carries the information necessary for an `Animation`, as well as the effects and
/// interactions that happen during each frame of that animation.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, new)]
pub struct Sequence<SeqId: SequenceId> {
    /// ID of the sequence to switch to after this one has completed.
    ///
    /// Note: This may not be immediately after the last frame of the sequence. For example, a
    /// character that is in mid-air should remain in the last frame until it lands on the ground.
    pub next: Option<SeqId>,
    /// Key frames in the animation sequence.
    pub frames: Vec<ObjectFrame>,
}

impl<SeqId: SequenceId> AnimationSequence for Sequence<SeqId> {
    type Frame = ObjectFrame;
    fn frames(&self) -> &[ObjectFrame] {
        &self.frames
    }
}

impl<SeqId: SequenceId> BodyAnimationSequence for Sequence<SeqId> {
    type Frame = ObjectFrame;
    fn frames(&self) -> &[ObjectFrame] {
        &self.frames
    }
}

impl<SeqId: SequenceId> InteractionAnimationSequence for Sequence<SeqId> {
    type Frame = ObjectFrame;
    fn frames(&self) -> &[ObjectFrame] {
        &self.frames
    }
}

#[cfg(test)]
mod tests {
    use amethyst::ecs::{storage::DenseVecStorage, Component};
    use collision_model::config::{Body, Interaction, Interactions};
    use derivative::Derivative;
    use serde::{Deserialize, Serialize};
    use shape_model::Volume;
    use specs_derive::Component;
    use sprite_model::config::SpriteRef;
    use toml;

    use super::{ObjectFrame, Sequence, SequenceId};
    use crate::config::object::Wait;

    const SEQUENCE_WITH_FRAMES: &str = r#"
        next = "Boo"
        frames = [
          { wait = 2, sprite = { sheet = 0, index = 4 } },
          { wait = 2, sprite = { sheet = 0, index = 5 } },
          { wait = 1, sprite = { sheet = 1, index = 6 } },
          { wait = 1, sprite = { sheet = 1, index = 7 } },
          { wait = 2, sprite = { sheet = 0, index = 6 } },
          { wait = 2, sprite = { sheet = 0, index = 5 } },
        ]
    "#;
    const SEQUENCE_WITH_FRAMES_EMPTY: &str = r#"
        frames = []
    "#;
    const SEQUENCE_WITH_BODY: &str = r#"
        [[frames]]
        sprite = { sheet = 0, index = 0 }
        body = [
          { box = { x = -1, y = -2, z = -3, w = 11, h = 12, d = 13 } },
          { sphere = { x = -7, y = -8, z = -9, r = 17 } },
        ]
    "#;
    const SEQUENCE_WITH_ITR: &str = r#"
        [[frames]]
        sprite = { sheet = 0, index = 0 }
        interactions = [
          { physical = { bounds = [{ sphere = { x = 1, y = 1, r = 1 } }] } },
        ]
    "#;

    #[test]
    fn sequence_with_empty_frames_list_deserializes_successfully() {
        let sequence = toml::from_str::<Sequence<TestSeqId>>(SEQUENCE_WITH_FRAMES_EMPTY)
            .expect("Failed to deserialize sequence.");

        let expected = Sequence::new(None, vec![]);
        assert_eq!(expected, sequence);
    }

    #[test]
    fn sequence_with_frames() {
        let sequence = toml::from_str::<Sequence<TestSeqId>>(SEQUENCE_WITH_FRAMES)
            .expect("Failed to deserialize sequence.");

        let frames = vec![
            ObjectFrame::new(
                Wait::new(2),
                SpriteRef::new(0, 4),
                Body::default(),
                Interactions::default(),
            ),
            ObjectFrame::new(
                Wait::new(2),
                SpriteRef::new(0, 5),
                Body::default(),
                Interactions::default(),
            ),
            ObjectFrame::new(
                Wait::new(1),
                SpriteRef::new(1, 6),
                Body::default(),
                Interactions::default(),
            ),
            ObjectFrame::new(
                Wait::new(1),
                SpriteRef::new(1, 7),
                Body::default(),
                Interactions::default(),
            ),
            ObjectFrame::new(
                Wait::new(2),
                SpriteRef::new(0, 6),
                Body::default(),
                Interactions::default(),
            ),
            ObjectFrame::new(
                Wait::new(2),
                SpriteRef::new(0, 5),
                Body::default(),
                Interactions::default(),
            ),
        ];
        let expected = Sequence::new(Some(TestSeqId::Boo), frames);
        assert_eq!(expected, sequence);
    }

    #[test]
    fn sequence_with_body() {
        let sequence = toml::from_str::<Sequence<TestSeqId>>(SEQUENCE_WITH_BODY)
            .expect("Failed to deserialize sequence.");

        let body_volumes = vec![
            Volume::Box {
                x: -1,
                y: -2,
                z: -3,
                w: 11,
                h: 12,
                d: 13,
            },
            Volume::Sphere {
                x: -7,
                y: -8,
                z: -9,
                r: 17,
            },
        ];
        let frames = vec![ObjectFrame::new(
            Wait::new(0),
            SpriteRef::new(0, 0),
            Body::new(body_volumes),
            Interactions::new(Vec::new()),
        )];
        let expected = Sequence::new(None, frames);
        assert_eq!(expected, sequence);
    }

    #[test]
    fn sequence_with_itr() {
        let sequence = toml::from_str::<Sequence<TestSeqId>>(SEQUENCE_WITH_ITR)
            .expect("Failed to deserialize sequence.");

        let interactions = vec![Interaction::Physical {
            bounds: vec![Volume::Sphere {
                x: 1,
                y: 1,
                z: 0,
                r: 1,
            }],
            hp_damage: 0,
            sp_damage: 0,
            multiple: false,
        }];
        let frames = vec![ObjectFrame::new(
            Wait::new(0),
            SpriteRef::new(0, 0),
            Body::new(Vec::new()),
            Interactions::new(interactions),
        )];
        let expected = Sequence::new(None, frames);
        assert_eq!(expected, sequence);
    }

    #[derive(
        Clone, Component, Copy, Debug, Derivative, Deserialize, PartialEq, Eq, Hash, Serialize,
    )]
    #[derivative(Default)]
    enum TestSeqId {
        #[derivative(Default)]
        Boo,
    }
    impl SequenceId for TestSeqId {}
}
