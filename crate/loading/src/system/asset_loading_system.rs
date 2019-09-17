use amethyst::{
    assets::ProgressCounter,
    ecs::{Read, System, World, Write},
    shred::{ResourceId, SystemData},
};
use asset_model::{
    config::{AssetIndex, AssetRecord, AssetType},
    loaded::{AssetId, AssetIdMappings, AssetTypeMappings},
};
use derivative::Derivative;
use derive_new::new;
use sequence_model::loaded::WaitSequenceHandles;
use slotmap::SecondaryMap;
use typename_derive::TypeName;

/// Loads game object assets.
#[derive(Default, Derivative, TypeName, new)]
#[derivative(Debug)]
pub struct AssetLoadingSystem {
    /// Tracks loaded assets.
    #[derivative(Debug = "ignore")]
    #[new(default)]
    progress_counter: ProgressCounter,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct AssetLoadingSystemData<'s> {
    /// `Option<AssetIndex>` resource.
    #[derivative(Debug = "ignore")]
    asset_index: Read<'s, Option<AssetIndex>>,
    /// `AssetLoadingResources`.
    #[derivative(Debug = "ignore")]
    asset_loading_resources: AssetLoadingResources<'s>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct AssetLoadingResources<'s> {
    /// `AssetIdMappings` resource.
    #[derivative(Debug = "ignore")]
    asset_id_mappings: Write<'s, AssetIdMappings>,
    /// `AssetTypeMappings` resource.
    #[derivative(Debug = "ignore")]
    asset_type_mappings: Write<'s, AssetTypeMappings>,
    /// `SecondaryMap::<AssetId, WaitSequenceHandles>` resource.
    #[derivative(Debug = "ignore")]
    asset_wait_sequence_handles: Write<'s, SecondaryMap<AssetId, WaitSequenceHandles>>,
}

impl<'s> System<'s> for AssetLoadingSystem {
    type SystemData = AssetLoadingSystemData<'s>;

    fn run(
        &mut self,
        AssetLoadingSystemData {
            asset_index,
            mut asset_loading_resources,
        }: Self::SystemData,
    ) {
        if asset_index.is_none() {
            return;
        }

        let asset_index = asset_index
            .as_ref()
            .expect("Expected `AssetIndex` to exist.");

        let capacity = asset_index
            .objects
            .values()
            .fold(0, |acc, records| acc + records.len())
            + asset_index.maps.len();
        asset_loading_resources.asset_id_mappings.reserve(capacity);
        (*asset_loading_resources.asset_wait_sequence_handles).set_capacity(capacity);

        let asset_records_objects =
            asset_index
                .objects
                .iter()
                .flat_map(|(object_type, asset_records)| {
                    let asset_type = AssetType::Object(*object_type);
                    asset_records
                        .iter()
                        .map(move |asset_record| (asset_type, asset_record))
                });
        let asset_records_maps = asset_index
            .maps
            .iter()
            .map(|asset_record| (AssetType::Map, asset_record));
        let asset_records = asset_records_objects.chain(asset_records_maps);
        asset_records.for_each(|(asset_type, asset_record)| {
            Self::process_asset(&mut asset_loading_resources, asset_type, asset_record)
        });
    }
}

impl AssetLoadingSystem {
    fn process_asset(
        AssetLoadingResources {
            ref mut asset_id_mappings,
            ref mut asset_type_mappings,
            asset_wait_sequence_handles: _,
        }: &mut AssetLoadingResources,
        asset_type: AssetType,
        asset_record: &AssetRecord,
    ) {
        let asset_id = asset_id_mappings.insert(asset_record.asset_slug.clone());
        asset_type_mappings.insert(asset_id, asset_type);

        match asset_type {
            AssetType::Object(_object_type) => {}
            AssetType::Map => {}
        }
    }
}
