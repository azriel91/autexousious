/// Events used to indicate top level transitions for an application.
#[derive(Clone, Debug, Hash, PartialEq)]
pub enum ApplicationEvent {
    /// Indicates the application should exit.
    Exit,
}
