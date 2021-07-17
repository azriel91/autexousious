use amethyst::Error;
use structopt::StructOpt;

use crate::MapperSystemData;

/// Maps tokens from stdin to a state specific event.
pub trait StdinMapper {
    /// Resource needed by the mapper to construct the state specific event.
    ///
    /// TODO: Pending <https://gitlab.com/azriel91/autexousious/issues/76>
    ///
    /// Ideally we can have this be the `SystemData` of an ECS system. However,
    /// we cannot add a `Resources: for<'res> SystemData<'res>` trait bound
    /// as generic associated types (GATs) are not yet implemented. See:
    ///
    /// * <https://users.rust-lang.org/t/17444>
    /// * <https://github.com/rust-lang/rust/issues/44265>
    ///
    /// As of 2019-01-19, this workaround was posted:
    ///
    /// * <https://gist.github.com/ExpHP/7a464c184c876eaf27056a83c41356ee>
    type SystemData: for<'s> MapperSystemData<'s>;
    /// State specific event type that this maps tokens to.
    type Event: Send + Sync + 'static;
    /// Data structure representing the arguments.
    type Args: StructOpt;
    /// Returns the state specific event constructed from stdin tokens.
    ///
    /// # Parameters
    ///
    /// * `system_data`: Borrowed resources from the world.
    /// * `args`: Tokens parsed from stdin.
    fn map(
        system_data: &<Self::SystemData as MapperSystemData>::SystemData,
        args: Self::Args,
    ) -> Result<Self::Event, Error>;
}
