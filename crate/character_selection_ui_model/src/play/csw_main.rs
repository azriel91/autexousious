use amethyst::{
    ecs::{storage::NullStorage, Component, Entity, World, WriteStorage},
    shred::{ResourceId, SystemData},
};
use asset_model::ItemComponent;
use character_selection_model::CharacterSelection;
use derivative::Derivative;

/// Marks an entity as the main character selection widget entity.
#[derive(Clone, Component, Copy, Debug, Default, PartialEq)]
#[storage(NullStorage)]
pub struct CswMain;

/// `CswMainSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct CswMainSystemData<'s> {
    /// `CharacterSelection` components.
    #[derivative(Debug = "ignore")]
    pub character_selections: WriteStorage<'s, CharacterSelection>,
}

impl<'s> ItemComponent<'s> for CswMain {
    type SystemData = CswMainSystemData<'s>;

    fn augment(&self, system_data: &mut Self::SystemData, entity: Entity) {
        let CswMainSystemData {
            character_selections,
        } = system_data;

        if character_selections.get(entity).is_none() {
            character_selections
                .insert(entity, CharacterSelection::Random)
                .expect("Failed to insert `CharacterSelection` component.");
        }
    }
}
