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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use amethyst::{
        assets::{AssetStorage, Handle, Loader, Processor, ProgressCounter, Source},
        ecs::WorldExt,
        error::format_err,
        Error, State, StateData, Trans,
    };
    use amethyst_test::{AmethystApplication, GameUpdate};
    use asset_derive::Asset;
    use derive_deref::{Deref, DerefMut};
    use derive_new::new;
    use serde::{Deserialize, Serialize};

    use super::YamlFormat;

    const CODE_SOURCE_ID: &str = "code_source";

    #[test]
    fn loads_asset_with_yaml_format() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_system(Processor::<YamlThing>::new(), "yaml_thing_processor", &[])
            .with_setup(|world| {
                let mut code_source = CodeSource::new();
                code_source.insert(String::from("file.yaml"), "val: 123".as_bytes().to_vec());

                let mut loader = world.write_resource::<Loader>();
                loader.add_source(CODE_SOURCE_ID, code_source);
            }) // kcov-ignore
            .with_effect(|world| {
                let mut progress_counter = ProgressCounter::new();
                let thing_handle = {
                    let loader = world.read_resource::<Loader>();
                    loader.load_from(
                        "file.yaml",
                        YamlFormat,
                        CODE_SOURCE_ID,
                        &mut progress_counter,
                        &world.read_resource::<AssetStorage<YamlThing>>(),
                    )
                };

                world.insert(thing_handle);
                world.insert(progress_counter);
            })
            .with_state(|| WaitForLoad)
            .with_assertion(|world| {
                let thing_handle = world.read_resource::<Handle<YamlThing>>();
                let yaml_thing_assets = world.read_resource::<AssetStorage<YamlThing>>();
                let yaml_thing = yaml_thing_assets
                    .get(&thing_handle)
                    .expect("Expected YamlThing to be loaded.");

                assert_eq!(&YamlThing { val: 123 }, yaml_thing);
            })
            .run()
    }

    #[derive(Debug)]
    struct WaitForLoad;
    impl<T, E> State<T, E> for WaitForLoad
    where
        T: GameUpdate,
        E: Send + Sync + 'static,
    {
        fn update(&mut self, data: StateData<'_, T>) -> Trans<T, E> {
            data.data.update(&data.world);

            let progress_counter = data.world.read_resource::<ProgressCounter>();
            if !progress_counter.is_complete() {
                Trans::None // kcov-ignore
            } else {
                Trans::Pop
            }
        }
    }

    #[derive(Asset, Debug, Deserialize, PartialEq, Serialize)]
    pub struct YamlThing {
        val: i32,
    }

    #[derive(Debug, Deref, DerefMut, new)]
    struct CodeSource(#[new(default)] HashMap<String, Vec<u8>>);

    impl Source for CodeSource {
        fn modified(&self, _path: &str) -> Result<u64, Error> {
            Ok(0)
        }

        fn load(&self, path: &str) -> Result<Vec<u8>, Error> {
            let path = path.to_string();
            self.0.get(&path).cloned().ok_or(format_err!(
                "The `{}` asset is not registered in the `CodeSource` asset source",
                path
            ))
        }
    }
}
