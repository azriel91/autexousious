use derive_new::new;
use game_input_model::{config::ControllerId, loaded::PlayerControllers};
use network_session_model::play::{Session, SessionDeviceId};
use serde::{Deserialize, Serialize};
use structopt_derive::StructOpt;

/// Response when a session join request is accepted.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, StructOpt, new)]
pub struct SessionAcceptResponse {
    // Structopt actually disallows us to have docs on this. `._.`
    //
    // Session information.
    //
    // This includes the session joiner's device.
    #[allow(missing_docs)]
    #[structopt(flatten)]
    pub session: Session,
    /// ID that the server generated for the session joiner.
    #[structopt(long)]
    pub session_device_id: SessionDeviceId,
    /// All player controllers.
    #[structopt(long)]
    pub player_controllers: PlayerControllers,
    /// Offset to use for local `ControllerId`s.
    #[structopt(long)]
    pub controller_id_offset: ControllerId,
}
