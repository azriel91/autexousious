use amethyst::{animation::Animation, assets::Handle};

use crate::animation::BodyFrameActiveHandle;

/// Type alias for body volume animation handles.
pub type BodyAnimationHandle = Handle<Animation<BodyFrameActiveHandle>>;
