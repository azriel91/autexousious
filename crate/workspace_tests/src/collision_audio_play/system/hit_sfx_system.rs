#[cfg(test)]
mod tests {
    use amethyst::{
        ecs::{Builder, World, WorldExt},
        shrev::EventChannel,
        Error,
    };
    use application_test_support::AutexousiousApplication;
    use collision_model::{
        config::{Hit, Interaction, InteractionKind},
        play::HitEvent,
    };
    use shape_model::Volume;

    use collision_audio_play::HitSfxSystem;

    #[test]
    fn plays_sound_on_hit_event() -> Result<(), Error> {
        AutexousiousApplication::config_base()
            .with_system(HitSfxSystem::new(), "", &[])
            .with_effect(|world| {
                let entity_from = world.create_entity().build();
                let entity_to = world.create_entity().build();

                let event = HitEvent::new(entity_from, entity_to, interaction(), body());
                send_event(world, event);
            })
            .with_assertion(|_world| {})
            .run_winit_loop()
    }

    fn send_event(world: &mut World, event: HitEvent) {
        let mut ec = world.write_resource::<EventChannel<HitEvent>>();
        ec.single_write(event)
    } // kcov-ignore

    fn interaction() -> Interaction {
        Interaction::new(InteractionKind::Hit(Hit::default()), vec![], true)
    }

    fn body() -> Volume {
        Volume::Box {
            x: 0,
            y: 0,
            z: 0,
            w: 1,
            h: 1,
            d: 1,
        }
    }
}
