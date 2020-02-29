#[cfg(test)]
mod tests {
    use std::net::{Ipv4Addr, SocketAddr};

    use amethyst::{
        ecs::{Read, WorldExt},
        Error,
    };
    use amethyst_test::AmethystApplication;
    use net_model::play::{NetEvent, NetEventChannel};
    use network_session_model::play::{
        Session, SessionCode, SessionDevice, SessionDeviceId, SessionDeviceName, SessionDevices,
        SessionStatus,
    };
    use session_join_model::{play::SessionAcceptResponse, SessionJoinEvent};

    use session_join_play::SessionJoinResponseSystemDesc;

    #[test]
    fn does_nothing_when_no_session_join_event() -> Result<(), Error> {
        run_test(
            SetupParams {
                session_code: SessionCode::new(String::from("abcd")),
                session_device_id: SessionDeviceId::new(123),
                session_devices: SessionDevices::new(vec![]),
                session_status: SessionStatus::None,
                session_join_event: None,
            },
            ExpectedParams {
                session_code: SessionCode::new(String::from("abcd")),
                session_device_id: SessionDeviceId::new(123),
                session_devices: SessionDevices::new(vec![]),
                session_status: SessionStatus::None,
            },
        )
    }

    #[test]
    fn inserts_resources_on_session_accepted() -> Result<(), Error> {
        run_test(
            SetupParams {
                session_code: SessionCode::new(String::from("abcd")),
                session_device_id: SessionDeviceId::new(123),
                session_devices: SessionDevices::new(vec![]),
                session_status: SessionStatus::JoinRequested {
                    session_code: SessionCode::new(String::from("defg")),
                },
                session_join_event: Some(SessionJoinEvent::SessionAccept(SessionAcceptResponse {
                    session_device_id: SessionDeviceId::new(234),
                    session: Session {
                        session_code: SessionCode::new(String::from("defg")),
                        session_devices: SessionDevices::new(vec![SessionDevice::new(
                            SessionDeviceId::new(234),
                            SessionDeviceName::new(String::from("azriel")),
                        )]),
                    },
                })),
            },
            ExpectedParams {
                session_code: SessionCode::new(String::from("defg")),
                session_device_id: SessionDeviceId::new(234),
                session_devices: SessionDevices::new(vec![SessionDevice::new(
                    SessionDeviceId::new(234),
                    SessionDeviceName::new(String::from("azriel")),
                )]),
                session_status: SessionStatus::JoinEstablished,
            },
        )
    }

    #[test]
    fn ignores_session_accept_event_when_no_longer_waiting() -> Result<(), Error> {
        run_test(
            SetupParams {
                session_code: SessionCode::new(String::from("abcd")),
                session_device_id: SessionDeviceId::new(123),
                session_devices: SessionDevices::new(vec![]),
                session_status: SessionStatus::None,
                session_join_event: Some(SessionJoinEvent::SessionAccept(SessionAcceptResponse {
                    session_device_id: SessionDeviceId::new(234),
                    session: Session {
                        session_code: SessionCode::new(String::from("defg")),
                        session_devices: SessionDevices::new(vec![SessionDevice::new(
                            SessionDeviceId::new(234),
                            SessionDeviceName::new(String::from("azriel")),
                        )]),
                    },
                })),
            },
            ExpectedParams {
                session_code: SessionCode::new(String::from("abcd")),
                session_device_id: SessionDeviceId::new(123),
                session_devices: SessionDevices::new(vec![]),
                session_status: SessionStatus::None,
            },
        )
    }

    fn run_test(
        SetupParams {
            session_code: session_code_setup,
            session_device_id: session_device_id_setup,
            session_devices: session_devices_setup,
            session_status: session_status_setup,
            session_join_event,
        }: SetupParams,
        ExpectedParams {
            session_code: session_code_expected,
            session_device_id: session_device_id_expected,
            session_devices: session_devices_expected,
            session_status: session_status_expected,
        }: ExpectedParams,
    ) -> Result<(), Error> {
        AmethystApplication::blank()
            .with_system_desc(SessionJoinResponseSystemDesc::default(), "", &[])
            .with_setup(move |world| {
                world.insert(session_code_setup);
                world.insert(session_device_id_setup);
                world.insert(session_devices_setup);
                world.insert(session_status_setup);
            })
            .with_effect(move |world| {
                if let Some(session_join_event) = session_join_event {
                    let socket_addr = SocketAddr::from((Ipv4Addr::LOCALHOST, 1234));
                    world
                        .write_resource::<NetEventChannel<SessionJoinEvent>>()
                        .single_write(NetEvent {
                            socket_addr,
                            event: session_join_event,
                        });
                }
            })
            .with_assertion(move |world| {
                let (session_code, session_device_id, session_devices, session_status) = world
                    .system_data::<(
                        Read<'_, SessionCode>,
                        Read<'_, SessionDeviceId>,
                        Read<'_, SessionDevices>,
                        Read<'_, SessionStatus>,
                    )>();

                let (session_code, session_device_id, session_devices, session_status) = (
                    &*session_code,
                    &*session_device_id,
                    &*session_devices,
                    &*session_status,
                );

                assert_eq!(
                    (
                        &session_code_expected,
                        &session_device_id_expected,
                        &session_devices_expected,
                        &session_status_expected
                    ),
                    (
                        session_code,
                        session_device_id,
                        session_devices,
                        session_status
                    )
                );
            })
            .run()
    }

    struct SetupParams {
        session_code: SessionCode,
        session_device_id: SessionDeviceId,
        session_devices: SessionDevices,
        session_status: SessionStatus,
        session_join_event: Option<SessionJoinEvent>,
    }

    struct ExpectedParams {
        session_code: SessionCode,
        session_device_id: SessionDeviceId,
        session_devices: SessionDevices,
        session_status: SessionStatus,
    }
}
