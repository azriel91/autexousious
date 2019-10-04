#[cfg(test)]
mod tests {
    #[cfg(unix)]
    use std::os::unix;
    #[cfg(windows)]
    use std::os::windows;
    use std::{fs, io};

    use hamcrest::prelude::*;
    use tempfile::tempdir;

    use asset_loading::DirTraverse;

    #[test]
    fn child_directories_returns_directory_children_and_symlinked_directories() -> io::Result<()> {
        let container_tempdir = tempdir()?;
        let container_dir = container_tempdir.path();
        let external_dir = container_dir.join("external");
        fs::create_dir(&external_dir)?;

        let assets_dir = container_dir.join("assets");
        fs::create_dir(&assets_dir)?;
        let child_dir = assets_dir.join("child_dir");
        fs::create_dir(&child_dir)?;
        let child_file = assets_dir.join("child_file");
        fs::File::create(&child_file)?;
        let child_symlink = assets_dir.join("child_sym");
        #[cfg(unix)]
        unix::fs::symlink(external_dir, &child_symlink)?;
        #[cfg(windows)]
        windows::fs::symlink_dir(external_dir, &child_symlink)?;

        // kcov-ignore-start
        assert_that!(
            // kcov-ignore-end
            &DirTraverse::child_directories(&assets_dir),
            contains(vec![child_dir, child_symlink]).exactly()
        );

        Ok(())
    }

    #[test]
    fn entries_returns_directory_entries_iterator_when_directory_can_be_read() -> io::Result<()> {
        let assets_tempdir = tempdir()?;
        let assets_dir = assets_tempdir.path();
        let child_dir = assets_dir.join("child");
        fs::create_dir(&child_dir)?;

        assert!(DirTraverse::entries(&child_dir).is_some());

        Ok(())
    }

    #[test]
    fn entries_returns_none_when_directory_fails_to_be_read() -> io::Result<()> {
        let assets_tempdir = tempdir()?;
        let assets_dir = assets_tempdir.path();
        let child_dir = assets_dir.join("non_existent");

        assert!(DirTraverse::entries(&child_dir).is_none());

        Ok(())
    }

    #[test]
    fn read_dir_opt_is_returns_some_when_dir_is_accessible() -> io::Result<()> {
        let assets_tempdir = tempdir()?;
        let assets_dir = assets_tempdir.path();
        let child_dir = assets_dir.join("child");
        fs::create_dir(&child_dir)?;

        assert!(DirTraverse::read_dir_opt(&child_dir).is_some());

        Ok(())
    }

    #[test]
    fn read_dir_opt_is_returns_none_when_dir_does_not_exist() -> io::Result<()> {
        let assets_tempdir = tempdir()?;
        let assets_dir = assets_tempdir.path();
        let child_dir = assets_dir.join("child");

        assert!(DirTraverse::read_dir_opt(&child_dir).is_none());

        Ok(())
    }

    #[test]
    fn read_dir_opt_is_returns_none_when_file_is_not_dir() -> io::Result<()> {
        let assets_tempdir = tempdir()?;
        let assets_dir = assets_tempdir.path();
        let child_file = assets_dir.join("child");
        fs::File::create(&child_file)?;

        assert!(DirTraverse::read_dir_opt(&child_file).is_none());

        Ok(())
    }

    #[test]
    fn entry_to_dir_path_buf_is_some_when_entry_is_dir() -> io::Result<()> {
        let assets_tempdir = tempdir()?;
        let assets_dir = assets_tempdir.path();
        let child_dir = assets_dir.join("child");
        fs::create_dir(&child_dir)?;

        let entry = assets_dir
            .read_dir()?
            .filter_map(|entry| entry.ok())
            .next()
            .expect("Expected entry for `child` directory.");

        assert_eq!(Some(child_dir), DirTraverse::entry_to_dir_path_buf(&entry));

        Ok(())
    }

    #[test]
    fn entry_to_dir_path_buf_is_some_when_entry_is_dir_symlink() -> io::Result<()> {
        let container_tempdir = tempdir()?;
        let container_dir = container_tempdir.path();
        let external_dir = container_dir.join("external");
        fs::create_dir(&external_dir)?;

        let assets_dir = container_dir.join("assets");
        fs::create_dir(&assets_dir)?;
        let child_symlink = assets_dir.join("child");
        #[cfg(unix)]
        unix::fs::symlink(external_dir, &child_symlink)?;
        #[cfg(windows)]
        windows::fs::symlink_dir(external_dir, &child_symlink)?;

        let entry = assets_dir
            .read_dir()?
            .filter_map(|entry| entry.ok())
            .next()
            .expect("Expected entry for `child` directory.");

        assert_eq!(
            Some(child_symlink),
            DirTraverse::entry_to_dir_path_buf(&entry)
        );

        Ok(())
    }

    #[test]
    fn entry_to_dir_path_buf_is_none_when_entry_is_file() -> io::Result<()> {
        let assets_tempdir = tempdir()?;
        let assets_dir = assets_tempdir.path();
        let child_file = assets_dir.join("child");
        fs::File::create(&child_file)?;

        let entry = assets_dir
            .read_dir()?
            .filter_map(|entry| entry.ok())
            .next()
            .expect("Expected entry for `child` file.");

        assert_eq!(None, DirTraverse::entry_to_dir_path_buf(&entry));

        Ok(())
    }

    #[test]
    fn entry_to_dir_path_buf_is_none_when_entry_is_file_symlink() -> io::Result<()> {
        let container_tempdir = tempdir()?;
        let container_dir = container_tempdir.path();
        let external_file = container_dir.join("external");
        fs::File::create(&external_file)?;

        let assets_dir = container_dir.join("assets");
        fs::create_dir(&assets_dir)?;
        let child_symlink = assets_dir.join("child");
        #[cfg(unix)]
        unix::fs::symlink(external_file, &child_symlink)?;
        #[cfg(windows)]
        windows::fs::symlink_file(external_file, &child_symlink)?;

        let entry = assets_dir
            .read_dir()?
            .filter_map(|entry| entry.ok())
            .next()
            .expect("Expected entry for `child` directory.");

        assert_eq!(None, DirTraverse::entry_to_dir_path_buf(&entry));

        Ok(())
    }
}
