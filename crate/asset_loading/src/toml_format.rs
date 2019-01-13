use amethyst::assets::{Asset, Error, ResultExt, SimpleFormat};
use serde::{Deserialize, Serialize};

/// Format for loading from TOML files.
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TomlFormat;

impl<A> SimpleFormat<A> for TomlFormat
where
    A: Asset,
    A::Data: for<'a> Deserialize<'a> + Send + Sync + 'static,
{
    const NAME: &'static str = "Toml";
    type Options = ();

    fn import(&self, bytes: Vec<u8>, _: ()) -> Result<A::Data, Error> {
        toml::from_slice::<A::Data>(&bytes).chain_err(|| "Failed to deserialize TOML file")
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, thread, time::Duration};

    use amethyst::{
        assets::{
            self, Asset, AssetStorage, Error, ErrorKind, Handle, Loader, ProcessingState,
            Processor, ProgressCounter, Source,
        },
        ecs::storage::VecStorage,
    };
    use amethyst_test::AmethystApplication;
    use derive_deref::{Deref, DerefMut};
    use derive_new::new;
    use serde::{Deserialize, Serialize};

    use super::TomlFormat;

    const CODE_SOURCE_ID: &str = "code_source";

    #[test]
    fn loads_asset_with_toml_format() -> amethyst::Result<()> {
        AmethystApplication::blank()
            .with_system(Processor::<TomlThing>::new(), "toml_thing_processor", &[])
            .with_setup(|world| {
                let mut code_source = CodeSource::new();
                code_source.insert(String::from("file.toml"), "val = 123".as_bytes().to_vec());

                let mut loader = world.write_resource::<Loader>();
                loader.add_source(CODE_SOURCE_ID, code_source);
            })
            .with_effect(|world| {
                let mut progress_counter = ProgressCounter::new();
                let thing_handle = {
                    let loader = world.read_resource::<Loader>();
                    loader.load_from(
                        "file.toml",
                        TomlFormat,
                        (), // Options
                        CODE_SOURCE_ID,
                        &mut progress_counter,
                        &world.read_resource::<AssetStorage<TomlThing>>(),
                    )
                };

                world.add_resource(thing_handle);
                world.add_resource(progress_counter);
            })
            .with_assertion(|world| {
                let progress_counter = world.read_resource::<ProgressCounter>();
                while !progress_counter.is_complete() {
                    // Should load pretty quickly.
                    thread::sleep(Duration::from_millis(3));
                }

                let thing_handle = world.read_resource::<Handle<TomlThing>>();
                let toml_thing_assets = world.read_resource::<AssetStorage<TomlThing>>();
                let toml_thing = toml_thing_assets
                    .get(&thing_handle)
                    .expect("Expected TomlThing to be loaded.");

                assert_eq!(&TomlThing { val: 123 }, toml_thing);
            })
            .run()
    }

    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    struct TomlThing {
        val: i32,
    }

    impl Asset for TomlThing {
        const NAME: &'static str = "asset_loading::toml_format::tests::TomlThing";
        type Data = Self;
        type HandleStorage = VecStorage<Handle<Self>>;
    }

    impl From<TomlThing> for Result<ProcessingState<TomlThing>, Error> {
        fn from(object: TomlThing) -> Result<ProcessingState<TomlThing>, Error> {
            Ok(ProcessingState::Loaded(object))
        }
    }

    #[derive(Debug, Deref, DerefMut, new)]
    struct CodeSource(#[new(default)] HashMap<String, Vec<u8>>);

    impl Source for CodeSource {
        fn modified(&self, _path: &str) -> assets::Result<u64> {
            Ok(0)
        }

        fn load(&self, path: &str) -> assets::Result<Vec<u8>> {
            let path = path.to_string();
            self.0
                .get(&path)
                .cloned()
                .ok_or(Error::from_kind(ErrorKind::Asset(path)))
        }
    }
}
