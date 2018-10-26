/// Newtype for `CollisionFramePrimitive` ID.
#[derive(Debug, Default, Display, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollisionFrameId(u64);

impl From<u64> for CollisionFrameId {
    fn from(id: u64) -> Self {
        CollisionFrameId(id)
    }
}
