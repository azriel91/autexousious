use std::collections::HashMap;

use amethyst::{
    assets::AssetLoaderSystemData,
    ecs::{storage::VecStorage, Component},
    Result,
};
use amethyst_test::AmethystApplication;
use derivative::Derivative;
use object_model::{
    config::object::SequenceId,
    game_object,
    loaded::{GameObject, Object, SequenceEndTransition, SequenceEndTransitions},
};
use serde::{Deserialize, Serialize};
use specs_derive::Component;

#[derive(Clone, Component, Copy, Debug, Derivative, Deserialize, PartialEq, Eq, Hash, Serialize)]
#[derivative(Default)]
#[storage(VecStorage)]
enum MagicSequenceId {
    #[derivative(Default)]
    Boo,
}
impl SequenceId for MagicSequenceId {}

#[game_object(MagicSequenceId, config::MagicDefinition)]
#[derive(Debug)]
struct Magic;

// TODO: use a proc_macro to generate most of this boilerplate.
mod config {
    use amethyst::{
        assets::{Asset, Handle},
        ecs::storage::VecStorage,
    };
    use derive_new::new;
    use object_model::config::{object::ObjectDefinition, GameObjectDefinition};
    use serde::{Deserialize, Serialize};

    use super::MagicSequenceId;

    /// Contains all of the sequences for an `Object`.
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize, new)]
    pub struct MagicDefinition {
        /// Sequences of actions this object can perform.
        #[serde(flatten)]
        pub object_definition: ObjectDefinition<MagicSequenceId>,
    }

    impl Asset for MagicDefinition {
        const NAME: &'static str = "object_model::tests::config::MagicDefinition";
        type Data = Self;
        type HandleStorage = VecStorage<Handle<Self>>;
    }

    impl GameObjectDefinition for MagicDefinition {
        type SequenceId = MagicSequenceId;

        fn object_definition(&self) -> &ObjectDefinition<Self::SequenceId> {
            &self.object_definition
        }
    }
}

#[test]
fn game_object_attribute_generates_handle_and_transitions_fields() -> Result<()> {
    AmethystApplication::blank()
        .with_assertion(|world| {
            let sequence_end_transitions = {
                let mut transitions = SequenceEndTransitions::default();
                transitions
                    .0
                    .insert(MagicSequenceId::Boo, SequenceEndTransition::default());
                transitions
            };
            let object_handle = {
                let object = Object::new(Vec::new(), HashMap::new(), HashMap::new());
                let magic_object_wrapper = MagicObjectWrapper(object);
                world.exec(|asset_loader: AssetLoaderSystemData<MagicObjectWrapper>| {
                    asset_loader.load_from_data(magic_object_wrapper, ())
                })
            };

            let magic_object = Magic {
                object_handle: object_handle.clone(),
                sequence_end_transitions: sequence_end_transitions.clone(),
            };

            assert_eq!(&object_handle, magic_object.object_handle());
            assert_eq!(
                &sequence_end_transitions,
                magic_object.sequence_end_transitions()
            );
        })
        .run()
}
