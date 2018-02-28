/// Events used to indicate top level transitions for an application.
///
/// # Type Parameters
///
/// * `I`: Type that represents the index of the selected menu item.
#[derive(Clone, Debug, Hash, PartialEq)]
pub enum MenuEvent<I> {
    /// Indicates a menu item was pressed.
    Select(I),
    /// Indicates the menu should be closed.
    ///
    /// TODO: Not used in this example; but should `UiEvent`s also include keyboard / device input?
    #[allow(dead_code)]
    Close,
}
