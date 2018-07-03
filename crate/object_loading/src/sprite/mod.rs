pub(self) use self::material_creator::MaterialCreator;
pub(crate) use self::sprite_loader::SpriteLoader;
pub(self) use self::sprite_mesh_creator::SpriteMeshCreator;
pub(self) use self::sprite_sheet_mapper::SpriteSheetMapper;
pub(self) use self::texture_loader::TextureLoader;

mod material_creator;
mod sprite_loader;
mod sprite_mesh_creator;
mod sprite_sheet_mapper;
mod texture_loader;

#[cfg(test)]
mod test {
    use std::path::Path;

    use amethyst_test_support::AmethystApplication;
    use application::resource::dir::assets_dir;
    use game_model::config::ConfigRecord;

    use super::SpriteLoader;

    #[test]
    fn loads_sprite_sheets_textures_and_mesh() {
        assert!(
            AmethystApplication::render_base("loads_sprite_sheets_textures_and_mesh", false)
                .with_assertion(|world| {
                    let texture_index_offset = 0;
                    let mut bat_path = assets_dir(Some(development_base_dirs!())).unwrap();
                    bat_path.extend(Path::new("test/object/character/bat").iter());
                    let config_record = ConfigRecord::new(bat_path);
                    let result = SpriteLoader::load(world, texture_index_offset, &config_record);

                    if let Err(e) = result {
                        panic!("Failed to load sprites: {:?}", e); // kcov-ignore
                    } // kcov-ignore
                })
                .run()
                .is_ok()
        );
    }
}
