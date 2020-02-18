#[cfg(test)]
mod tests {
    use amethyst::{
        ecs::{Read, WorldExt},
        shrev::EventChannel,
        Error,
    };
    use amethyst_test::AmethystApplication;
    use network_join_model::{play::SessionAcceptResponse, NetworkJoinEvent};
    use network_session_model::play::{
        SessionCode, SessionDevice, SessionDeviceId, SessionDeviceName, SessionDevices,
        SessionStatus,
    };

    use network_join_play::SessionJoinAcceptedSystemDesc;

    #[test]
    fn does_nothing_when_no_network_join_event() -> Result<(), Error> {
        run_test(
            SetupParams {
                session_code: SessionCode::new(String::from("abcd")),
                session_device_id: SessionDeviceId::new(123),
                session_devices: SessionDevices::new(vec![]),
                session_status: SessionStatus::None,
                network_join_event: None,
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
                session_status: SessionStatus::None,
                network_join_event: Some(NetworkJoinEvent::SessionAccept(SessionAcceptResponse {
                    session_code: SessionCode::new(String::from("defg")),
                    session_device_id: SessionDeviceId::new(234),
                    session_devices: SessionDevices::new(vec![SessionDevice::new(
                        SessionDeviceId::new(234),
                        SessionDeviceName::new(String::from("azriel")),
                    )]),
                })),
            },
            ExpectedParams {
                session_code: SessionCode::new(String::from("defg")),
                session_device_id: SessionDeviceId::new(234),
                session_devices: SessionDevices::new(vec![SessionDevice::new(
                    SessionDeviceId::new(234),
                    SessionDeviceName::new(String::from("azriel")),
                )]),
                session_status: SessionStatus::Established,
            },
        )
    }

    fn run_test(
        SetupParams {
            session_code: session_code_setup,
            session_device_id: session_device_id_setup,
            session_devices: session_devices_setup,
            session_status: session_status_setup,
            network_join_event,
        }: SetupParams,
        ExpectedParams {
            session_code: session_code_expected,
            session_device_id: session_device_id_expected,
            session_devices: session_devices_expected,
            session_status: session_status_expected,
        }: ExpectedParams,
    ) -> Result<(), Error> {
        AmethystApplication::blank()
            .with_system_desc(SessionJoinAcceptedSystemDesc::default(), "", &[])
            .with_setup(move |world| {
                world.insert(session_code_setup);
                world.insert(session_device_id_setup);
                world.insert(session_devices_setup);
                world.insert(session_status_setup);
            })
            .with_effect(move |world| {
                if let Some(network_join_event) = network_join_event {
                    world
                        .write_resource::<EventChannel<NetworkJoinEvent>>()
                        .single_write(network_join_event);
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
        network_join_event: Option<NetworkJoinEvent>,
    }

    struct ExpectedParams {
        session_code: SessionCode,
        session_device_id: SessionDeviceId,
        session_devices: SessionDevices,
        session_status: SessionStatus,
    }
}
