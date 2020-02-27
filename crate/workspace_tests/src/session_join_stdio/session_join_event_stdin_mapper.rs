#[cfg(test)]
mod tests {
    use network_session_model::play::{
        SessionCode, SessionDevice, SessionDeviceId, SessionDeviceName, SessionDevices,
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
        let args = SessionJoinEvent::SessionJoinRequest(SessionJoinRequestParams {
            session_device_name,
            session_code,
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
            },
            SessionDevice {
                id: SessionDeviceId::new(2),
                name: SessionDeviceName::from(String::from("バイロン")),
            },
            SessionDevice {
                id: SessionDeviceId::new(3),
                name: SessionDeviceName::from(String::from("カルロー")),
            },
        ]);
        let args = SessionJoinEvent::SessionAccept(SessionAcceptResponse {
            session_code,
            session_device_id,
            session_devices,
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
