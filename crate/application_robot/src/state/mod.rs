//! Provides support for programmatic `State` tracking and manipulation.

pub use self::robot_state::{RobotState, RobotStateBuilder};
pub use self::intercept::{ApplicationEventIntercept, FixedTimeoutIntercept, Intercept};

mod robot_state;
mod intercept;
