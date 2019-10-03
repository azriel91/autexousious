#[cfg(test)]
mod tests {
    use game_input_stdio::GameInputStdioError;

    #[test]
    fn fmt_display_is_user_friendly() {
        assert_eq!(
            "Failed to find entity with an `InputControlled` component with the specified controller ID: `4`.\n\
             The following controller IDs are associated with entities:\n\
             \n\
             * 0\n\
             * 1\n\
             \n\
            ",
            format!("{}", GameInputStdioError::EntityWithControllerIdNotFound { controller_id: 4, existent_controllers: vec![0, 1] })
        );
    }
}
