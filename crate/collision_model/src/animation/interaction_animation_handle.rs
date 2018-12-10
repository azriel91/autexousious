use amethyst::{animation::Animation, assets::Handle};

use crate::animation::InteractionFrameActiveHandle;

/// Type alias for interaction volume animation handles.
pub type InteractionAnimationHandle = Handle<Animation<InteractionFrameActiveHandle>>;
