#[cfg(test)]
mod tests {
    use std::{fs, io};

    use hamcrest::prelude::*;
    use tempfile::tempdir;

    use asset_loading::{
        NamespaceDirectory, NamespaceDiscoverer, ASSETS_DEFAULT_DIR, ASSETS_DOWNLOAD_DIR,
        ASSETS_TEST_DIR,
    };

    #[test]
    fn child_directories_returns_directory_children_and_symlinked_directories() -> io::Result<()> {
        let assets_tempdir = tempdir()?;
        let assets_dir = assets_tempdir.path();

        let test_dir = assets_dir.join(ASSETS_TEST_DIR);
        let default_dir = assets_dir.join(ASSETS_DEFAULT_DIR);
        let download_dir = assets_dir.join(ASSETS_DOWNLOAD_DIR);
        let user1_dir = download_dir.join("user1");
        let user2_dir = download_dir.join("user2");
        [
            &test_dir,
            &default_dir,
            &download_dir,
            &user1_dir,
            &user2_dir,
        ]
        .iter()
        .fold(Ok(()), |result, dir| {
            result.and_then(|_| fs::create_dir(&dir))
        })?;

        assert_that!(
            &NamespaceDiscoverer::discover(&assets_dir),
            contains(vec![
                NamespaceDirectory::new(ASSETS_TEST_DIR.to_string(), test_dir),
                NamespaceDirectory::new(ASSETS_DEFAULT_DIR.to_string(), default_dir),
                NamespaceDirectory::new("user1".to_string(), user1_dir),
                NamespaceDirectory::new("user2".to_string(), user2_dir),
            ])
            .exactly()
        );

        Ok(())
    }
}
