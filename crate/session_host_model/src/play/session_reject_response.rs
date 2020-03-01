use derive_new::new;
use serde::{Deserialize, Serialize};
use structopt_derive::StructOpt;

/// Response when a session host request is rejected.
///
/// We should also include a reason.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, StructOpt, new)]
pub struct SessionRejectResponse {}
