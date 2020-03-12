#[cfg(test)]
mod tests {
    use game_input_model::{
        loaded::{PlayerController, PlayerControllers},
        play::ControllerIdOffset,
    };
    use network_session_model::play::{
        Session, SessionCode, SessionDevice, SessionDeviceId, SessionDeviceName, SessionDevices,
    };
    use session_join_model::{
        play::{SessionAcceptResponse, SessionJoinRequestParams},
        SessionJoinEvent,
    };
    use stdio_spi::StdinMapper;

    use session_join_stdio::SessionJoinEventStdinMapper;

    #[test]
    fn maps_session_join_request_event() {
        let session_device_name = SessionDeviceName::from(String::from("エイズリエル"));
        let session_code = SessionCode::from(String::from("abcd"));
        let player_controllers =
            PlayerControllers::new(vec![PlayerController::new(0, String::from("p0"))]);
        let args = SessionJoinEvent::SessionJoinRequest(SessionJoinRequestParams {
            session_device_name,
            session_code,
            player_controllers,
        });

        let result = SessionJoinEventStdinMapper::map(&(), args.clone());

        assert!(result.is_ok());
        assert_eq!(args, result.unwrap())
    }

    #[test]
    fn maps_join_cancel_event() {
        let args = SessionJoinEvent::JoinCancel;

        let result = SessionJoinEventStdinMapper::map(&(), args.clone());

        assert!(result.is_ok());
        assert_eq!(args, result.unwrap())
    }

    #[test]
    fn maps_session_accept_event() {
        let session_code = SessionCode::from(String::from("abcd"));
        let session_device_id = SessionDeviceId::new(1);
        let session_devices = SessionDevices::new(vec![
            SessionDevice {
                id: SessionDeviceId::new(1),
                name: SessionDeviceName::from(String::from("エイズリエル")),
                player_controllers: PlayerControllers::new(vec![PlayerController::new(
                    0,
                    String::from("p0"),
                )]),
            },
            SessionDevice {
                id: SessionDeviceId::new(2),
                name: SessionDeviceName::from(String::from("バイロン")),
                player_controllers: PlayerControllers::new(vec![PlayerController::new(
                    1,
                    String::from("p1"),
                )]),
            },
            SessionDevice {
                id: SessionDeviceId::new(3),
                name: SessionDeviceName::from(String::from("カルロー")),
                player_controllers: PlayerControllers::new(vec![PlayerController::new(
                    2,
                    String::from("p2"),
                )]),
            },
        ]);
        let player_controllers = PlayerControllers::new(vec![
            PlayerController::new(0, String::from("p0")),
            PlayerController::new(1, String::from("p1")),
            PlayerController::new(2, String::from("p2")),
        ]);
        let controller_id_offset = ControllerIdOffset::new(0);
        let args = SessionJoinEvent::SessionAccept(SessionAcceptResponse {
            session_device_id,
            session: Session {
                session_code,
                session_devices,
            },
            player_controllers,
            controller_id_offset,
        });

        let result = SessionJoinEventStdinMapper::map(&(), args.clone());

        assert!(result.is_ok());
        assert_eq!(args, result.unwrap())
    }

    #[test]
    fn maps_back_event() {
        let args = SessionJoinEvent::Back;

        let result = SessionJoinEventStdinMapper::map(&(), args.clone());

        assert!(result.is_ok());
        assert_eq!(args, result.unwrap())
    }
}
