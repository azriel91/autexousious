#[cfg(test)]
mod tests {
    use network_join_model::{
        play::{
            SessionAcceptResponse, SessionCode, SessionDevice, SessionDeviceId, SessionDeviceName,
            SessionDevices, SessionJoinRequestParams,
        },
        NetworkJoinEvent,
    };
    use stdio_spi::StdinMapper;

    use network_join_stdio::NetworkJoinEventStdinMapper;

    #[test]
    fn maps_session_join_request_event() {
        let session_device_name = SessionDeviceName::from(String::from("エイズリエル"));
        let session_code = SessionCode::from(String::from("abcd"));
        let args = NetworkJoinEvent::SessionJoinRequest(SessionJoinRequestParams {
            session_device_name,
            session_code,
        });

        let result = NetworkJoinEventStdinMapper::map(&(), args.clone());

        assert!(result.is_ok());
        assert_eq!(args, result.unwrap())
    }

    #[test]
    fn maps_join_cancel_event() {
        let args = NetworkJoinEvent::JoinCancel;

        let result = NetworkJoinEventStdinMapper::map(&(), args.clone());

        assert!(result.is_ok());
        assert_eq!(args, result.unwrap())
    }

    #[test]
    fn maps_session_accept_event() {
        let session_code = SessionCode::from(String::from("abcd"));
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
        let args = NetworkJoinEvent::SessionAccept(SessionAcceptResponse {
            session_code,
            session_devices,
        });

        let result = NetworkJoinEventStdinMapper::map(&(), args.clone());

        assert!(result.is_ok());
        assert_eq!(args, result.unwrap())
    }

    #[test]
    fn maps_back_event() {
        let args = NetworkJoinEvent::Back;

        let result = NetworkJoinEventStdinMapper::map(&(), args.clone());

        assert!(result.is_ok());
        assert_eq!(args, result.unwrap())
    }
}
