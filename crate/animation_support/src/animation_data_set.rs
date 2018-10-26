use std::fmt::Debug;
use std::hash::Hash;

use fnv::FnvHashMap;
use named_type::NamedType;

/// References to non-`Serializable` or `Copy` data used by animations.
///
/// `Animation` primitives must be `InterpolationPrimitive + Clone + Copy + Send + Sync + 'static`.
/// If you need to animate a `Component` that does not meet these bounds, you may use this set to
/// store the data against an identifier such as `u64`. The `u64` can be used in the animation
/// primitive, and the data looked up during sampling in the animation.
#[derive(Debug, Default, NamedType, new)]
pub struct AnimationDataSet<I, D>
where
    I: Clone + Copy + Debug + Hash + PartialEq + Eq,
    D: Clone + Debug + Hash + PartialEq + Eq,
{
    #[new(default)]
    data: FnvHashMap<I, D>,
    #[new(default)]
    data_inverse: FnvHashMap<D, I>,
}

impl<I, D> AnimationDataSet<I, D>
where
    I: Clone + Copy + Debug + Hash + PartialEq + Eq,
    D: Clone + Debug + Hash + PartialEq + Eq,
{
    /// Returns the data for a given ID.
    pub fn data(&self, id: I) -> Option<D> {
        self.data.get(&id).cloned()
    }

    /// Returns the ID for a given data item..
    pub fn id(&self, data: &D) -> Option<I> {
        self.data_inverse.get(data).cloned()
    }

    /// Inserts a data item at the given ID
    pub fn insert(&mut self, id: I, data: D) {
        self.data.insert(id, data.clone());
        self.data_inverse.insert(data, id);
    }

    /// Removes data with the given ID.
    ///
    /// This has no effect if there is no data stored with that ID.
    pub fn remove(&mut self, id: I) {
        if let Some(data) = self.data.remove(&id) {
            self.data_inverse.remove(&data);
        }
    }

    /// Returns the number of data items in the set.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns whether the set contains any data.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Removes all data in the set.
    pub fn clear(&mut self) {
        self.data.clear();
        self.data_inverse.clear();
    }
}
