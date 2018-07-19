use amethyst::{
    animation::AnimationBundle,
    core::transform::TransformBundle,
    input::InputBundle,
    prelude::*,
    renderer::{Material, ScreenDimensions},
    ui::UiBundle,
};
use amethyst_test_support::{prelude::*, HIDPI, SCREEN_HEIGHT, SCREEN_WIDTH};
use character_selection::CharacterSelectionBundle;
use game_input::{PlayerActionControl, PlayerAxisControl};
use object_loading::ObjectLoadingBundle;
use object_model::config::object::CharacterSequenceId;

// Copied from `amethyst_test_support`
type StatePlaceholder = EmptyState;
type FnSetupPlaceholder = &'static fn(&mut World);
type FnStatePlaceholder = &'static fn() -> StatePlaceholder;
type FnEffectPlaceholder = &'static fn(&mut World);
type FnAssertPlaceholder = &'static fn(&mut World);

/// Baselines for building Amethyst applications with Autexousious types.
#[derive(Debug)]
pub struct AutexousiousApplication;

impl AutexousiousApplication {
    /// Returns an application with the Transform, Input, and UI bundles.
    ///
    /// This also adds a `ScreenDimensions` resource to the `World` so that UI calculations can be
    /// done.
    ///
    /// The difference between this and `AmethystApplication::base()` is the type parameters to the
    /// Input and UI bundles are the `PlayerAxisControl` and `PlayerActionControl`.
    pub fn base() -> AmethystApplication<
        StatePlaceholder,
        GameData<'static, 'static>,
        FnSetupPlaceholder,
        FnStatePlaceholder,
        FnEffectPlaceholder,
        FnAssertPlaceholder,
    > {
        AmethystApplication::blank()
            .with_bundle(TransformBundle::new())
            .with_bundle(InputBundle::<PlayerAxisControl, PlayerActionControl>::new())
            .with_bundle(UiBundle::<PlayerAxisControl, PlayerActionControl>::new())
            .with_resource(ScreenDimensions::new(SCREEN_WIDTH, SCREEN_HEIGHT, HIDPI))
    }

    /// Returns an application with the Animation, Transform, Input, UI, and Render bundles.
    ///
    /// The difference between this and `AmethystApplication::render_base()` is the type parameters
    /// to the Input and UI bundles are the `PlayerAxisControl` and `PlayerActionControl`, and the
    /// Animation bundle uses the object type sequence IDs for animation control sets.
    ///
    /// # Parameters
    ///
    /// * `test_name`: Name of the test, used to populate the window title.
    /// * `visibility`: Whether the window should be visible.
    pub fn render_base<'name, N>(
        test_name: N,
        visibility: bool,
    ) -> AmethystApplication<
        StatePlaceholder,
        GameData<'static, 'static>,
        FnSetupPlaceholder,
        FnStatePlaceholder,
        FnEffectPlaceholder,
        FnAssertPlaceholder,
    >
    where
        N: Into<&'name str>,
    {
        AmethystApplication::blank()
            .with_bundle(AnimationBundle::<CharacterSequenceId, Material>::new(
                "character_animation_control_system",
                "character_sampler_interpolation_system",
            ))
            .with_bundle(TransformBundle::new().with_dep(&[
                "character_animation_control_system",
                "character_sampler_interpolation_system",
            ]))
            .with_bundle(InputBundle::<PlayerAxisControl, PlayerActionControl>::new())
            .with_bundle(UiBundle::<PlayerAxisControl, PlayerActionControl>::new())
            .with_render_bundle(test_name, visibility)
    }

    /// Returns an application with the Animation, Transform, Input, UI, and Render bundles.
    ///
    /// The difference between this and `AmethystApplication::render_base()` is the type parameters
    /// to the Input and UI bundles are the `PlayerAxisControl` and `PlayerActionControl`, and the
    /// Animation bundle uses the object type sequence IDs for animation control sets.
    ///
    /// # Parameters
    ///
    /// * `test_name`: Name of the test, used to populate the window title.
    /// * `visibility`: Whether the window should be visible.
    pub fn game_base<'name, N>(
        test_name: N,
        visibility: bool,
    ) -> AmethystApplication<
        StatePlaceholder,
        GameData<'static, 'static>,
        FnSetupPlaceholder,
        FnStatePlaceholder,
        FnEffectPlaceholder,
        FnAssertPlaceholder,
    >
    where
        N: Into<&'name str>,
    {
        AmethystApplication::render_base(test_name, visibility)
            .with_bundle(ObjectLoadingBundle::new())
            .with_bundle(CharacterSelectionBundle::new())
    }
}

#[cfg(test)]
mod test {
    use amethyst::{input::InputHandler, ui::MouseReactive};
    use amethyst_test_support::MaterialAnimationFixture;
    use game_input::{PlayerActionControl, PlayerAxisControl};

    use super::AutexousiousApplication;

    // TODO: Allow users to specify their own type parameters to `AmethystApplication::base()`.
    //
    // This will make the dev experience better for crates that need strong types for the Input and
    // UI bundles, but are not able to depend on `AutexousiousApplication`, as the
    // `autexousious_test_support` crate would be depending on *that crate* (for better dev
    // experience for higher level crates).
    #[test]
    fn base_uses_strong_types_for_input_and_ui_bundles() {
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AutexousiousApplication::base()
                .with_assertion(|world| {
                    // Panics if the type parameters used are not these ones.
                    world.read_resource::<InputHandler<PlayerAxisControl, PlayerActionControl>>();
                    world.read_storage::<MouseReactive>();
                })
                .run()
                .is_ok()
        );
    }

    #[test]
    fn render_base_uses_strong_types_for_input_and_ui_bundles() {
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AutexousiousApplication::render_base(
                "render_base_uses_strong_types_for_input_and_ui_bundles",
                false
            ).with_assertion(|world| {
                // Panics if the type parameters used are not these ones.
                world.read_resource::<InputHandler<PlayerAxisControl, PlayerActionControl>>();
                world.read_storage::<MouseReactive>();
            })
                .run()
                .is_ok()
        );
    }

    #[test]
    fn render_base_application_can_load_material_animations() {
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AutexousiousApplication::render_base(
                "render_base_application_can_load_material_animations",
                false
            ).with_effect(MaterialAnimationFixture::effect)
                .with_assertion(MaterialAnimationFixture::assertion)
                .run()
                .is_ok()
        );
    }
}
