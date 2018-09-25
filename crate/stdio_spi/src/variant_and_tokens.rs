use application_event::AppEventVariant;

/// Tuple with an `AppEventVariant` and the stdin tokens.
pub type VariantAndTokens = (AppEventVariant, Vec<String>);
