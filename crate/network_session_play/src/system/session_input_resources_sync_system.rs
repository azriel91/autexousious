use amethyst::{
    derive::SystemDesc,
    ecs::{Read, System, World, Write},
    input::InputHandler,
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use derivative::Derivative;
use derive_new::new;
use game_input_model::{
    config::{ControlBindings, PlayerInputConfigs},
    loaded::PlayerControllers,
    play::ControllerIdOffset,
};
use log::error;
use network_session_model::{play::SessionStatus, SessionStatusEvent};

/// Rectifies `PlayerController`s and input `Bindings` when switching between local and online play.
#[derive(Debug, SystemDesc, new)]
#[system_desc(name(SessionInputResourcesSyncSystemDesc))]
pub struct SessionInputResourcesSyncSystem {
    /// Reader ID for the `SessionStatusEvent` channel.
    #[system_desc(event_channel_reader)]
    session_status_event_rid: ReaderId<SessionStatusEvent>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct SessionInputResourcesSyncSystemData<'s> {
    /// `SessionStatusEvent` channel.
    #[derivative(Debug = "ignore")]
    pub session_status_ec: Read<'s, EventChannel<SessionStatusEvent>>,
    /// `SessionStatus` resource.
    #[derivative(Debug = "ignore")]
    pub session_status: Read<'s, SessionStatus>,
    /// `PlayerInputConfigs` resource.
    #[derivative(Debug = "ignore")]
    pub player_input_configs: Read<'s, PlayerInputConfigs>,
    /// `ControllerIdOffset` resource.
    #[derivative(Debug = "ignore")]
    pub controller_id_offset: Read<'s, ControllerIdOffset>,
    /// `InputHandler<ControlBindings>` resource.
    #[derivative(Debug = "ignore")]
    pub input_handler: Write<'s, InputHandler<ControlBindings>>,
    /// `PlayerControllers` resource.
    #[derivative(Debug = "ignore")]
    pub player_controllers: Write<'s, PlayerControllers>,
}

impl SessionInputResourcesSyncSystem {
    fn update_input_bindings(
        player_input_configs: &PlayerInputConfigs,
        input_handler: &mut InputHandler<ControlBindings>,
        controller_id_offset: ControllerIdOffset,
    ) {
        // Update `PlayerAxisControl`s and `PlayerActionControl`s in `Bindings`
        match player_input_configs.generate_bindings(controller_id_offset) {
            Ok(bindings) => input_handler.bindings = bindings,
            Err(e) => {
                error!(
                    "Failed to update input `Bindings`. \
                    Players may control incorrect characters. Error: {}",
                    e
                );
            }
        }
    }
}

impl<'s> System<'s> for SessionInputResourcesSyncSystem {
    type SystemData = SessionInputResourcesSyncSystemData<'s>;

    fn run(
        &mut self,
        SessionInputResourcesSyncSystemData {
            session_status_ec,
            session_status,
            player_input_configs,
            controller_id_offset,
            mut input_handler,
            mut player_controllers,
        }: Self::SystemData,
    ) {
        let session_status_changed = session_status_ec
            .read(&mut self.session_status_event_rid)
            .next()
            .is_some();
        if session_status_changed {
            match *session_status {
                SessionStatus::None => {
                    Self::update_input_bindings(
                        &player_input_configs,
                        &mut input_handler,
                        *controller_id_offset,
                    );

                    // Reload `PlayerControllers` from configuration.
                    *player_controllers = PlayerControllers::from(&*player_input_configs);
                }
                SessionStatus::JoinEstablished | SessionStatus::HostEstablished => {
                    Self::update_input_bindings(
                        &player_input_configs,
                        &mut input_handler,
                        *controller_id_offset,
                    );
                }
                _ => {}
            }
        }
    }
}
