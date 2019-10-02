use amethyst::{
    assets::Format,
    error::{format_err, ResultExt},
    Error,
};
use serde::{Deserialize, Serialize};

/// Format for loading from YAML files.
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct YamlFormat;

impl<D> Format<D> for YamlFormat
where
    D: for<'a> Deserialize<'a> + Send + Sync + 'static,
{
    fn name(&self) -> &'static str {
        stringify!(YamlFormat)
    }

    fn import_simple(&self, bytes: Vec<u8>) -> Result<D, Error> {
        serde_yaml::from_slice::<D>(&bytes)
            .with_context(|_| format_err!("Failed to deserialize YAML file")) // kcov-ignore
    }
}
