use std::marker::PhantomData;

use amethyst::{ecs::prelude::*, shrev::EventChannel};
use application_event::{AppEvent, AppEventVariant};
use structopt::StructOpt;
use typename::TypeName;

use Result;
use StdinMapper;
use VariantAndTokens;

/// Type to fetch the application event channel.
type MapperSystemData<'s, SysData> = (
    Read<'s, EventChannel<VariantAndTokens>>,
    Write<'s, EventChannel<AppEvent>>,
    SysData,
);

/// Rendering system.
#[derive(Debug, TypeName, new)]
pub struct MapperSystem<M>
where
    M: StdinMapper + TypeName,
{
    /// The `AppEventVariant` that this system should handle.
    variant: AppEventVariant,
    /// Reader ID for variant and tokens event channel.
    #[new(default)]
    reader_id: Option<ReaderId<VariantAndTokens>>,
    /// Marker.
    marker: PhantomData<M>,
}

impl<'s, M> System<'s> for MapperSystem<M>
where
    M: StdinMapper + TypeName,
    M::Resource: Default + Send + Sync + 'static,
    AppEvent: From<M::Event>,
{
    type SystemData = MapperSystemData<'s, Read<'s, M::Resource>>;

    fn run(&mut self, (variant_channel, mut app_event_channel, resources): Self::SystemData) {
        let mut events = variant_channel
            .read(self.reader_id.as_mut().unwrap())
            .filter_map(|&(variant, ref tokens)| {
                if variant == self.variant {
                    Some(tokens)
                } else {
                    None
                }
            }).map(|tokens| -> Result<M::Event> {
                let args = M::Args::from_iter_safe(tokens.iter())?;
                M::map(&resources, args)
            }).filter_map(|result| match result {
                Ok(event) => Some(AppEvent::from(event)),
                Err(e) => {
                    error!("{}", e);
                    None
                }
            }).collect::<Vec<_>>();

        app_event_channel.drain_vec_write(&mut events);
    }

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        self.reader_id = Some(
            res.fetch_mut::<EventChannel<VariantAndTokens>>()
                .register_reader(),
        );
    }
}
