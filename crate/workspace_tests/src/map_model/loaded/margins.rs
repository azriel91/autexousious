#[cfg(test)]
mod tests {
    use map_model::{config::MapBounds, loaded::Margins};

    #[test]
    fn from_map_bounds() {
        let map_bounds = MapBounds::new(1, 2, 3, 10, 20, 30);
        assert_eq!(
            Margins {
                left: 1.,
                right: 11.,
                bottom: 35.,
                top: 55.,
                back: 3.,
                front: 33.,
            },
            map_bounds.into()
        );
    }
}
