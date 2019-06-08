// kcov-ignore-start
use amethyst::{
    assets::{AssetStorage, Loader},
    ecs::{storage::VecStorage, Component},
    Result,
};
use amethyst_test::AmethystApplication;
use derivative::Derivative;
use object_model::{config::ObjectAssetData, game_object, loaded::GameObject};
use sequence_model::config::SequenceId;
use serde::{Deserialize, Serialize};
use specs_derive::Component;
// kcov-ignore-end

#[derive(Clone, Component, Copy, Debug, Derivative, Deserialize, PartialEq, Eq, Hash, Serialize)]
#[derivative(Default)]
#[storage(VecStorage)]
pub enum MagicSequenceId {
    #[derivative(Default)]
    Boo,
}
impl SequenceId for MagicSequenceId {}

#[game_object(
    MagicSequenceId,
    sequence = config::MagicSequence,
    definition = config::MagicDefinition,
    object_type = TestObject,
)]
#[derive(Debug)]
struct Magic;

// TODO: use a proc_macro to generate most of this boilerplate.
mod config {
    use amethyst::{
        assets::{Asset, Handle},
        ecs::storage::VecStorage,
    };
    use derive_new::new;
    use object_model::config::{
        GameObjectDefinition, GameObjectSequence, ObjectDefinition, ObjectFrame, ObjectSequence,
    };
    use serde::{Deserialize, Serialize};

    use super::MagicSequenceId;

    /// Contains all of the sequences for an `Object`.
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, new)]
    pub struct MagicDefinition {
        /// Sequences of actions this object can perform.
        #[serde(flatten)]
        pub object_definition: ObjectDefinition<MagicSequence>,
    }

    impl Asset for MagicDefinition {
        const NAME: &'static str = "object_model::tests::config::MagicDefinition";
        type Data = Self;
        type HandleStorage = VecStorage<Handle<Self>>;
    }

    impl GameObjectDefinition for MagicDefinition {
        type GameObjectSequence = MagicSequence;

        fn object_definition(&self) -> &ObjectDefinition<Self::GameObjectSequence> {
            &self.object_definition
        }
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct MagicSequence {
        /// Object sequence for common object fields.
        #[serde(flatten)]
        pub object_sequence: ObjectSequence<MagicSequenceId>,
    }

    impl GameObjectSequence for MagicSequence {
        type SequenceId = MagicSequenceId;
        type GameObjectFrame = ObjectFrame;

        fn object_sequence(&self) -> &ObjectSequence<Self::SequenceId, Self::GameObjectFrame> {
            &self.object_sequence
        }
    }
}

#[test]
fn game_object_attribute_generates_handle_field() -> Result<()> {
    AmethystApplication::blank()
        .with_setup(|world| {
            world.add_resource(AssetStorage::<config::MagicDefinition>::new());
            world.add_resource(AssetStorage::<MagicObjectWrapper>::new());
        })
        .with_assertion(|world| {
            let object_handle = {
                let loader = world.read_resource::<Loader>();
                let definition = config::MagicDefinition::default();
                let definition_handle =
                    loader.load_from_data(definition, (), &world.read_resource());

                let object_asset_data = ObjectAssetData::new(definition_handle, Vec::new());

                loader.load_from_data(object_asset_data, (), &world.read_resource())
            };

            let magic_object = Magic {
                object_handle: object_handle.clone(),
            };

            assert_eq!(&object_handle, magic_object.object_handle());
        })
        .run()
}
