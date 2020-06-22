use std::convert::TryInto;

use bimap::BiMap;
use derive_deref::{Deref, DerefMut};
use derive_new::new;
use network_session_model::play::SessionCode;

use crate::model::SessionCodeId;

/// Bidirectional mappings from `SessionCode` to `SessionCodeId`.
///
/// This is used to prevent the need to clone `SessionCode`.
#[derive(Clone, Debug, Default, Deref, DerefMut, PartialEq, new)]
pub struct SessionCodeToId(pub BiMap<SessionCode, SessionCodeId>);

impl SessionCodeToId {
    /// Returns the `SessionCodeId` for the given `SessionCode`, generating a new one if it doesn't exist.
    pub fn code(&self, session_code_id: SessionCodeId) -> Option<&SessionCode> {
        self.0.get_by_right(&session_code_id)
    }

    /// Returns the `SessionCodeId` for the given `SessionCode`.
    ///
    /// If a new `SessionCodeId` should be generated, use the `id_or_default` method.
    pub fn id(&self, session_code: &SessionCode) -> Option<SessionCodeId> {
        self.0.get_by_left(session_code).copied()
    }

    /// Returns the `SessionCodeId` for the given `SessionCode`, generating a new one if it doesn't exist.
    pub fn id_or_default(&mut self, session_code: &SessionCode) -> SessionCodeId {
        self.id(session_code).unwrap_or_else(|| {
            let session_code_id: u64 = self
                .0
                .len()
                .try_into()
                .expect("Failed to convert `usize` to `u64` for `SessionCodeId`.");

            let session_code_id = SessionCodeId::new(session_code_id);
            self.0.insert(session_code.clone(), session_code_id);

            session_code_id
        })
    }
}
