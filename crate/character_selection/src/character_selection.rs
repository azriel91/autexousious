/// Selected character ID or random for a particular controller.
#[derive(Clone, Copy, Debug, Derivative, Display, PartialEq, Eq)]
#[derivative(Default)]
pub enum CharacterSelection {
    /// Random.
    #[derivative(Default)]
    Random,
    /// Character with a particular ID.
    Id(usize),
}
