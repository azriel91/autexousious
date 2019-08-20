use std::collections::BTreeMap;

use amethyst::{
    assets::Handle,
    ecs::{World, WorldExt},
};
use derivative::Derivative;
use derive_new::new;

use crate::config::AssetSlug;

/// Type that holds an asset's slug and handle.
#[derive(Derivative, new)]
#[derivative(Clone(bound = ""), Debug(bound = ""), PartialEq(bound = ""))]
pub struct SlugAndHandle<T> {
    /// The asset slug
    pub slug: AssetSlug,
    /// The handle.
    pub handle: Handle<T>,
}

impl<T> From<(AssetSlug, Handle<T>)> for SlugAndHandle<T> {
    fn from((slug, handle): (AssetSlug, Handle<T>)) -> Self {
        Self { slug, handle }
    }
}

impl<'a, T> From<(&'a AssetSlug, &'a Handle<T>)> for SlugAndHandle<T> {
    fn from((slug, handle): (&'a AssetSlug, &'a Handle<T>)) -> Self {
        Self {
            slug: slug.clone(),
            handle: handle.clone(),
        }
    }
}

impl<'a, T> From<(&'a BTreeMap<AssetSlug, Handle<T>>, AssetSlug)> for SlugAndHandle<T> {
    fn from((slug_to_handles, slug): (&'a BTreeMap<AssetSlug, Handle<T>>, AssetSlug)) -> Self {
        let handle = slug_to_handles
            .get(&slug)
            .unwrap_or_else(|| panic!("Expected `{}` to be loaded.", slug))
            .clone();

        SlugAndHandle::from((slug, handle))
    }
}

impl<'a, T> From<(&'a World, AssetSlug)> for SlugAndHandle<T>
where
    T: Send + Sync + 'static,
{
    fn from((world, slug): (&'a World, AssetSlug)) -> Self {
        SlugAndHandle::from((
            &*world.read_resource::<BTreeMap<AssetSlug, Handle<T>>>(),
            slug,
        ))
    }
}
