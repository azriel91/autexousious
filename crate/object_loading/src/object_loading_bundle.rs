use amethyst::assets::Processor;
use amethyst::core::bundle::{Result, SystemBundle};
use amethyst::ecs::prelude::*;
use object_model::loaded::Character;

/// Adds `Processor::<`*`ObjectType`*`>` to the `World` with id
/// `"`*`object_type`*`_processor"`.
///
/// This is needed to allow the `loaded::*` types to be stored in `AssetStorage`.
#[derive(Debug, new)]
pub struct ObjectLoadingBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for ObjectLoadingBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        builder.add(Processor::<Character>::new(), "character_processor", &[]);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::env;

    use amethyst::assets::AssetStorage;
    use amethyst::prelude::*;
    use amethyst::Result;
    use object_model::loaded::Character;
    use object_model::ObjectType;

    use super::ObjectLoadingBundle;

    fn setup<'a, 'b>() -> Result<Application<'a, 'b>> {
        env::set_var("APP_DIR", env!("CARGO_MANIFEST_DIR"));
        let app = Application::build(format!("{}/assets", env!("CARGO_MANIFEST_DIR")), MockState)?
            .with_bundle(ObjectLoadingBundle)?
            .build()?;

        Ok(app)
    } // kcov-ignore

    #[test]
    fn bundle_build_adds_character_processor() {
        let app = setup().expect("ObjectLoadingBundle#build() should succeed");

        ObjectType::variants().iter().for_each(|object_type| {
            match *object_type {
                ObjectType::Character => {
                    // Next line will panic if the Processor wasn't added
                    app.world.read_resource::<AssetStorage<Character>>();
                }
            }
        });
    }

    #[derive(Debug)]
    struct MockState;
    impl State for MockState {}
}
