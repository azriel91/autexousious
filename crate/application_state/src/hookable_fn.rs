/// Enum variants representing where hook functions are supported.
///
/// These allow view crates to plug in logic that requires a `World` to run,
/// which works best for UI type crates in which managing the UI through a
/// `System` is complex:
///
/// * Initialization of UI entities should only be done once.
/// * Unable to detect pause/resume without introducing external status
///   resource.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum HookableFn {
    /// State `on_start()`.
    OnStart,
    /// State `on_stop()`.
    OnStop,
    /// State `on_pause()`.
    OnPause,
    /// State `on_resume()`.
    OnResume,
}
