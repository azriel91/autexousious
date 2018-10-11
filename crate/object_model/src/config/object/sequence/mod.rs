//! Configuration types for object sequences.
//!
//! A sequence is an independent grouping of frames, which contains not only animation information,
//! but also the collision zones, interaction, and effects.
//!
//! Sequences are shared by different object types, and are genericized by the sequence ID. This is
//! because different object types have different valid sequence IDs, and we want to be able to
//! define this at compile time rather than needing to process this at run time.

pub use self::frame::Frame;
pub use self::sequence_id::SequenceId;
pub use self::sequence_state::SequenceState;

use sprite_loading::AnimationSequence;

mod frame;
mod sequence_id;
mod sequence_state;

/// Represents an independent action sequence of an object.
///
/// This carries the information necessary for an `Animation`, as well as the effects and
/// interactions that happen during each frame of that animation.
#[derive(Clone, Debug, Deserialize, PartialEq, new)]
pub struct Sequence<SeqId: SequenceId> {
    /// ID of the sequence to switch to after this one has completed.
    ///
    /// Note: This may not be immediately after the last frame of the sequence. For example, a
    /// character that is in mid-air should remain in the last frame until it lands on the ground.
    pub next: Option<SeqId>,
    /// Key frames in the animation sequence.
    pub frames: Vec<Frame>,
}

impl<SeqId: SequenceId> AnimationSequence for Sequence<SeqId> {
    type Frame = Frame;
    fn frames(&self) -> &[Frame] {
        &self.frames
    }
}

#[cfg(test)]
mod tests {
    use collision_model::config::CollisionFrame;
    use shape_model::Volume;
    use sprite_model::config::SpriteFrame;
    use toml;

    use super::{Frame, Sequence, SequenceId};

    const SEQUENCE_WITH_FRAMES: &str = r#"
        next = "Boo"
        frames = [
          { sheet = 0, sprite = 4, wait = 2 },
          { sheet = 0, sprite = 5, wait = 2 },
          { sheet = 1, sprite = 6, wait = 1 },
          { sheet = 1, sprite = 7, wait = 1 },
          { sheet = 0, sprite = 6, wait = 2 },
          { sheet = 0, sprite = 5, wait = 2 },
        ]
    "#;
    const SEQUENCE_WITH_FRAMES_EMPTY: &str = r#"
        frames = []
    "#;
    const SEQUENCE_WITH_BODY: &str = r#"
        [[frames]]
        sheet = 0
        sprite = 0
        wait = 0
        body = [
          { box = { x = -1, y = -2, z = -3, w = 11, h = 12, d = 13 } },
          { sphere = { x = -7, y = -8, z = -9, r = 17 } },
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
            Frame::new(SpriteFrame::new(0, 4, 2), CollisionFrame::new(None)),
            Frame::new(SpriteFrame::new(0, 5, 2), CollisionFrame::new(None)),
            Frame::new(SpriteFrame::new(1, 6, 1), CollisionFrame::new(None)),
            Frame::new(SpriteFrame::new(1, 7, 1), CollisionFrame::new(None)),
            Frame::new(SpriteFrame::new(0, 6, 2), CollisionFrame::new(None)),
            Frame::new(SpriteFrame::new(0, 5, 2), CollisionFrame::new(None)),
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
        let frames = vec![Frame::new(
            SpriteFrame::new(0, 0, 0),
            CollisionFrame::new(Some(body_volumes)),
        )];
        let expected = Sequence::new(None, frames);
        assert_eq!(expected, sequence);
    }

    #[derive(Clone, Copy, Debug, Derivative, Deserialize, PartialEq, Eq, Hash)]
    #[derivative(Default)]
    enum TestSeqId {
        #[derivative(Default)]
        Boo,
    }
    impl SequenceId for TestSeqId {}
}
