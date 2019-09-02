use amethyst::{
    assets::AssetStorage,
    ecs::{Join, Read, ReadExpect, ReadStorage, System, World, WriteStorage},
    shred::{ResourceId, SystemData},
};
use derivative::Derivative;
use derive_new::new;
use kinematic_model::config::Position;
use map_model::loaded::Map;
use map_selection_model::MapSelection;
use object_model::play::Grounding;
use typename_derive::TypeName;

/// Updates `Grounding` to `Airborne` for objects above the map bottom boundary.
#[derive(Debug, Default, TypeName, new)]
pub struct ObjectGroundingSystem;

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct ObjectGroundingSystemData<'s> {
    /// `MapSelection` resource.
    #[derivative(Debug = "ignore")]
    pub map_selection: ReadExpect<'s, MapSelection>,
    /// `Map` assets.
    #[derivative(Debug = "ignore")]
    pub maps: Read<'s, AssetStorage<Map>>,
    /// `Position<f32>` components.
    #[derivative(Debug = "ignore")]
    pub positions: ReadStorage<'s, Position<f32>>,
    /// `Grounding` components.
    #[derivative(Debug = "ignore")]
    pub groundings: WriteStorage<'s, Grounding>,
}

impl<'s> System<'s> for ObjectGroundingSystem {
    type SystemData = ObjectGroundingSystemData<'s>;

    fn run(
        &mut self,
        ObjectGroundingSystemData {
            map_selection,
            maps,
            positions,
            mut groundings,
        }: Self::SystemData,
    ) {
        let map_margins = {
            maps.get(map_selection.handle())
                .map(|map| &map.margins)
                .expect("Expected map to be loaded.")
        };

        (&positions, &mut groundings)
            .join()
            .for_each(|(position, grounding)| {
                if position[1] > map_margins.bottom {
                    *grounding = Grounding::Airborne;
                } else if position[1] < map_margins.bottom {
                    *grounding = Grounding::Underground;
                } else {
                    *grounding = Grounding::OnGround;
                }
            });
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use amethyst::{
        assets::{AssetStorage, Loader},
        ecs::{Builder, Entity, System, SystemData, World, WorldExt},
        Error,
    };
    use amethyst_test::AmethystApplication;
    use asset_model::{config::AssetSlug, loaded::SlugAndHandle};
    use kinematic_model::config::Position;
    use map_loading::MapLoadingBundle;
    use map_model::{
        config::{MapBounds, MapDefinition, MapHeader},
        loaded::{Map, Margins},
    };
    use map_selection_model::MapSelection;
    use object_model::play::Grounding;
    use typename::TypeName;

    use super::ObjectGroundingSystem;

    #[test]
    fn sets_grounding_to_on_ground_when_on_bottom_boundary() -> Result<(), Error> {
        run_test(
            SetupParams {
                grounding: Grounding::Airborne,
                position: Position::new(0., 200., 0.),
            },
            ExpectedParams {
                grounding: Grounding::OnGround,
            },
        )
    }

    #[test]
    fn sets_grounding_to_underground_when_below_ground() -> Result<(), Error> {
        run_test(
            SetupParams {
                grounding: Grounding::OnGround,
                position: Position::new(0., 190., 0.),
            },
            ExpectedParams {
                grounding: Grounding::Underground,
            },
        )
    }

    #[test]
    fn sets_grounding_to_airborne_when_above_ground() -> Result<(), Error> {
        run_test(
            SetupParams {
                grounding: Grounding::OnGround,
                position: Position::new(0., 210., 0.),
            },
            ExpectedParams {
                grounding: Grounding::Airborne,
            },
        )
    }

    fn run_test(
        SetupParams {
            grounding,
            position,
        }: SetupParams,
        ExpectedParams {
            grounding: grounding_expected,
        }: ExpectedParams,
    ) -> Result<(), Error> {
        AmethystApplication::blank()
            .with_bundle(MapLoadingBundle::new())
            .with_setup(setup_system_data)
            .with_setup(|world| {
                let map_handle = {
                    let map = empty_map();
                    let loader = world.read_resource::<Loader>();
                    let map_assets = world.read_resource::<AssetStorage<Map>>();

                    loader.load_from_data(map, (), &map_assets)
                };

                let slug = AssetSlug::from_str("test/empty_map")
                    .expect("Expected asset slug to be valid.");
                let snh = SlugAndHandle::new(slug, map_handle);
                let map_selection = MapSelection::Id(snh);

                world.insert(map_selection);
            })
            .with_effect(move |world| {
                let entity = world.create_entity().with(grounding).with(position).build();

                world.insert(entity);
            })
            .with_system_single(
                ObjectGroundingSystem::new(),
                ObjectGroundingSystem::type_name(),
                &[],
            ) // kcov-ignore
            .with_assertion(move |world| {
                let entity = *world.read_resource::<Entity>();
                let groundings = world.read_storage::<Grounding>();
                let grounding_actual = groundings
                    .get(entity)
                    .copied()
                    .expect("Expected entity to have `Grounding` component.");

                assert_eq!(grounding_expected, grounding_actual);
            })
            .run()
    }

    fn setup_system_data(world: &mut World) {
        <ObjectGroundingSystem as System<'_>>::SystemData::setup(world);
    }

    fn empty_map() -> Map {
        let map_bounds = MapBounds::new(0, 0, 0, 800, 600, 200);
        let map_header = MapHeader::new(String::from("empty_map"), map_bounds);
        let map_definition = MapDefinition::new(map_header, Vec::new());
        let map_margins = Margins::from(map_bounds);
        Map::new(
            map_definition,
            map_margins,
            Vec::new(),
            Vec::new(),
            Vec::new(),
        )
    }

    struct SetupParams {
        grounding: Grounding,
        position: Position<f32>,
    }

    struct ExpectedParams {
        grounding: Grounding,
    }
}
