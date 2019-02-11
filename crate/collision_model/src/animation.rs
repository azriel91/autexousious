//! Provides types to enable animation of `BodyFrame`s.
//!
//! # Object Configuration Animation
//!
//! In Amethyst, "animation" does not simply mean visual animation; it means any sort of data that
//! can morph over a certain input value such as time.
//!
//! When using Amethyst, animation is designed to be done per `Component`, e.g. we can morph the
//! `SpriteRender`, or alter the local `Transform` component. Therefore, if you structure your
//! application's configuration with a different data hierarchy, you should process it into the
//! `Component` type.
//!
//! In Autexousious, every object definition has a series of `Sequence`s, each `Sequence` has a
//! `Vec<ObjectFrame>`. Each `ObjectFrame` contains multiple parts:
//!
//! * Sprite
//! * Body
//! * Interactions
//! * Potentially more, such as weapon position
//!
//! These are intended to change frame by frame, in sync with each other.
//!
//! The question arises, should we have `ObjectFrame` as a `Component`, which means we have a single
//! `Animation<ObjectFrame>`, or should we create `Component`s for each of these configuration
//! parts?
//!
//! A top level `Animation<ObjectFrame>` means, when we add separate configuration parts, we do not
//! need to create separate `Component` types for each of these parts. `System`s that use a
//! configuration part will access it either via `ReadStorage<'s, ObjectFrame>`, or there must be an
//! intermediate system that reads the `ObjectFrame`s and writes the individual `Component`s for all
//! entities. Note that this design means animation is strictly `Step` interpolated, so each part is
//! discreetly calculated.
//!
//! Separate `Component`s means writing separate `Animation<_>` types for each component, and
//! processing the configuration into those types. Each of the animations are run at the same time
//! to keep them in sync with each other (same `time` input value). `System`s that use a
//! configuration part will access it via `ReadStorage<'s, Component>`. This design allows separate
//! components to have separate interpolation. However, if `Component`s are all going to be `Step`
//! interpolated, then this introduces a lot of additional effort for which the value is not
//! applicable.
//!
//! The following table compares the development implications for the two design choices:
//!
//! | Item              | Frame Component Animation   | Multi Component Animation                  |
//! | ----------------- | --------------------------- | ------------------------------------------ |
//! | Effort - Addition | Lower                       | Higher - process config into Component     |
//! | Performance | No duplication of interpolation   | Duplicates interpolation                   |
//! | Performance | Bottleneck with splitter `System` | -                                          |
//! | Performance | Poor data locality for systems    | Good locality - read components separately |
//! | System Data | Unclear if no splitter system     | Clear                                      |
//! | Compilation | Extra(?) if no splitter system    | Generation of animation component types    |
//!
//! There may be more that I haven't thought of right now.
//!
//! Animation primitives are `Copy + Serializable`, which means we cannot simply pass references to
//! the active `ObjectFrame` or component. This means we have to store a serializable ID, and look
//! up the frame when sampling the animation. This could be done by generating a `HashMap<u64, _>`,
//! where `_` is either `&ObjectFrame` or `&Component`, or `Handle`s to each, in order to not
//! `clone()` the configuration data.
//!
//! Was intending to go with `Animation<ObjectFrame>`, but the data locality point weighs pretty
//! strongly, so instead going with animating each component separately.

pub use self::{
    body_animation_frame::BodyAnimationFrame, body_animation_handle::BodyAnimationHandle,
    body_animation_sequence::BodyAnimationSequence,
    body_frame_active_handle::BodyFrameActiveHandle, body_frame_primitive::BodyFramePrimitive,
    interaction_animation_handle::InteractionAnimationHandle,
    interaction_frame_active_handle::InteractionFrameActiveHandle,
    interaction_frame_primitive::InteractionFramePrimitive,
};

mod body_animation_frame;
mod body_animation_handle;
mod body_animation_sequence;
mod body_frame_active_handle;
mod body_frame_primitive;
mod interaction_animation_handle;
mod interaction_frame_active_handle;
mod interaction_frame_primitive;
