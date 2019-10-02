#[cfg(test)]
mod tests {
    use std::{fs, io, path::PathBuf};

    use asset_model::config::{AssetRecord, AssetSlugBuilder};
    use hamcrest::prelude::*;
    use object_type::ObjectType;
    use tempfile::tempdir;

    use asset_loading::ObjectIndexer;

    #[test]
    fn returns_asset_record_for_each_object() -> io::Result<()> {
        let objects_tempdir = tempdir()?;
        let objects_dir = objects_tempdir.path();

        let character_dir = objects_dir.join("character");
        let char_0_dir = character_dir.join("char_0");
        let char_1_dir = character_dir.join("char_1");
        [&character_dir, &char_0_dir, &char_1_dir]
            .iter()
            .fold(Ok(()), |result, dir| {
                result.and_then(|_| fs::create_dir(&dir))
            })?;

        let object_assets_records = ObjectIndexer::index("rara", &objects_dir);

        // kcov-ignore-start
        assert_that!(
            // kcov-ignore-end
            object_assets_records.get(&ObjectType::Character).unwrap(),
            contains(vec![
                asset_record("rara", "char_0", char_0_dir),
                asset_record("rara", "char_1", char_1_dir),
            ])
            .exactly()
        );

        Ok(())
    }

    fn asset_record(namespace: &str, name: &str, path: PathBuf) -> AssetRecord {
        AssetRecord {
            asset_slug: AssetSlugBuilder::default()
                .namespace(namespace.to_string())
                .name(name.to_string())
                .build()
                .expect("Failed to build asset slug."),
            path,
        }
    }
}
