use derive_new::new;
use network_session_model::play::SessionCode;
use serde::{Deserialize, Serialize};
use structopt_derive::StructOpt;

/// Parameters required to start a session.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, StructOpt, new)]
pub struct SessionStartRequestParams {
    /// Code of the session.
    #[structopt(long)]
    pub session_code: SessionCode,
}
