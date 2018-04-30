use std::collections::HashMap;

use amethyst::ui::FontHandle;

use FontVariant;

/// Application user interface theme.
#[derive(Constructor, Debug)]
pub struct Theme {
    /// Handles to the loaded fonts.
    pub fonts: HashMap<FontVariant, FontHandle>,
}
