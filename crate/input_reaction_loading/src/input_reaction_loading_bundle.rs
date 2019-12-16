use amethyst::{
    assets::Processor,
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, World},
    Error,
};
use derive_new::new;
use input_reaction_model::loaded::{InputReactions, InputReactionsSequence};

/// Adds the following processor `System`s to the world:
///
/// * `Processor::<InputReactions>`
/// * `Processor::<InputReactionsSequence>`
#[derive(Debug, new)]
pub struct InputReactionLoadingBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for InputReactionLoadingBundle {
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        builder.add(
            Processor::<InputReactions>::new(),
            "input_reactions_processor",
            &[],
        ); // kcov-ignore
        builder.add(
            Processor::<InputReactionsSequence>::new(),
            "input_reactions_sequence_processor",
            &["input_reactions_processor"],
        ); // kcov-ignore
        Ok(())
    }
}
