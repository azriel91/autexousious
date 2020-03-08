use derive_new::new;
use network_session_model::play::SessionCode;
use serde::{Deserialize, Serialize};
use structopt_derive::StructOpt;

use crate::play::SessionJoinError;

/// Response when a session join request is rejected.
///
/// We should also include a reason.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, StructOpt, new)]
pub struct SessionRejectResponse {
    /// Code of the session.
    #[structopt(long)]
    pub session_code: SessionCode,
    /// Session join rejection reason.
    #[structopt(long)]
    pub session_join_error: SessionJoinError,
}
