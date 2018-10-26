/// Channels that are animatable on `CollisionFrame`
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum CollisionFrameChannel {
    /// The frame to use.
    Frame,
}
