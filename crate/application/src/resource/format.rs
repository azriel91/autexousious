/// Format of the resource to load.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Format {
    /// [Rusty Object Notation](https://crates.io/crates/ron).
    Ron,
    /// [YAML Ain't Markup Language](https://yaml.org/).
    Yaml,
}
