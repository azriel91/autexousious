use derive_new::new;

/// Scale and transform values to adjust UI components when the window is resized.
#[derive(Clone, Copy, Debug, PartialEq, new)]
pub struct UiFovScaleTransform {
    /// Scale to apply to UI components.
    ///
    /// This should be used to multiply:
    ///
    /// * `UiText.font_size`
    /// * `UiTransform.{x,y,width,height}`
    pub scale: f32,
    /// Units to shift UI components on the X axis.
    ///
    /// The shift should be applied after coordinates have been scaled.
    pub x_offset: f32,
    /// Units to shift UI components on the Y axis.
    ///
    /// The shift should be applied after coordinates have been scaled.
    pub y_offset: f32,
}

impl Default for UiFovScaleTransform {
    fn default() -> Self {
        UiFovScaleTransform {
            scale: 1.,
            x_offset: 0.,
            y_offset: 0.,
        }
    }
}
