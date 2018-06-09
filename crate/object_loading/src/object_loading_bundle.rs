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

    use amethyst::{assets::AssetStorage, prelude::*, Result};
    use object_model::{loaded::Character, ObjectType};

    use super::ObjectLoadingBundle;

    fn setup<'a, 'b>() -> Result<Application<'a, GameData<'a, 'b>>> {
        env::set_var("APP_DIR", env!("CARGO_MANIFEST_DIR"));

        let game_data = GameDataBuilder::default().with_bundle(ObjectLoadingBundle)?;
        let app = Application::build(format!("{}/assets", env!("CARGO_MANIFEST_DIR")), MockState)?
            .build(game_data)?;

        Ok(app)
    } // kcov-ignore

    #[test]
    fn bundle_build_adds_character_processor() {
        setup()
            .expect("ObjectLoadingBundle#build() should succeed")
            .run();
    }

    #[derive(Debug)]
    struct MockState;
    impl<'a, 'b> State<GameData<'a, 'b>> for MockState {
        fn update(&mut self, data: StateData<GameData>) -> Trans<GameData<'a, 'b>> {
            ObjectType::variants().iter().for_each(|object_type| {
                match *object_type {
                    ObjectType::Character => {
                        // Next line will panic if the Processor wasn't added
                        data.world.read_resource::<AssetStorage<Character>>();
                    }
                }
            });

            Trans::Quit
        }
    }
}
