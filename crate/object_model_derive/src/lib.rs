#![recursion_limit = "512"]

//! Provides the `#[game_object]` proc_macro_attribute to implement the `GameObject` trait.
//!
//! For example usage of this macro, refer to the documentation for the `GameObject` trait.

extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

use crate::{
    game_object_attribute_args::GameObjectAttributeArgs, game_object_gen::game_object_gen,
};

mod game_object_attribute_args;
mod game_object_gen;
mod game_object_impl;
mod object_wrapper_gen;

/// For the following code:
///
/// ```rust,ignore
/// #[game_object(MagicSequenceId, MagicDefinition)]
/// pub struct Magic;
/// ```
///
/// the following is effectively generated, without the `use` statements polluting scope:
///
/// ```rust,ignore
/// use asset_derive::Asset;
/// use derive_deref::{Deref, DerefMut};
/// use sequence_model::{frame_component_data, config::SequenceEndTransition};
/// use typename_derive::TypeName;
///
/// use crate::config::MagicSequenceId;
///
/// pub struct Magic {
///     /// Handle to loaded object data.
///     pub object_handle: Handle<MagicObjectWrapper>,
/// }
///
/// impl GameObject for Magic {
///     const OBJECT_TYPE: ObjectType = ObjectType::TestObject;
///     type SequenceId = MagicSequenceId;
///     type GameObjectSequence = MagicSequence;
///     type Definition = MagicObjectDefinition;
///     type ObjectWrapper = MagicObjectWrapper;
///
///     fn object_handle(&self) -> &Handle<MagicObjectWrapper> {
///         &self.object_handle
///     }
/// }
///
/// /// Newtype for `Object`.
/// #[derive(Debug, Deref, DerefMut)]
/// pub struct MagicObjectWrapper(Object);
///
/// impl ObjectWrapper for MagicObjectWrapper {
///     fn new(object: Object) -> Self {
///         MagicObjectWrapper(object)
///     }
///
///     fn inner(&self) -> &object_model::loaded::Object {
///         &self.0
///     }
///
///     fn inner_mut(&mut self) -> &mut object_model::loaded::Object {
///         &mut self.0
///     }
/// }
///
/// impl Asset for MagicObjectWrapper {
///     const NAME: &'static str = concat!(module_path!(), "::", stringify!(MagicObjectWrapper));
///     type Data = MagicDefinition;
///     type HandleStorage = VecStorage<Handle<Self>>;
/// }
///
/// impl From<MagicObjectWrapper> for Result<ProcessingState<MagicObjectWrapper>, Error> {
///     fn from(object: MagicObjectWrapper) -> Result<ProcessingState<MagicObjectWrapper>, Error> {
///         Ok(ProcessingState::Loaded(object))
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn game_object(args: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as GameObjectAttributeArgs);
    let ast = parse_macro_input!(item as DeriveInput);

    game_object_gen(args, ast)
}
