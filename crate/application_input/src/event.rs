/// Events used to indicate top level transitions for an application.
#[derive(Debug, Clone)]
pub enum ApplicationEvent {
    /// Indicates the application should exit.
    Exit,
}
