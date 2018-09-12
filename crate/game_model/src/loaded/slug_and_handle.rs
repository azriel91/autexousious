use amethyst::assets::Handle;

use config::AssetSlug;

/// Type that holds an asset's slug and handle.
#[derive(Clone, Debug, PartialEq, new)]
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
