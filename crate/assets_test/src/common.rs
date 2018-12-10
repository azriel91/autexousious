use std::path::PathBuf;

use lazy_static::lazy_static;

/// Crate specific "assets" directory name.
const ASSETS: &str = "assets";

/// "test" namespace.
pub const NAMESPACE_TEST: &str = "test";

lazy_static! {
    /// `PathBuf` to the test assets directory.
    pub static ref ASSETS_PATH: PathBuf = {
        [
            env!("CARGO_MANIFEST_DIR"),
            ASSETS
        ]
        .iter()
        .collect::<PathBuf>()
    };

    /// `PathBuf` to the test assets directory.
    pub static ref NAMESPACE_TEST_PATH: PathBuf = {
        ASSETS_PATH.join(NAMESPACE_TEST)
    };
}
