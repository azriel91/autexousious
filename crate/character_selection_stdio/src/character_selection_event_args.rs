/// Parameters to the mapper.
#[derive(Clone, Debug, PartialEq, StructOpt)]
pub struct CharacterSelectionEventArgs {
    /// Namespace of the character.
    pub namespace: String,
    /// Name of the character.
    pub name: String,
}
