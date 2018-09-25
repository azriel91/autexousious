use clap;

/// Kinds of errors when handling stdio logic.
#[derive(Debug, ErrorChain)]
pub enum ErrorKind {
    /// Plain error message without additional structure or context.
    Msg(String),

    /// Error when failing to parse stdin tokens to state specific event arguments.
    #[error_chain(foreign)]
    Clap(clap::Error),
}
