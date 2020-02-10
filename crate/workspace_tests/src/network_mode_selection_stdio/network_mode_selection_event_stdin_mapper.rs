#[cfg(test)]
mod tests {
    use menu_model::MenuEvent;
    use network_mode_selection_model::{NetworkModeIndex, NetworkModeSelectionEventArgs};
    use stdio_spi::StdinMapper;

    use network_mode_selection_stdio::NetworkModeSelectionEventStdinMapper;

    #[test]
    fn maps_select_event() {
        let args = NetworkModeSelectionEventArgs::Select {
            index: NetworkModeIndex::Join,
        };

        let result = NetworkModeSelectionEventStdinMapper::map(&(), args);

        assert!(result.is_ok());
        assert_eq!(MenuEvent::Select(NetworkModeIndex::Join), result.unwrap())
    }

    #[test]
    fn maps_close_event() {
        let args = NetworkModeSelectionEventArgs::Close;

        let result = NetworkModeSelectionEventStdinMapper::map(&(), args);

        assert!(result.is_ok());
        assert_eq!(MenuEvent::Close, result.unwrap())
    }
}
