#[cfg(test)]
mod tests {
    use state_registry::StateId;
    use stdio_command_model::{StateBarrier, StdioCommandEvent};
    use stdio_spi::StdinMapper;

    use stdio_command_stdio::StdioCommandEventStdinMapper;

    #[test]
    fn maps_command_barrier_event() {
        let args = StdioCommandEvent::StateBarrier(StateBarrier::new(StateId::GamePlay));

        let result = StdioCommandEventStdinMapper::map(&(), args);

        assert!(result.is_ok());
        assert_eq!(
            StdioCommandEvent::StateBarrier(StateBarrier::new(StateId::GamePlay)),
            result.unwrap()
        )
    }
}
