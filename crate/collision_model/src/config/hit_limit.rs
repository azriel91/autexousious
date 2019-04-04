use derivative::Derivative;

/// Number of objects a `Hit` may collide with.
#[derive(Clone, Copy, Debug, Derivative, PartialEq, Eq)]
#[derivative(Default)]
pub enum HitLimit {
    #[derivative(Default)]
    /// Limit to `n` objects.
    Limit(#[derivative(Default(value = "1"))] u32),
    /// Not limited.
    Unlimited,
}
