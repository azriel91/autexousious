use std::cmp::Ordering;

use amethyst::{
    ecs::{Entities, Read, ReadStorage, System, World, Write, WriteStorage},
    shred::{ResourceId, SystemData},
};
use asset_model::loaded::ItemId;
use derivative::Derivative;
use derive_new::new;
use kinematic_model::{
    config::Position,
    play::{PositionInitOffset, PositionInitParent},
};
use log::debug;
use network_session_model::play::SessionDevices;
use parent_model::play::ParentEntity;
use session_lobby_ui_model::{
    loaded::SessionDevicesWidget,
    play::{SessionDeviceWidget, SessionDevicesEntities},
};
use ui_model_spi::config::Dimensions;

/// Updates `SessionDevicesEntities` to have the right number of entities for each `SessionDevice`.
#[derive(Debug, new)]
pub struct SessionDeviceEntityCreateDeleteSystem;

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct SessionDeviceEntityCreateDeleteSystemData<'s> {
    /// `SessionDevicesEntities` resource.
    #[derivative(Debug = "ignore")]
    pub session_devices_entities: Write<'s, SessionDevicesEntities>,
    /// `SessionDevicesWidget` components.
    #[derivative(Debug = "ignore")]
    pub session_devices_widgets: ReadStorage<'s, SessionDevicesWidget>,
    /// `Entities`.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// `Dimensions` components.
    #[derivative(Debug = "ignore")]
    pub dimensionses: ReadStorage<'s, Dimensions>,
    /// `SessionDevices` resource.
    #[derivative(Debug = "ignore")]
    pub session_devices: Read<'s, SessionDevices>,
    /// `ItemId` components.
    #[derivative(Debug = "ignore")]
    pub item_ids: WriteStorage<'s, ItemId>,
    /// `ParentEntity` components.
    #[derivative(Debug = "ignore")]
    pub parent_entities: WriteStorage<'s, ParentEntity>,
    /// `PositionInitParent` components.
    #[derivative(Debug = "ignore")]
    pub position_init_parents: WriteStorage<'s, PositionInitParent>,
    /// `PositionInitOffset` components.
    #[derivative(Debug = "ignore")]
    pub position_init_offsets: WriteStorage<'s, PositionInitOffset>,
    /// `SessionDeviceWidget` components.
    #[derivative(Debug = "ignore")]
    pub session_device_widgets: WriteStorage<'s, SessionDeviceWidget>,
}

impl<'s> System<'s> for SessionDeviceEntityCreateDeleteSystem {
    type SystemData = SessionDeviceEntityCreateDeleteSystemData<'s>;

    fn run(
        &mut self,
        SessionDeviceEntityCreateDeleteSystemData {
            mut session_devices_entities,
            session_devices_widgets,
            entities,
            dimensionses,
            session_devices,
            mut item_ids,
            mut parent_entities,
            mut position_init_parents,
            mut position_init_offsets,
            mut session_device_widgets,
        }: Self::SystemData,
    ) {
        let SessionDevicesEntities {
            session_devices_entity,
            session_device_entities,
        } = &mut *session_devices_entities;

        let session_devices_entity = session_devices_entity.and_then(|session_devices_entity| {
            if entities.is_alive(session_devices_entity) {
                Some(session_devices_entity)
            } else {
                None
            }
        });
        if let Some(session_devices_entity) = session_devices_entity {
            // Find session device entities to:
            //
            // * Modify values.
            // * Remove extra.
            // * Create deficit.

            match session_devices.len().cmp(&session_device_entities.len()) {
                Ordering::Equal => {}
                Ordering::Less => {
                    debug!(
                        "Removing extra session device entities. Required: {}, Actual: {}",
                        session_devices.len(),
                        session_device_entities.len()
                    );

                    // Remove extra entities.
                    session_device_entities
                        .drain(session_devices.len()..)
                        .for_each(|session_device_entity| {
                            let session_device_widget =
                                session_device_widgets.get(session_device_entity).copied();

                            if let Some(session_device_widget) = session_device_widget {
                                let SessionDeviceWidget {
                                    entity_id,
                                    entity_name,
                                } = session_device_widget;

                                entities
                                    .delete(entity_id)
                                    .expect("Failed to delete `entity_id`.");
                                entities
                                    .delete(entity_name)
                                    .expect("Failed to delete `entity_name`.");
                            }

                            entities
                                .delete(session_device_entity)
                                .expect("Failed to delete `session_device_entity`.");
                        });
                }
                Ordering::Greater => {
                    debug!(
                        "Creating additional session device entities. Required: {}, Actual: {}",
                        session_devices.len(),
                        session_device_entities.len()
                    );

                    // Create additional entities.
                    let session_devices_widget =
                        session_devices_widgets.get(session_devices_entity).expect(
                            "Expected `SessionDevicesWidget` to exist for \
                            `session_devices_entity`.",
                        );
                    let dimensions = dimensionses
                        .get(session_devices_entity)
                        .expect("Expected `Dimensions` to exist for `session_devices_entity`.");

                    let session_device_entities_new = (0..session_devices.len())
                        .skip(session_device_entities.len())
                        .map(|n| {
                            let parent_entity = ParentEntity::new(session_devices_entity);
                            let position_init_parent =
                                PositionInitParent::new(session_devices_entity);
                            let y_offset = -((n * dimensions.h as usize) as f32);
                            let position_init_offset =
                                PositionInitOffset::new(Position::new(0., y_offset, 0.));

                            let item_id_session_device_id =
                                session_devices_widget.item_id_session_device_id;
                            let item_id_session_device_name =
                                session_devices_widget.item_id_session_device_name;

                            let entity_id = entities
                                .build_entity()
                                .with(parent_entity, &mut parent_entities)
                                .with(position_init_parent, &mut position_init_parents)
                                .with(position_init_offset, &mut position_init_offsets)
                                .with(item_id_session_device_id, &mut item_ids)
                                .build();
                            let entity_name = entities
                                .build_entity()
                                .with(parent_entity, &mut parent_entities)
                                .with(position_init_parent, &mut position_init_parents)
                                .with(position_init_offset, &mut position_init_offsets)
                                .with(item_id_session_device_name, &mut item_ids)
                                .build();

                            let session_device_widget =
                                SessionDeviceWidget::new(entity_id, entity_name);

                            entities
                                .build_entity()
                                .with(parent_entity, &mut parent_entities)
                                .with(position_init_parent, &mut position_init_parents)
                                .with(position_init_offset, &mut position_init_offsets)
                                .with(session_device_widget, &mut session_device_widgets)
                                .build()
                        });

                    session_device_entities.extend(session_device_entities_new);
                }
            }

            // Update values if necessary. This should be run regardless of value comparison.
        }
    }
}
