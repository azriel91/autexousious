use std::env;
use std::path::{Path, PathBuf};

use config::error::Result;

/// Reads and returns the amethyst renderer configuration
pub fn find(path: &str) -> Result<PathBuf> {
    let mut exe_dir = env::current_exe().unwrap();
    exe_dir.pop();

    // Not sure that we need to have both OUT_DIR and CARGO_MANIFEST_DIR checked, but when we add
    // other resources we probably want to just read OUT_DIR and not CARGO_MANIFEST_DIR
    let mut base_dirs = vec![exe_dir];
    let optional_base_dirs = vec![option_env!("OUT_DIR"), option_env!("CARGO_MANIFEST_DIR")];
    for optional_dir in &optional_base_dirs {
        if let &Some(base_dir) = optional_dir {
            base_dirs.push(Path::new(base_dir).to_owned());
        }
    }
    for base_dir in &base_dirs {
        let mut config_path = base_dir.join("resources");
        config_path.push(&path);

        if config_path.exists() {
            return Ok(config_path);
        }
    }

    bail!(format!("Failed to find resources/{}", path))
}

#[cfg(test)]
mod test {
    use tempdir::tempdir;

    use super::find;

    #[test]
    fn find_returns_config_when_present() {}
}
