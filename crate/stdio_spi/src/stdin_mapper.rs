use structopt::StructOpt;

use crate::Result;

/// Maps tokens from stdin to a state specific event.
pub trait StdinMapper {
    /// Resource needed by the mapper to construct the state specific event.
    ///
    /// TODO: Pending <https://gitlab.com/azriel91/autexousious/issues/76>
    ///
    /// Ideally we can have this be the `SystemData` of an ECS system. However, we cannot add
    /// a `Resources: for<'res> SystemData<'res>` trait bound as generic associated types (GATs)
    /// are not yet implemented. See:
    ///
    /// * <https://users.rust-lang.org/t/17444>
    /// * <https://github.com/rust-lang/rust/issues/44265>
    type Resource;
    /// State specific event type that this maps tokens to.
    type Event: Send + Sync + 'static;
    /// Data structure representing the arguments.
    type Args: StructOpt;
    /// Returns the state specific event constructed from stdin tokens.
    ///
    /// # Parameters
    ///
    /// * `tokens`: Tokens received from stdin.
    fn map(resource: &Self::Resource, args: Self::Args) -> Result<Self::Event>;
}
