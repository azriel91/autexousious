use std::{collections::HashMap, hash::Hash};

use amethyst::{
    animation::{Animation, InterpolationFunction, Sampler},
    assets::{AssetStorage, Loader},
};
use animation_support::{ActiveHandleChannel, ActiveHandlePrimitive};
use collision_model::{
    animation::{
        BodyAnimationFrame, BodyAnimationHandle, BodyAnimationSequence, BodyFrameActiveHandle,
        BodyFramePrimitive,
    },
    config::BodyFrame,
};

/// Loads `Animation`s from character sequences.
#[derive(Debug)]
pub struct BodyAnimationLoader;

impl BodyAnimationLoader {
    /// Loads `BodyFrame` animations and returns a hash map of their handles by sequence ID.
    ///
    /// # Parameters
    ///
    /// * `loader`: `Loader` to load assets.
    /// * `body_frame_assets`: `AssetStorage` for `BodyFrame`s.
    /// * `body_frame_primitive_sampler_assets`: `AssetStorage` for `Sampler<BodyFramePrimitive>`s.
    /// * `body_frame_animation_assets`: `AssetStorage` for `Animation<BodyFrameActiveHandle>`s.
    /// * `sequences`: Sequences of the animation.
    pub fn load_into_map<'seq, SequenceId, Sequence, Frame>(
        loader: &'seq Loader,
        body_frame_assets: &'seq AssetStorage<BodyFrame>,
        body_frame_primitive_sampler_assets: &'seq AssetStorage<Sampler<BodyFramePrimitive>>,
        body_frame_animation_assets: &'seq AssetStorage<Animation<BodyFrameActiveHandle>>,
        sequences: &HashMap<SequenceId, Sequence>,
    ) -> HashMap<SequenceId, BodyAnimationHandle>
    where
        SequenceId: Copy + Eq + Hash + 'seq,
        Frame: BodyAnimationFrame,
        Sequence: BodyAnimationSequence<Frame = Frame> + 'seq,
    {
        Self::load(
            loader,
            body_frame_assets,
            body_frame_primitive_sampler_assets,
            body_frame_animation_assets,
            sequences.iter(),
        )
        .map(|(id, handle)| (*id, handle))
        .collect::<HashMap<SequenceId, BodyAnimationHandle>>()
    }

    /// Loads `BodyFrame` animations and returns a vector of their handles in order.
    ///
    /// # Parameters
    ///
    /// * `loader`: `Loader` to load assets.
    /// * `body_frame_assets`: `AssetStorage` for `BodyFrame`s.
    /// * `body_frame_primitive_sampler_assets`: `AssetStorage` for `Sampler<BodyFramePrimitive>`s.
    /// * `body_frame_animation_assets`: `AssetStorage` for `Animation<BodyFrameActiveHandle>`s.
    /// * `sequences`: Sequences of the animation.
    pub fn load_into_vec<'seq, Sequences, Sequence, Frame>(
        loader: &'seq Loader,
        body_frame_assets: &'seq AssetStorage<BodyFrame>,
        body_frame_primitive_sampler_assets: &'seq AssetStorage<Sampler<BodyFramePrimitive>>,
        body_frame_animation_assets: &'seq AssetStorage<Animation<BodyFrameActiveHandle>>,
        sequences: Sequences,
    ) -> Vec<BodyAnimationHandle>
    where
        Sequences: Iterator<Item = &'seq Sequence>,
        Frame: BodyAnimationFrame,
        Sequence: BodyAnimationSequence<Frame = Frame> + 'seq,
    {
        sequences
            .map(|sequence| {
                Self::sequence_to_animation(
                    loader,
                    body_frame_assets,
                    body_frame_primitive_sampler_assets,
                    sequence,
                )
            })
            .map(|animation| loader.load_from_data(animation, (), body_frame_animation_assets))
            .collect::<Vec<BodyAnimationHandle>>()
    }

    /// Loads `BodyFrame` animations and returns an iterator to their handles by sequence ID.
    ///
    /// The caller is responsible for collecting the elements into the desired collection type.
    ///
    /// # Parameters
    ///
    /// * `loader`: `Loader` to load assets.
    /// * `body_frame_assets`: `AssetStorage` for `BodyFrame`s.
    /// * `body_frame_primitive_sampler_assets`: `AssetStorage` for `Sampler<BodyFramePrimitive>`s.
    /// * `body_frame_animation_assets`: `AssetStorage` for `Animation<BodyFrameActiveHandle>`s.
    /// * `sequences`: Sequences of the animation.
    pub fn load<'seq, Sequences, SequenceId, Sequence, Frame>(
        loader: &'seq Loader,
        body_frame_assets: &'seq AssetStorage<BodyFrame>,
        body_frame_primitive_sampler_assets: &'seq AssetStorage<Sampler<BodyFramePrimitive>>,
        body_frame_animation_assets: &'seq AssetStorage<Animation<BodyFrameActiveHandle>>,
        sequences: Sequences,
    ) -> impl Iterator<Item = (&'seq SequenceId, BodyAnimationHandle)>
    where
        Sequences: Iterator<Item = (&'seq SequenceId, &'seq Sequence)>,
        SequenceId: 'seq,
        Frame: BodyAnimationFrame,
        Sequence: BodyAnimationSequence<Frame = Frame> + 'seq,
    {
        sequences
            .map(move |(id, sequence)| {
                (
                    id,
                    Self::sequence_to_animation(
                        loader,
                        body_frame_assets,
                        body_frame_primitive_sampler_assets,
                        sequence,
                    ),
                )
            })
            .map(move |(id, animation)| {
                let animation_handle =
                    loader.load_from_data(animation, (), body_frame_animation_assets);
                (id, animation_handle)
            })
    }

    /// Maps a `Sequence` into an Amethyst `Animation`.
    ///
    /// # Parameters
    ///
    /// * `loader`: `Loader` to load assets.
    /// * `body_frame_assets`: `AssetStorage` for `BodyFrame`s.
    /// * `body_frame_primitive_sampler_assets`: `AssetStorage` for `Sampler<BodyFramePrimitive>`s.
    /// * `sequence`: `Sequence` to create the animation from.
    fn sequence_to_animation<Sequence: BodyAnimationSequence<Frame = F>, F: BodyAnimationFrame>(
        loader: &Loader,
        body_frame_assets: &AssetStorage<BodyFrame>,
        body_frame_primitive_sampler_assets: &AssetStorage<Sampler<BodyFramePrimitive>>,
        sequence: &Sequence,
    ) -> Animation<BodyFrameActiveHandle> {
        let frames = sequence.frames();
        let mut input = Vec::with_capacity(frames.len() + 1);
        let mut tick_counter = 0.;
        for frame in frames {
            input.push(tick_counter);
            tick_counter += 1. + frame.wait() as f32;
        }
        input.push(tick_counter);

        let frame_sampler = Self::frame_sampler(loader, body_frame_assets, sequence, input.clone());
        let frame_sampler_handle =
            loader.load_from_data(frame_sampler, (), body_frame_primitive_sampler_assets);

        Animation {
            nodes: vec![(0, ActiveHandleChannel::Handle, frame_sampler_handle)],
        }
    }

    fn frame_sampler<Sequence: BodyAnimationSequence<Frame = F>, F: BodyAnimationFrame>(
        loader: &Loader,
        body_frame_assets: &AssetStorage<BodyFrame>,
        sequence: &Sequence,
        input: Vec<f32>,
    ) -> Sampler<BodyFramePrimitive> {
        let frame_count = sequence.frames().len();
        let mut output = Vec::with_capacity(frame_count);
        sequence.frames().iter().for_each(|frame| {
            // TODO: load `BodyFrame`s and pass `Handle`s in.
            let handle = loader.load_from_data(
                BodyFrame::new(frame.body().clone(), frame.wait()),
                (),
                body_frame_assets,
            );

            output.push(ActiveHandlePrimitive::Handle(handle));
        });
        let final_key_frame = output.last().cloned();
        if final_key_frame.is_some() {
            output.push(final_key_frame.unwrap());
        }

        Sampler {
            input,
            output,
            function: InterpolationFunction::Step,
        }
    }
}

