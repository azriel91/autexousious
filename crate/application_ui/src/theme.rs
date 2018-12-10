use std::collections::HashMap;

use amethyst::ui::FontHandle;
use derive_new::new;

use crate::FontVariant;

/// Application user interface theme.
#[derive(Debug, new)]
pub struct Theme {
    /// Handles to the loaded fonts.
    pub fonts: HashMap<FontVariant, FontHandle>,
}
