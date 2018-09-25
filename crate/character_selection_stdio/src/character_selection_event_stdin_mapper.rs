use character_selection_model::{CharacterSelection, CharacterSelectionEvent};
use game_model::{
    config::AssetSlugBuilder,
    loaded::{CharacterAssets, SlugAndHandle},
};
use stdio_spi::{Result, StdinMapper};

use CharacterSelectionEventArgs;

/// Builds a `CharacterSelectionEvent` from stdin tokens.
#[derive(Debug, TypeName)]
pub struct CharacterSelectionEventStdinMapper;

impl StdinMapper for CharacterSelectionEventStdinMapper {
    type Resource = CharacterAssets;
    type Event = CharacterSelectionEvent;
    type Args = CharacterSelectionEventArgs;

    fn map(character_assets: &CharacterAssets, args: Self::Args) -> Result<Self::Event> {
        let slug = AssetSlugBuilder::default()
            .namespace(args.namespace)
            .name(args.name)
            .build()?;

        let handle = character_assets
            .get(&slug)
            .ok_or(format!("No character found with asset slug `{}`.", slug))?
            .clone();

        let snh = SlugAndHandle { slug, handle };
        let character_selection = CharacterSelection::Id(snh);
        let character_selection_event = CharacterSelectionEvent::Select {
            controller_id: 0,
            character_selection,
        };

        Ok(character_selection_event)
    }
}
