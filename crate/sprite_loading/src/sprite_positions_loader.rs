use asset_model::loaded::AssetId;
use derivative::Derivative;
use sequence_loading_spi::SequenceComponentDataLoader;
use sprite_model::{
    config::SpritePosition,
    loaded::{AssetSpritePositions, SpritePositions},
};

/// Loads `SpritePosition`s from sequences.
#[derive(Derivative)]
#[derivative(Debug)]
pub struct SpritePositionsLoader<'s> {
    /// `AssetSpritePositions`.
    pub asset_sprite_positions: &'s mut AssetSpritePositions,
}

impl<'s> SpritePositionsLoader<'s> {
    /// Loads `SpritePositions`.
    ///
    /// This is similar to calling the `SequenceComponentDataLoader::load` trait method, with the
    /// difference that the resources are stored by an instantiation of this type, so they do not
    /// need to be passed in when this method is called.
    pub fn load<SequencesIterator, SequenceRef>(
        &mut self,
        sequences_iterator: SequencesIterator,
        asset_id: AssetId,
    ) where
        SequencesIterator: Iterator<Item = SequenceRef>,
        SequenceRef: AsRef<SpritePosition>,
    {
        let sprite_positions = <Self as SequenceComponentDataLoader>::load(
            |sequence_ref| *AsRef::<SpritePosition>::as_ref(&sequence_ref),
            sequences_iterator,
        );
        self.asset_sprite_positions
            .insert(asset_id, sprite_positions);
    }
}

impl<'s> SequenceComponentDataLoader for SpritePositionsLoader<'s> {
    type Component = SpritePosition;
    type ComponentData = SpritePositions;
}
