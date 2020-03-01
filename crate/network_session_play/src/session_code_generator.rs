use derive_new::new;
use network_session_model::play::SessionCode;
use rand::{
    distributions::{Distribution, Uniform},
    rngs::StdRng,
    SeedableRng,
};

/// Number of characters in the session code.
const SESSION_CODE_LENGTH: usize = 4;

/// Generates session codes for sessions.
#[derive(Debug, new)]
pub struct SessionCodeGenerator {
    /// Range of characters to sample from.
    #[new(value = "Uniform::new_inclusive('A' as u32, 'Z' as u32)")]
    character_range: Uniform<u32>,
    /// The random number generator to use.
    rng: StdRng,
}

impl Default for SessionCodeGenerator {
    fn default() -> Self {
        SessionCodeGenerator {
            character_range: Uniform::new_inclusive('A' as u32, 'Z' as u32),
            rng: StdRng::from_entropy(),
        }
    }
}

impl SessionCodeGenerator {
    /// Returns a randomly generated session code with the given length.
    pub fn generate(&mut self) -> SessionCode {
        let code = self
            .character_range
            .sample_iter(&mut self.rng)
            .take(SESSION_CODE_LENGTH)
            .map(|c_u32| unsafe { std::char::from_u32_unchecked(c_u32) })
            .collect::<String>();

        SessionCode::new(code)
    }
}
