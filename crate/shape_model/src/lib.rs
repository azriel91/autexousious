#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Types used to represent shapes.


#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate strum_macros;

pub use crate::axis::Axis;
pub use crate::volume::Volume;

mod axis;
mod volume;
