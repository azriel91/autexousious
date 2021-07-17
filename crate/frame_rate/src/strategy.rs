//! Frame Rate Limit Strategies

use std::time::Duration;

use amethyst::core::frame_limiter::{FrameRateLimitConfig, FrameRateLimitStrategy};

// === Frame Limit Configurations === //
//
// These are the high level types passed to Amethyst.

/// Returns the `FrameRateLimitConfig` to use for the application.
pub fn frame_rate_limit_config(frame_rate: Option<u32>) -> FrameRateLimitConfig {
    frame_rate
        .map(|frame_rate| FrameRateLimitConfig {
            strategy: SLEEP_AND_YIELD,
            fps: frame_rate,
        })
        .unwrap_or(FRAME_RATE_DEFAULT)
}

/// Use the "normal" frame rate of 60 FPS.
pub const FRAME_RATE_DEFAULT: FrameRateLimitConfig = FrameRateLimitConfig {
    strategy: SLEEP_AND_YIELD,
    fps: FPS_DEFAULT,
};

/// Use the double frame rate of 120 FPS.
pub const FRAME_RATE_DOUBLE: FrameRateLimitConfig = FrameRateLimitConfig {
    strategy: SLEEP_AND_YIELD,
    fps: FPS_DOUBLE,
};

/// Don't limit the frame rate -- run as fast as possible.
pub const FRAME_RATE_NO_LIMIT: FrameRateLimitConfig = FrameRateLimitConfig {
    strategy: FrameRateLimitStrategy::Yield,
    fps: FPS_NO_LIMIT,
};

// === FPS numeric constants === //

/// Default number of frames per second to limit the game to.
pub const FPS_DEFAULT: u32 = 60;
/// Double the default number of frames per second.
pub const FPS_DOUBLE: u32 = FPS_DEFAULT * 2;
/// "FPS limit" for the non-limited frame rate limit.
pub const FPS_NO_LIMIT: u32 = 9999;

// === Frame Rate "sleep" strategies === //

/// Sleep until the given duration remains, then yield.
///
/// At 60 FPS, we have 16 ms per frame, so we should be able to risk not
/// spin-looping a thread until the last 1 ms. At 120 FPS, we have 8 ms per
/// frame, and the error margin increases, but should not be too high to cause
/// an inconsistent frame rate.
pub const SLEEP_AND_YIELD: FrameRateLimitStrategy =
    FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(1));
