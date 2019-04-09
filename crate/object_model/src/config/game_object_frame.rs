use crate::config::ObjectFrame;

/// Fields common to object types' frames.
pub trait GameObjectFrame {
    /// Returns the `ObjectFrame` for this `GameObjectFrame`.
    fn object_frame(&self) -> &ObjectFrame;
}

impl GameObjectFrame for ObjectFrame {
    fn object_frame(&self) -> &ObjectFrame {
        self
    }
}
