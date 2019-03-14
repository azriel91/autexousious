/// Length to render the `HpBar`.
pub const HP_BAR_LENGTH: f32 = 100.;
/// Height to render the `HpBar`.
pub const HP_BAR_HEIGHT: f32 = 3.;
/// Number of sprites on the `HpBar` sprite sheet.
pub const HP_BAR_SPRITE_COUNT: usize = 20;
/// Width and height of sprites on the `HpBar` sprite sheet.
///
/// This is here because if we use 1x1 pixels, the adjacent transparent border will cause the
/// `HpBar` to have a "fade" effect, which, although aesthetic, isn't intended right now.
pub const HP_BAR_SPRITE_SIZE: u32 = 25;
