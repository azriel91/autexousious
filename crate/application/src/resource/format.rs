/// Format of the resource to load.
#[derive(Debug)]
pub enum Format {
    /// [Rusty Object Notation](https://crates.io/crates/ron).
    Ron,
    /// [Tom's Obvious Minimal Language](https://crates.io/crates/toml).
    Toml,
}
