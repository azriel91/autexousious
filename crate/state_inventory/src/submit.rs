/// Convenience macro for submitting a `StateEntry` to the `State` inventory.
#[macro_export]
macro_rules! submit {
    ($state_name:ident) => {
        use state_inventory::inventory;
        state_inventory::inventory::submit!(state_inventory::StateEntry::new(
            env!("CARGO_PKG_NAME"),
            stringify!($state_name)
        ));
    };
}
