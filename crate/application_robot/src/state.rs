//! Provides support for programmatic `State` tracking and manipulation.

pub use self::intercept::{ApplicationEventIntercept, FixedTimeoutIntercept, Intercept};
pub use self::robot_state::RobotState;

mod intercept;
mod robot_state;
