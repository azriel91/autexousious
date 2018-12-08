//! Provides support for programmatic `State` tracking and manipulation.

pub use self::{
    intercept::{ApplicationEventIntercept, FixedTimeoutIntercept, Intercept},
    robot_state::RobotState,
};

mod intercept;
mod robot_state;
