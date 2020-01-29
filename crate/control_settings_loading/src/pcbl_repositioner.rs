use camera_model::play::CameraZoomDimensions;
use control_settings_model::{config::ControlButtonLabel, loaded::PlayerControlButtonsLabels};
use kinematic_model::config::PositionInit;

/// `PlayerControlButtonsLabels` repositioner.
#[derive(Debug)]
pub struct PcblRepositioner;

impl PcblRepositioner {
    /// Shifts `ControlButtonLabel`s to begin at X: 0, and action buttons above axis buttons.
    pub fn reposition(player_control_buttons_labels: &mut PlayerControlButtonsLabels) {
        macro_rules! shift_min {
            ($control_button:ident, $pos:ident, $pos_min:ident) => {
                let $pos_min = player_control_buttons_labels
                    .$control_button
                    .values()
                    .map(|control_button_label| control_button_label.sprite.position.$pos)
                    .min();
                if let Some($pos_min) = $pos_min {
                    player_control_buttons_labels
                        .$control_button
                        .values_mut()
                        .for_each(|control_button_label| {
                            control_button_label.sprite.position.$pos -= $pos_min
                        });
                }
            };
        }

        shift_min!(axes, x, x_min);
        shift_min!(axes, y, y_min);
        shift_min!(actions, x, x_min);
        shift_min!(actions, y, y_min);

        // Shift action buttons above axes buttons.
        let y_max = player_control_buttons_labels
            .axes
            .values()
            .map(|control_button_label| control_button_label.sprite.position.y)
            .max();
        if let Some(y_max) = y_max {
            player_control_buttons_labels
                .actions
                .values_mut()
                .for_each(|control_button_label| {
                    // `y_max` is likely to be one sprite's height -- up arrow's bottom coordinate
                    // is above down arrow's sprite.
                    //
                    // Multiplying this by 3 allows gives us one sprite's gap between the up arrow
                    // and action buttons.
                    control_button_label.sprite.position.y += y_max * 3
                });
        }
    }

    /// Positions the control button labels on screen evenly.
    pub fn reposition_on_screen(
        camera_zoom_dimensions: CameraZoomDimensions,
        player_control_buttons_labelses: &mut [PlayerControlButtonsLabels],
    ) {
        // For a screen 800 pixels wide, we want to space the controls like so:
        //
        // |     ||     ||     ||     ||     |
        // 0     p0     p1     p2     p3     800
        //
        // * 5 gaps, so we divide 800 by 5, then skip the first coordinate.
        // * Then we must account for the width of each set of buttons, and make sure the midpoint
        //   is at each of those coordinates.
        let midpoint_distance = camera_zoom_dimensions.width as i32
            / (player_control_buttons_labelses.len() + 1) as i32;

        player_control_buttons_labelses
            .iter_mut()
            .map(Self::buttons_all_iter_mut)
            .map(Self::buttons_position_init_iter_mut)
            .enumerate()
            .for_each(|(index, buttons_position_init_iter)| {
                let x_shift = (index + 1) as i32 * midpoint_distance;

                buttons_position_init_iter.for_each(|position_init| position_init.x += x_shift);
            });
    }

    fn buttons_all_iter_mut<'f>(
        player_control_buttons_labels: &'f mut PlayerControlButtonsLabels,
    ) -> impl Iterator<Item = &mut ControlButtonLabel> + 'f {
        player_control_buttons_labels
            .axes
            .values_mut()
            .chain(player_control_buttons_labels.actions.values_mut())
    }

    fn buttons_position_init_iter_mut<'f>(
        buttons_all_iter: impl Iterator<Item = &'f mut ControlButtonLabel> + 'f,
    ) -> impl Iterator<Item = &'f mut PositionInit> + 'f {
        buttons_all_iter.map(Self::button_position_mut)
    }

    fn button_position_mut(control_button_label: &mut ControlButtonLabel) -> &mut PositionInit {
        &mut control_button_label.sprite.position
    }
}
