#[cfg(test)]
mod tests {
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
        let session_code = SessionCode::from(String::from("abcd"));
        let args = SessionHostEvent::SessionHostRequest(SessionHostRequestParams {
            session_device_name,
            session_code,
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
