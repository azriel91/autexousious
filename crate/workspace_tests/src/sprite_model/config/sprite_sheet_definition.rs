#[cfg(test)]
mod tests {
    use sprite_model::config::SpriteSheetDefinition;

    #[test]
    fn has_border_default_is_true() {
        assert!(SpriteSheetDefinition::has_border_default());
    }
}
