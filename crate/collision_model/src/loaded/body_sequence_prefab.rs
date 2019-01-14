use amethyst::{
    assets::{AssetStorage, Handle, Loader, PrefabData, ProgressCounter},
    ecs::{Entity, Read, ReadExpect},
    Error,
};
use serde::{Deserialize, Serialize};

use crate::{
    animation::{BodyAnimationFrame, BodyAnimationSequence},
    config::BodyFrame,
    loaded::BodySequence,
};

/// Sequence for volumes that can be hit.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum BodySequencePrefab<S>
where
    S: BodyAnimationSequence,
{
    /// Prefab data is part of `S`, and needs to be loaded.
    Object(S),
    /// Already loaded handle.
    #[serde(skip)]
    Handle(Handle<BodySequence>),
}

impl<'s, S> PrefabData<'s> for BodySequencePrefab<S>
where
    S: BodyAnimationSequence,
{
    type SystemData = (
        ReadExpect<'s, Loader>,
        Read<'s, AssetStorage<BodySequence>>,
        Read<'s, AssetStorage<BodyFrame>>,
    );
    type Result = Handle<BodySequence>;

    fn add_to_entity(
        &self,
        _: Entity,
        (loader, body_sequence_assets, body_frame_assets): &mut Self::SystemData,
        _: &[Entity],
    ) -> Result<Handle<BodySequence>, Error> {
        let handle = match *self {
            BodySequencePrefab::Object(ref sequence) => {
                let body_sequence = Self::build_from_object(sequence, loader, body_frame_assets);

                loader.load_from_data(body_sequence, (), &body_sequence_assets)
            }
            BodySequencePrefab::Handle(ref handle) => handle.clone(),
        };
        Ok(handle)
    }

    fn load_sub_assets(
        &mut self,
        progress: &mut ProgressCounter,
        (loader, body_sequence_assets, body_frame_assets): &mut Self::SystemData,
    ) -> Result<bool, Error> {
        let handle = match *self {
            BodySequencePrefab::Object(ref sequence) => {
                let body_sequence = Self::build_from_object(sequence, loader, body_frame_assets);

                Some(loader.load_from_data(body_sequence, progress, &body_sequence_assets))
            }
            BodySequencePrefab::Handle(_) => None,
        };
        if let Some(handle) = handle {
            *self = BodySequencePrefab::Handle(handle);
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

impl<S> BodySequencePrefab<S>
where
    S: BodyAnimationSequence,
{
    fn build_from_object(
        sequence: &S,
        loader: &Loader,
        body_frame_assets: &Read<AssetStorage<BodyFrame>>,
    ) -> BodySequence {
        let body_frame_handles = sequence
            .frames()
            .iter()
            .map(|frame| {
                loader.load_from_data(
                    BodyFrame::new(frame.body().map(Clone::clone), frame.wait()),
                    (),
                    body_frame_assets,
                )
            })
            .collect::<Vec<Handle<BodyFrame>>>();
        BodySequence::new(body_frame_handles)
    }
}
