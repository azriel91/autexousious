use amethyst::{
    ecs::{Read, System, World, Write},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use asset_model::{config::AssetType, loaded::AssetTypeMappings};
use asset_selection_model::play::{AssetSelection, AssetSelectionEvent};
use character_selection_model::CharacterSelections;
use derivative::Derivative;
use derive_new::new;
use log::warn;
use object_type::ObjectType;

/// Populates the `CharacterSelections` based on user input.
#[derive(Debug, Default, new)]
pub struct CharacterSelectionSystem {
    /// Reader ID for the `AssetSelectionEvent` event channel.
    #[new(default)]
    asset_selection_event_rid: Option<ReaderId<AssetSelectionEvent>>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct CharacterSelectionSystemData<'s> {
    /// `AssetSelectionEvent` channel.
    #[derivative(Debug = "ignore")]
    pub asset_selection_ec: Read<'s, EventChannel<AssetSelectionEvent>>,
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
            asset_selection_ec,
            asset_type_mappings,
            mut character_selections,
        }: Self::SystemData,
    ) {
        asset_selection_ec
            .read(
                self.asset_selection_event_rid
                    .as_mut()
                    .expect("Expected `asset_selection_event_rid` to be set."),
            )
            .copied()
            .for_each(|ev| match ev {
                AssetSelectionEvent::Select {
                    controller_id,
                    asset_selection,
                    ..
                } => {
                    let asset_id = match asset_selection {
                        AssetSelection::Id(asset_id) => asset_id,
                        AssetSelection::Random => {
                            // TODO: Implement Random
                            // TODO: <https://gitlab.com/azriel91/autexousious/issues/137>
                            asset_type_mappings
                                .iter_ids(&AssetType::Object(ObjectType::Character))
                                .next()
                                .copied()
                                .expect("Expected at least one character to be loaded.")
                        }
                    };

                    let asset_type = asset_type_mappings.get(asset_id);
                    if let Some(AssetType::Object(ObjectType::Character)) = asset_type {
                        character_selections
                            .selections
                            .insert(controller_id, asset_id);
                    } else {
                        warn!(
                            "Received `AssetSelectionEvent` for {:?} which has type: {:?} in \
                            `CharacterSelectionSystem`",
                            asset_id, asset_type
                        );
                    }
                }
                AssetSelectionEvent::Deselect { controller_id, .. } => {
                    character_selections.selections.remove(&controller_id);
                }
                _ => {}
            });
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.asset_selection_event_rid = Some(
            world
                .fetch_mut::<EventChannel<AssetSelectionEvent>>()
                .register_reader(),
        );
    }
}
