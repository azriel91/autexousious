use derive_new::new;

/// Event indicating the `SessionStatus` resource changed.
#[derive(Clone, Copy, Debug, Default, PartialEq, new)]
pub struct SessionStatusEvent;
