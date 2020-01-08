use amethyst::{
    ecs::{Read, System, World, Write},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use asset_model::{config::AssetType, loaded::AssetTypeMappings};
use character_selection_model::{CharacterSelection, CharacterSelectionEvent, CharacterSelections};
use derivative::Derivative;
use derive_new::new;
use object_type::ObjectType;

/// Populates the `CharacterSelections` based on user input.
#[derive(Debug, Default, new)]
pub struct CharacterSelectionSystem {
    /// Reader ID for the `CharacterSelectionEvent` event channel.
    #[new(default)]
    character_selection_event_rid: Option<ReaderId<CharacterSelectionEvent>>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct CharacterSelectionSystemData<'s> {
    /// `CharacterSelectionEvent` channel.
    #[derivative(Debug = "ignore")]
    pub character_selection_ec: Read<'s, EventChannel<CharacterSelectionEvent>>,
    /// `AssetTypeMappings` resource.
    #[derivative(Debug = "ignore")]
    pub asset_type_mappings: Read<'s, AssetTypeMappings>,
    /// `CharacterSelections` resource.
    #[derivative(Debug = "ignore")]
    pub character_selections: Write<'s, CharacterSelections>,
}

impl<'s> System<'s> for CharacterSelectionSystem {
    type SystemData = CharacterSelectionSystemData<'s>;

    fn run(
        &mut self,
        CharacterSelectionSystemData {
            character_selection_ec,
            asset_type_mappings,
            mut character_selections,
        }: Self::SystemData,
    ) {
        character_selection_ec
            .read(
                self.character_selection_event_rid
                    .as_mut()
                    .expect("Expected `character_selection_event_rid` to be set."),
            )
            .for_each(|ev| match ev {
                CharacterSelectionEvent::Select {
                    controller_id,
                    character_selection,
                } => {
                    let asset_id = match character_selection {
                        CharacterSelection::Id(asset_id) => *asset_id,
                        CharacterSelection::Random => {
                            // TODO: Implement Random
                            // TODO: <https://gitlab.com/azriel91/autexousious/issues/137>
                            asset_type_mappings
                                .iter_ids(&AssetType::Object(ObjectType::Character))
                                .next()
                                .copied()
                                .expect("Expected at least one character to be loaded.")
                        }
                    };
                    character_selections
                        .selections
                        .insert(*controller_id, asset_id);
                }
                CharacterSelectionEvent::Deselect { controller_id } => {
                    character_selections.selections.remove(&controller_id);
                }
                _ => {}
            });
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.character_selection_event_rid = Some(
            world
                .fetch_mut::<EventChannel<CharacterSelectionEvent>>()
                .register_reader(),
        );
    }
}
