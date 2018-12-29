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
    // impl_processing_state_from_object,
    loaded::{GameObject, Object, ObjectHandle, SequenceEndTransition, SequenceEndTransitions},
};
use object_model_derive::GameObject;
use specs_derive::Component;

#[derive(Clone, Component, Copy, Debug, Derivative, PartialEq, Eq, Hash)]
#[derivative(Default)]
#[storage(VecStorage)]
enum TestSequenceId {
    #[derivative(Default)]
    Boo,
}
impl SequenceId for TestSequenceId {}
// TODO: Pending <https://github.com/rust-lang/rfcs/issues/1856>
//
// Due to orphan rules being too strict, we cannot implement:
//
// ```rust,ignore
// impl From<Object<TestSequenceId>> for Result<ProcessingState<Object<TestSequenceId>>, Error>
// ```
//
// It works within the `object_model` crate because `Object` originates from that crate.
//
// impl_processing_state_from_object!(TestSequenceId);

#[derive(Debug, GameObject)]
struct MagicObject {
    /// Handle to loaded object data.
    pub handle: ObjectHandle<TestSequenceId>,
    /// Component sequence transitions when a sequence ends.
    pub transitions: SequenceEndTransitions<TestSequenceId>,
}

#[test]
fn derived_game_object_returns_handle_and_transitions() -> Result<()> {
    AmethystApplication::blank()
        .with_assertion(|world| {
            let sequence_end_transitions = {
                let mut transitions = SequenceEndTransitions::default();
                transitions
                    .0
                    .insert(TestSequenceId::Boo, SequenceEndTransition::default());
                transitions
            };
            let object_handle = {
                let object = Object::new(Vec::new(), HashMap::new());
                world.exec(
                    |asset_loader: AssetLoaderSystemData<Object<TestSequenceId>>| {
                        asset_loader.load_from_data(object, ())
                    },
                )
            };

            let transitions = sequence_end_transitions.clone();
            let handle = object_handle.clone();

            let magic_object = MagicObject {
                handle,
                transitions,
            };

            assert_eq!(&object_handle, magic_object.object_handle());
            assert_eq!(
                &sequence_end_transitions,
                magic_object.sequence_end_transitions()
            );
        })
        .run()
}
