use derive_new::new;
use game_input_model::loaded::PlayerControllers;
use network_session_model::play::{Session, SessionDeviceId};
use serde::{Deserialize, Serialize};
use structopt_derive::StructOpt;

/// Response when a session host request is accepted.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, StructOpt, new)]
pub struct SessionAcceptResponse {
    // Structopt actually disallows us to have docs on this. `._.`
    //
    // Session information.
    //
    // This includes the session hoster's device.
    #[allow(missing_docs)]
    #[structopt(flatten)]
    pub session: Session,
    /// ID that the server generated for the session hoster.
    #[structopt(long)]
    pub session_device_id: SessionDeviceId,
    /// All player controllers.
    #[structopt(long)]
    pub player_controllers: PlayerControllers,
}
