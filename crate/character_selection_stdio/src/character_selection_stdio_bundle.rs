use amethyst::{
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, World},
    Error,
};
use application_event::AppEventVariant;
use derive_new::new;
use stdio_spi::MapperSystem;
use typename::TypeName;

use crate::CharacterSelectionEventStdinMapper;

/// Adds a `MapperSystem<CharacterSelectionEventStdinMapper>` to the `World`.
#[derive(Debug, new)]
pub struct CharacterSelectionStdioBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for CharacterSelectionStdioBundle {
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        builder.add(
            MapperSystem::<CharacterSelectionEventStdinMapper>::new(
                AppEventVariant::CharacterSelection,
            ),
            &MapperSystem::<CharacterSelectionEventStdinMapper>::type_name(),
            &[],
        ); // kcov-ignore
        Ok(())
    }
}
