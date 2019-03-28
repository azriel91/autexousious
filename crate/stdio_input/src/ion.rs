#![allow(missing_docs)] // These are from [ion](https://gitlab.redox-os.org/redox-os/ion)

pub(crate) use self::{
    quotes::Terminator,
    splitter::{StatementSplitter, StatementVariant},
};

mod quotes;
mod splitter;
