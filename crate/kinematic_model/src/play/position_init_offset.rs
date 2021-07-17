use amethyst::ecs::{storage::DenseVecStorage, Component};
use derive_deref::{Deref, DerefMut};
use derive_new::new;

use crate::config::Position;

/// Offsets a `PositionInit` by some distance.
///
/// This is useful when the entity should be spawned some distance away from the
/// configuration `PositionInit` value, where the distance is calculated at
/// runtime.
#[derive(Clone, Component, Copy, Debug, Deref, DerefMut, new)]
pub struct PositionInitOffset(pub Position<f32>);
