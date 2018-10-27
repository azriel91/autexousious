/// Newtype for `CollisionFramePrimitive` ID.
#[derive(Clone, Copy, Debug, Default, Display, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollisionFrameId(pub u64);

impl From<u64> for CollisionFrameId {
    fn from(id: u64) -> Self {
        CollisionFrameId(id)
    }
}