#[cfg(test)]
mod test {
    use shape_model::Volume;
    use std::collections::HashMap;

    use amethyst::{
        animation::{Animation, AnimationBundle, InterpolationFunction, Sampler},
        assets::{AssetStorage, Loader},
        ecs::World,
    };
    use amethyst_test::prelude::*;
    use animation_support::ActiveHandleChannel;
    use collision_model::{
        animation::{
            BodyAnimationHandle, BodyAnimationSequence, BodyFrameActiveHandle, BodyFramePrimitive,
        },
        config::BodyFrame,
    };
    use derive_new::new;

    use super::BodyAnimationLoader;
    use crate::CollisionLoadingBundle;

    #[test]
    fn loads_body_animations_into_map() {
        let effect = |world: &mut World| {
            let test_sequences = test_sequences();

            let animation_handles = {
                let loader = world.read_resource::<Loader>();
                let body_frame_assets = world.read_resource::<AssetStorage<BodyFrame>>();
                let body_frame_primitive_sampler_assets =
                    world.read_resource::<AssetStorage<Sampler<BodyFramePrimitive>>>();
                let body_frame_animation_assets =
                    world.read_resource::<AssetStorage<Animation<BodyFrameActiveHandle>>>();

                BodyAnimationLoader::load_into_map(
                    &loader,
                    &body_frame_assets,
                    &body_frame_primitive_sampler_assets,
                    &body_frame_animation_assets,
                    &test_sequences,
                )
            };

            world.add_resource(EffectReturn(animation_handles));
        }; // kcov-ignore
        let assertion = |world: &mut World| {
            let animation_handles = &world
                .read_resource::<EffectReturn<HashMap<TestSequenceId, BodyAnimationHandle>>>()
                .0;

            // Verify animation is loaded
            verify_animation_handle(world, animation_handles.get(&TestSequenceId::Boo));
        };

        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::render_base("loads_body_animations_into_map", false)
                .with_bundle(AnimationBundle::<u32, BodyFrameActiveHandle>::new(
                    "body_frame_acs",
                    "body_frame_sis",
                ))
                .with_bundle(CollisionLoadingBundle::new())
                .with_effect(effect)
                .with_assertion(assertion)
                .run()
                .is_ok()
        );
    }

    #[test]
    fn loads_body_animations_into_vec() {
        let effect = |world: &mut World| {
            let test_sequences = test_sequences();

            let animation_handles = {
                let loader = world.read_resource::<Loader>();
                let body_frame_assets = world.read_resource::<AssetStorage<BodyFrame>>();
                let body_frame_primitive_sampler_assets =
                    world.read_resource::<AssetStorage<Sampler<BodyFramePrimitive>>>();
                let body_frame_animation_assets =
                    world.read_resource::<AssetStorage<Animation<BodyFrameActiveHandle>>>();

                BodyAnimationLoader::load_into_vec(
                    &loader,
                    &body_frame_assets,
                    &body_frame_primitive_sampler_assets,
                    &body_frame_animation_assets,
                    test_sequences.values(),
                )
            };

            world.add_resource(EffectReturn(animation_handles));
        };
        let assertion = |world: &mut World| {
            let animation_handles = &world
                .read_resource::<EffectReturn<Vec<BodyAnimationHandle>>>()
                .0;

            // Verify animation is loaded
            verify_animation_handle(world, animation_handles.first());
        };

        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::render_base("loads_body_animations_into_vec", false)
                .with_bundle(AnimationBundle::<u32, BodyFrameActiveHandle>::new(
                    "body_frame_acs",
                    "body_frame_sis",
                ))
                .with_bundle(CollisionLoadingBundle::new())
                .with_effect(effect)
                .with_assertion(assertion)
                .run()
                .is_ok()
        );
    }

    fn verify_animation_handle(world: &World, animation_handle: Option<&BodyAnimationHandle>) {
        assert!(animation_handle.is_some());

        let animation_handle = animation_handle.unwrap();
        let animation_store =
            world.read_resource::<AssetStorage<Animation<BodyFrameActiveHandle>>>();
        let animation = animation_store.get(animation_handle);
        assert!(animation.is_some());

        let animation = animation.unwrap();
        assert_eq!(1, animation.nodes.len());

        let node_0 = &animation.nodes[0];
        assert_eq!(0, node_0.0);
        assert_eq!(ActiveHandleChannel::Handle, node_0.1);

        // Verify animation samplers
        let frame_sampler_handle = &node_0.2;
        let frame_sampler_store =
            world.read_resource::<AssetStorage<Sampler<BodyFramePrimitive>>>();
        let frame_sampler = frame_sampler_store.get(frame_sampler_handle);
        assert!(frame_sampler.is_some());

        let frame_sampler = frame_sampler.unwrap();
        assert_eq!(vec![0.0, 1.0, 4.0, 6.0], frame_sampler.input);
        // TODO: Verify on handles
        // assert_eq!(
        //     vec![
        //         ActiveHandlePrimitive::Handle(10.into()),
        //         ActiveHandlePrimitive::Handle(11.into()),
        //         ActiveHandlePrimitive::Handle(12.into()),
        //         ActiveHandlePrimitive::Handle(12.into()),
        //     ],
        //     frame_sampler.output
        // );
        assert_eq!(InterpolationFunction::Step, frame_sampler.function);
    }

    fn test_sequences() -> HashMap<TestSequenceId, TestSequence> {
        // Sheet, Sprite, Wait
        let frames = vec![
            BodyFrame::new(test_body(), 0), // TU: 0 to 1
            BodyFrame::new(test_body(), 2), // TU: 1 to 4
            BodyFrame::new(test_body(), 1), // TU: 4 to 6
        ];
        let sequence = TestSequence::new(frames);
        let mut sequences = HashMap::new();
        sequences.insert(TestSequenceId::Boo, sequence);
        sequences
    }

    fn test_body() -> Vec<Volume> {
        vec![Volume::Sphere {
            x: 1,
            y: 2,
            z: 3,
            r: 4,
        }]
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    enum TestSequenceId {
        Boo,
    }
    impl Default for TestSequenceId {
        fn default() -> Self {
            TestSequenceId::Boo
        }
    }
    #[derive(Debug, new)]
    struct TestSequence {
        frames: Vec<BodyFrame>,
    }
    impl BodyAnimationSequence for TestSequence {
        type Frame = BodyFrame;
        fn frames(&self) -> &[BodyFrame] {
            &self.frames
        }
    }
}
