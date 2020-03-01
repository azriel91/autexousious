use amethyst::ui::UiTransform;
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

impl UiFovScaleTransform {
    /// Applies the scaling and translation to adjust to the new `ScreenDimensions`.
    ///
    /// # Parameters
    ///
    /// * `ui_transform`: The `UiTransform` to change.
    pub fn apply(self, ui_transform: &mut UiTransform) {
        ui_transform.local_x *= self.scale;
        ui_transform.local_x += self.x_offset;

        ui_transform.local_y *= self.scale;
        ui_transform.local_y += self.y_offset;

        ui_transform.width *= self.scale;
        ui_transform.height *= self.scale;
    }

    /// Undoes the scaling and translation from the previous `ScreenDimensions`.
    ///
    /// # Parameters
    ///
    /// * `ui_transform`: The `UiTransform` to revert.
    pub fn unapply(self, ui_transform: &mut UiTransform) {
        ui_transform.local_x -= self.x_offset;
        ui_transform.local_x /= self.scale;

        ui_transform.local_y -= self.y_offset;
        ui_transform.local_y /= self.scale;

        ui_transform.width /= self.scale;
        ui_transform.height /= self.scale;
    }
}
