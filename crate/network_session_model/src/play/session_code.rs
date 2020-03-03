use std::{
    convert::Infallible,
    fmt::{self, Display, Formatter},
    str::FromStr,
};

use derive_new::new;
use serde::{Deserialize, Serialize};

/// Session code (`String` newtype).
#[derive(Clone, Debug, Default, Deserialize, Hash, PartialEq, Eq, Serialize, new)]
pub struct SessionCode(pub String);

impl Display for SessionCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl From<String> for SessionCode {
    fn from(s: String) -> SessionCode {
        SessionCode(s)
    }
}

impl FromStr for SessionCode {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Infallible> {
        Ok(SessionCode::new(String::from(s)))
    }
}
