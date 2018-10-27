use std::collections::HashMap;
use std::hash::Hash;

use amethyst::{
    animation::{Animation, InterpolationFunction, Sampler},
    assets::Loader,
    prelude::*,
};
use animation_support::{ActiveHandleChannel, ActiveHandlePrimitive};
use collision_model::{
    animation::{
        CollisionAnimationHandle, CollisionDataSet, CollisionFrameActiveHandle,
        CollisionFramePrimitive,
    },
    config::CollisionFrame,
};

use CollisionAnimationFrame;
use CollisionAnimationSequence;

/// Loads `Animation`s from character sequences.
#[derive(Debug)]
pub struct CollisionAnimationLoader;

impl CollisionAnimationLoader {
    /// Loads `CollisionFrame` animations and returns a hash map of their handles by sequence ID.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` to load animations into.
    /// * `sequences`: Sequences of the animation.
    /// * `frame_index_offset`: Offset of the collision frame IDs in the `CollisionDataSet`.
    pub fn load_into_map<'seq, SequenceId, Sequence, Frame>(
        world: &'seq World,
        sequences: &HashMap<SequenceId, Sequence>,
        frame_index_offset: u64,
    ) -> HashMap<SequenceId, CollisionAnimationHandle>
    where
        SequenceId: Copy + Eq + Hash + 'seq,
        Frame: CollisionAnimationFrame,
        Sequence: CollisionAnimationSequence<Frame = Frame> + 'seq,
    {
        Self::load(world, sequences.iter(), frame_index_offset)
            .map(|(id, handle)| (*id, handle))
            .collect::<HashMap<SequenceId, CollisionAnimationHandle>>()
    }

    /// Loads `CollisionFrame` animations and returns a vector of their handles in order.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` to load animations into.
    /// * `sequences`: Sequences of the animation.
    /// * `frame_index_offset`: Offset of the collision frame IDs in the `CollisionDataSet`.
    pub fn load_into_vec<'seq, Sequences, Sequence, Frame>(
        world: &'seq World,
        sequences: Sequences,
        frame_index_offset: u64,
    ) -> Vec<CollisionAnimationHandle>
    where
        Sequences: Iterator<Item = &'seq Sequence>,
        Frame: CollisionAnimationFrame,
        Sequence: CollisionAnimationSequence<Frame = Frame> + 'seq,
    {
        sequences
            .map(|sequence| Self::sequence_to_animation(world, frame_index_offset, sequence))
            .map(|animation| {
                let loader = world.read_resource::<Loader>();
                loader.load_from_data(animation, (), &world.read_resource())
            })
            .collect::<Vec<CollisionAnimationHandle>>()
    }

    /// Loads `CollisionFrame` animations and returns an iterator to their handles by sequence ID.
    ///
    /// The caller is responsible for collecting the elements into the desired collection type.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` to load animations into.
    /// * `sequences`: Sequences of the animation.
    /// * `frame_index_offset`: Offset of the collision frame IDs in the `CollisionDataSet`.
    pub fn load<'seq, Sequences, SequenceId, Sequence, Frame>(
        world: &'seq World,
        sequences: Sequences,
        frame_index_offset: u64,
    ) -> impl Iterator<Item = (&'seq SequenceId, CollisionAnimationHandle)>
    where
        Sequences: Iterator<Item = (&'seq SequenceId, &'seq Sequence)>,
        SequenceId: 'seq,
        Frame: CollisionAnimationFrame,
        Sequence: CollisionAnimationSequence<Frame = Frame> + 'seq,
    {
        sequences
            .map(move |(id, sequence)| {
                (
                    id,
                    Self::sequence_to_animation(world, frame_index_offset, sequence),
                )
            })
            .map(move |(id, animation)| {
                let loader = world.read_resource::<Loader>();
                let animation_handle = loader.load_from_data(animation, (), &world.read_resource());
                (id, animation_handle)
            })
    }

    /// Maps a `Sequence` into an Amethyst `Animation`.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` to store the `Animation`s.
    /// * `frame_index_offset`: Offset of the collision frame IDs in the `CollisionDataSet`.
    /// * `sequence`: `Sequence` to create the animation from.
    fn sequence_to_animation<
        Sequence: CollisionAnimationSequence<Frame = F>,
        F: CollisionAnimationFrame,
    >(
        world: &World,
        frame_index_offset: u64,
        sequence: &Sequence,
    ) -> Animation<CollisionFrameActiveHandle> {
        let frames = sequence.frames();
        let mut input = Vec::with_capacity(frames.len() + 1);
        let mut tick_counter = 0.;
        for frame in frames {
            input.push(tick_counter);
            tick_counter += 1. + frame.wait() as f32;
        }
        input.push(tick_counter);

        let frame_sampler = Self::frame_sampler(world, frame_index_offset, sequence, input.clone());

        let loader = world.read_resource::<Loader>();
        let frame_sampler_handle = loader.load_from_data(frame_sampler, (), &world.read_resource());

        Animation {
            nodes: vec![(0, ActiveHandleChannel::Handle, frame_sampler_handle)],
        }
    }

    fn frame_sampler<
        Sequence: CollisionAnimationSequence<Frame = F>,
        F: CollisionAnimationFrame,
    >(
        world: &World,
        frame_index_offset: u64,
        sequence: &Sequence,
        input: Vec<f32>,
    ) -> Sampler<CollisionFramePrimitive> {
        let frame_count = sequence.frames().len();
        let mut collision_data_set = world.write_resource::<CollisionDataSet>();
        let mut output = Vec::with_capacity(frame_count);
        sequence
            .frames()
            .iter()
            .enumerate()
            .for_each(|(frame_index, frame)| {
                let id = frame_index_offset + frame_index as u64;

                // TODO: load `CollisionFrame`s and pass `Handle`s in.
                let loader = world.read_resource::<Loader>();
                let store = world.read_resource();
                let handle = loader.load_from_data(
                    CollisionFrame::new(
                        frame.body().map(Clone::clone),
                        frame.interactions().map(Clone::clone),
                        frame.wait(),
                    ),
                    (),
                    &store,
                );

                collision_data_set.insert(id.into(), handle);
                output.push(ActiveHandlePrimitive::Handle(id.into()));
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
        assets::AssetStorage,
        prelude::*,
    };
    use amethyst_test_support::prelude::*;
    use animation_support::{ActiveHandleChannel, ActiveHandlePrimitive};
    use collision_model::{
        animation::{
            CollisionAnimationHandle, CollisionDataSet, CollisionFrameActiveHandle,
            CollisionFrameId, CollisionFramePrimitive,
        },
        config::{CollisionFrame, Interaction},
    };

    use super::CollisionAnimationLoader;
    use CollisionAnimationSequence;

    #[test]
    fn loads_collision_animations_into_map() {
        let effect = |world: &mut World| {
            let frame_index_offset = 10;
            let test_sequences = test_sequences();
            let animation_handles =
                CollisionAnimationLoader::load_into_map(world, &test_sequences, frame_index_offset);
            world.add_resource(EffectReturn(animation_handles));
        }; // kcov-ignore
        let assertion = |world: &mut World| {
            let animation_handles = &world
                .read_resource::<EffectReturn<HashMap<TestSequenceId, CollisionAnimationHandle>>>()
                .0;

            // Verify animation is loaded
            verify_animation_handle(world, animation_handles.get(&TestSequenceId::Boo));
        };

        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::render_base("loads_collision_animations_into_map", false)
                .with_bundle(AnimationBundle::<
                    CollisionFrameId,
                    CollisionFrameActiveHandle,
                >::new(
                    "collision_animation_control_system",
                    "collision_sampler_interpolation_system",
                ))
                .with_resource(CollisionDataSet::new())
                .with_effect(effect)
                .with_assertion(assertion)
                .run()
                .is_ok()
        );
    }

    #[test]
    fn loads_collision_animations_into_vec() {
        let effect = |world: &mut World| {
            let frame_index_offset = 10;
            let test_sequences = test_sequences();
            let animation_handles = CollisionAnimationLoader::load_into_vec(
                world,
                test_sequences.values(),
                frame_index_offset,
            );
            world.add_resource(EffectReturn(animation_handles));
        };
        let assertion = |world: &mut World| {
            let animation_handles = &world
                .read_resource::<EffectReturn<Vec<CollisionAnimationHandle>>>()
                .0;

            // Verify animation is loaded
            verify_animation_handle(world, animation_handles.first());
        };

        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::render_base("loads_collision_animations_into_vec", false)
                .with_bundle(AnimationBundle::<
                    CollisionFrameId,
                    CollisionFrameActiveHandle,
                >::new(
                    "collision_animation_control_system",
                    "collision_sampler_interpolation_system",
                ))
                .with_resource(CollisionDataSet::new())
                .with_effect(effect)
                .with_assertion(assertion)
                .run()
                .is_ok()
        );
    }

    fn verify_animation_handle(world: &World, animation_handle: Option<&CollisionAnimationHandle>) {
        assert!(animation_handle.is_some());

        let animation_handle = animation_handle.unwrap();
        let animation_store =
            world.read_resource::<AssetStorage<Animation<CollisionFrameActiveHandle>>>();
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
            world.read_resource::<AssetStorage<Sampler<CollisionFramePrimitive>>>();
        let frame_sampler = frame_sampler_store.get(frame_sampler_handle);
        assert!(frame_sampler.is_some());

        let frame_sampler = frame_sampler.unwrap();
        assert_eq!(vec![0.0, 1.0, 4.0, 6.0], frame_sampler.input);
        assert_eq!(
            vec![
                ActiveHandlePrimitive::Handle(10.into()),
                ActiveHandlePrimitive::Handle(11.into()),
                ActiveHandlePrimitive::Handle(12.into()),
                ActiveHandlePrimitive::Handle(12.into()),
            ],
            frame_sampler.output
        );
        assert_eq!(InterpolationFunction::Step, frame_sampler.function);
    }

    fn test_sequences() -> HashMap<TestSequenceId, TestSequence> {
        // Sheet, Sprite, Wait
        let frames = vec![
            CollisionFrame::new(test_body(), test_interactions(), 0), // TU: 0 to 1
            CollisionFrame::new(test_body(), test_interactions(), 2), // TU: 1 to 4
            CollisionFrame::new(test_body(), test_interactions(), 1), // TU: 4 to 6
        ];
        let sequence = TestSequence::new(frames);
        let mut sequences = HashMap::new();
        sequences.insert(TestSequenceId::Boo, sequence);
        sequences
    }

    fn test_body() -> Option<Vec<Volume>> {
        Some(vec![Volume::Sphere {
            x: 1,
            y: 2,
            z: 3,
            r: 4,
        }])
    }

    fn test_interactions() -> Option<Vec<Interaction>> {
        Some(vec![Interaction::Physical {
            bounds: vec![Volume::Sphere {
                x: 1,
                y: 2,
                z: 3,
                r: 4,
            }],
            hp_damage: 10,
            sp_damage: 20,
            multiple: true,
        }])
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
        frames: Vec<CollisionFrame>,
    }
    impl CollisionAnimationSequence for TestSequence {
        type Frame = CollisionFrame;
        fn frames(&self) -> &[CollisionFrame] {
            &self.frames
        }
    }
}
