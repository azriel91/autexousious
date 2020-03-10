#[cfg(test)]
mod tests {
    use game_input_model::loaded::{PlayerController, PlayerControllers};
    use network_session_model::play::{
        Session, SessionCode, SessionDevice, SessionDeviceId, SessionDeviceName, SessionDevices,
    };
    use session_host_model::{
        play::{SessionAcceptResponse, SessionHostRequestParams},
        SessionHostEvent,
    };
    use stdio_spi::StdinMapper;

    use session_host_stdio::SessionHostEventStdinMapper;

    #[test]
    fn maps_session_host_request_event() {
        let session_device_name = SessionDeviceName::from(String::from("エイズリエル"));
        let player_controllers =
            PlayerControllers::new(vec![PlayerController::new(0, String::from("p0"))]);
        let args = SessionHostEvent::SessionHostRequest(SessionHostRequestParams {
            session_device_name,
            player_controllers,
        });

        let result = SessionHostEventStdinMapper::map(&(), args.clone());

        assert!(result.is_ok());
        assert_eq!(args, result.unwrap())
    }

    #[test]
    fn maps_host_cancel_event() {
        let args = SessionHostEvent::HostCancel;

        let result = SessionHostEventStdinMapper::map(&(), args.clone());

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
        let args = SessionHostEvent::SessionAccept(SessionAcceptResponse {
            session_device_id,
            session: Session {
                session_code,
                session_devices,
            },
        });

        let result = SessionHostEventStdinMapper::map(&(), args.clone());

        assert!(result.is_ok());
        assert_eq!(args, result.unwrap())
    }

    #[test]
    fn maps_back_event() {
        let args = SessionHostEvent::Back;

        let result = SessionHostEventStdinMapper::map(&(), args.clone());

        assert!(result.is_ok());
        assert_eq!(args, result.unwrap())
    }
}
