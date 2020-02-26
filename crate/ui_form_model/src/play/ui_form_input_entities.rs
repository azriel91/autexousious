use amethyst::ecs::Entity;
use derive_deref::{Deref, DerefMut};
use derive_new::new;

/// Input entities of the `UiForm` (`Vec<Entity>` newtype).
///
/// These are sorted in the order declared in configuration.
#[derive(Clone, Debug, Default, Deref, DerefMut, PartialEq, new)]
pub struct UiFormInputEntities(pub Vec<Entity>);
