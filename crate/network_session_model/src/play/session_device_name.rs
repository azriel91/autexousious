use std::{
    convert::Infallible,
    fmt::{self, Display, Formatter},
    str::FromStr,
};

use derive_new::new;
use serde::{Deserialize, Serialize};

/// Session device name (`String` newtype).
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, new)]
pub struct SessionDeviceName(pub String);

impl Display for SessionDeviceName {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl From<String> for SessionDeviceName {
    fn from(s: String) -> SessionDeviceName {
        SessionDeviceName(s)
    }
}

impl FromStr for SessionDeviceName {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Infallible> {
        Ok(SessionDeviceName::new(String::from(s)))
    }
}
