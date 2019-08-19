use std::marker::PhantomData;

use amethyst::{
    ecs::{Read, System, SystemData, World, Write},
    shrev::{EventChannel, ReaderId},
    Error,
};
use application_event::AppEventVariant;
use derive_new::new;
use log::error;
use structopt::StructOpt;
use typename::TypeName as TypeNameTrait;
use typename_derive::TypeName;

use crate::{MapperSystemData, StdinMapper, VariantAndTokens};

/// Rendering system.
#[derive(Debug, TypeName, new)]
pub struct MapperSystem<M>
where
    M: StdinMapper + TypeNameTrait,
{
    /// The `AppEventVariant` that this system should handle.
    variant: AppEventVariant,
    /// Reader ID for variant and tokens event channel.
    #[new(default)]
    reader_id: Option<ReaderId<VariantAndTokens>>,
    /// Marker.
    marker: PhantomData<M>,
}

type MapperSystemSystemData<'s, E, SD> = (
    Read<'s, EventChannel<VariantAndTokens>>,
    Write<'s, EventChannel<E>>,
    <SD as MapperSystemData<'s>>::SystemData,
);

impl<'s, M> System<'s> for MapperSystem<M>
where
    M: StdinMapper + TypeNameTrait,
{
    type SystemData = MapperSystemSystemData<'s, M::Event, M::SystemData>;

    fn run(
        &mut self,
        (variant_channel, mut app_event_channel, mapper_system_data): Self::SystemData,
    ) {
        let mut events = variant_channel
            .read(self.reader_id.as_mut().unwrap())
            .filter_map(|&(variant, ref tokens)| {
                if variant == self.variant {
                    Some(tokens)
                } else {
                    None
                }
            })
            .map(|tokens| -> Result<M::Event, Error> {
                let args = M::Args::from_iter_safe(tokens.iter())?;
                M::map(&mapper_system_data, args)
            })
            .filter_map(|result| match result {
                Ok(event) => Some(event),
                Err(e) => {
                    error!("{}", e);
                    None
                }
            })
            .collect::<Vec<_>>();

        app_event_channel.drain_vec_write(&mut events);
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.reader_id = Some(
            world
                .fetch_mut::<EventChannel<VariantAndTokens>>()
                .register_reader(),
        );
    }
}
