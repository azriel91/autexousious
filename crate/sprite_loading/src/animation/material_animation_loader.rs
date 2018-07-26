use std::collections::HashMap;
use std::hash::Hash;

use amethyst::{
    animation::{Animation, InterpolationFunction, MaterialChannel, MaterialPrimitive, Sampler},
    assets::{Handle, Loader},
    prelude::*,
    renderer::{Material, SpriteSheet},
};

use AnimationFrame;
use AnimationSequence;

/// Loads `Animation`s from character sequences.
#[derive(Debug)]
pub struct MaterialAnimationLoader;

impl MaterialAnimationLoader {
    /// Loads `Material` animations and returns a hash map of their handles by sequence ID.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` to load animations into.
    /// * `sequences`: Sequences of the animation.
    /// * `texture_index_offset`: Offset of the texture IDs in the `MaterialTextureSet`.
    /// * `sprite_sheets`: `SpriteSheet`s that contain the texture coordinates for sprites.
    pub fn load_into_map<'seq, SequenceId, Sequence, Frame>(
        world: &'seq World,
        sequences: &HashMap<SequenceId, Sequence>,
        texture_index_offset: u64,
        sprite_sheets: &'seq [SpriteSheet],
    ) -> HashMap<SequenceId, Handle<Animation<Material>>>
    where
        SequenceId: Copy + Eq + Hash + 'seq,
        Frame: AnimationFrame,
        Sequence: AnimationSequence<Frame = Frame> + 'seq,
    {
        Self::load(world, sequences.iter(), texture_index_offset, sprite_sheets)
            .map(|(id, handle)| (*id, handle))
            .collect::<HashMap<SequenceId, Handle<Animation<Material>>>>()
    }

    /// Loads `Material` animations and returns a vector of their handles in order.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` to load animations into.
    /// * `sequences`: Sequences of the animation.
    /// * `texture_index_offset`: Offset of the texture IDs in the `MaterialTextureSet`.
    /// * `sprite_sheets`: `SpriteSheet`s that contain the texture coordinates for sprites.
    pub fn load_into_vec<'seq, Sequences, Sequence, Frame>(
        world: &'seq World,
        sequences: Sequences,
        texture_index_offset: u64,
        sprite_sheets: &'seq [SpriteSheet],
    ) -> Vec<Handle<Animation<Material>>>
    where
        Sequences: Iterator<Item = &'seq Sequence>,
        Frame: AnimationFrame,
        Sequence: AnimationSequence<Frame = Frame> + 'seq,
    {
        sequences
            .map(|sequence| {
                Self::sequence_to_animation(world, texture_index_offset, sprite_sheets, sequence)
            })
            .map(|animation| {
                let loader = world.read_resource::<Loader>();
                loader.load_from_data(animation, (), &world.read_resource())
            })
            .collect::<Vec<Handle<Animation<Material>>>>()
    }

    /// Loads `Material` animations and returns an iterator to their handles by sequence ID.
    ///
    /// The caller is responsible for collecting the elements into the desired collection type.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` to load animations into.
    /// * `sequences`: Sequences of the animation.
    /// * `texture_index_offset`: Offset of the texture IDs in the `MaterialTextureSet`.
    /// * `sprite_sheets`: `SpriteSheet`s that contain the texture coordinates for sprites.
    pub fn load<'seq, Sequences, SequenceId, Sequence, Frame>(
        world: &'seq World,
        sequences: Sequences,
        texture_index_offset: u64,
        sprite_sheets: &'seq [SpriteSheet],
    ) -> impl Iterator<Item = (&'seq SequenceId, Handle<Animation<Material>>)>
    where
        Sequences: Iterator<Item = (&'seq SequenceId, &'seq Sequence)>,
        SequenceId: 'seq,
        Frame: AnimationFrame,
        Sequence: AnimationSequence<Frame = Frame> + 'seq,
    {
        sequences
            .map(move |(id, sequence)| {
                (
                    id,
                    Self::sequence_to_animation(
                        world,
                        texture_index_offset,
                        sprite_sheets,
                        sequence,
                    ),
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
    /// * `texture_index_offset`: Offset of the texture IDs in the `MaterialTextureSet`.
    /// * `sprite_sheets`: `SpriteSheet`s that contain the texture coordinates for sprites.
    /// * `sequence`: `Sequence` to create the animation from.
    fn sequence_to_animation<Sequence: AnimationSequence<Frame = F>, F: AnimationFrame>(
        world: &World,
        texture_index_offset: u64,
        sprite_sheets: &[SpriteSheet],
        sequence: &Sequence,
    ) -> Animation<Material> {
        let frames = sequence.frames();
        let mut input = Vec::with_capacity(frames.len() + 1);
        let mut tick_counter = 0.;
        for frame in frames {
            input.push(tick_counter);
            tick_counter += 1. + frame.wait() as f32;
        }
        input.push(tick_counter);

        let texture_sampler = Self::texture_sampler(texture_index_offset, sequence, input.clone());
        let sprite_offset_sampler = Self::sprite_offset_sampler(sprite_sheets, sequence, input);

        let loader = world.read_resource::<Loader>();
        let texture_sampler_handle =
            loader.load_from_data(texture_sampler, (), &world.read_resource());
        let offset_sampler_handle =
            loader.load_from_data(sprite_offset_sampler, (), &world.read_resource());

        Animation {
            nodes: vec![
                (0, MaterialChannel::AlbedoTexture, texture_sampler_handle),
                (0, MaterialChannel::AlbedoOffset, offset_sampler_handle),
            ],
        }
    }

    fn texture_sampler<Sequence: AnimationSequence<Frame = F>, F: AnimationFrame>(
        texture_index_offset: u64,
        sequence: &Sequence,
        input: Vec<f32>,
    ) -> Sampler<MaterialPrimitive> {
        let mut output = sequence
            .frames()
            .iter()
            .map(|frame: &F| {
                MaterialPrimitive::Texture(texture_index_offset + frame.texture_index() as u64)
            })
            .collect::<Vec<MaterialPrimitive>>();
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

    fn sprite_offset_sampler<Sequence: AnimationSequence<Frame = F>, F: AnimationFrame>(
        sprite_sheets: &[SpriteSheet],
        sequence: &Sequence,
        input: Vec<f32>,
    ) -> Sampler<MaterialPrimitive> {
        let mut output = sequence
            .frames()
            .iter()
            .map(|frame: &F| {
                (&sprite_sheets[frame.texture_index()].sprites[frame.sprite_index()]).into()
            })
            .collect::<Vec<MaterialPrimitive>>();
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
    use std::collections::HashMap;

    use amethyst::{
        animation::{
            Animation, InterpolationFunction, MaterialChannel, MaterialPrimitive, Sampler,
        },
        assets::{AssetStorage, Handle},
        prelude::*,
        renderer::{Material, SpriteSheet},
    };
    use amethyst_test_support::prelude::*;

    use super::MaterialAnimationLoader;
    use AnimationFrame;
    use AnimationSequence;

    #[test]
    fn loads_material_animations_into_map() {
        let effect = |world: &mut World| {
            let texture_index_offset = 10;
            let test_sequences = test_sequences();
            let sprite_sheets = sprite_sheets();
            let animation_handles = MaterialAnimationLoader::load_into_map(
                world,
                &test_sequences,
                texture_index_offset,
                &sprite_sheets,
            );
            world.add_resource(EffectReturn(animation_handles));
        };
        let assertion = |world: &mut World| {
            let animation_handles = &world
                .read_resource::<EffectReturn<HashMap<TestSequenceId, Handle<Animation<Material>>>>>(
                )
                .0;

            // Verify animation is loaded
            verify_animation_handle(world, animation_handles.get(&TestSequenceId::Boo));
        };

        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::render_base("loads_material_animations_into_map", false)
                .with_effect(effect)
                .with_assertion(assertion)
                .run()
                .is_ok()
        );
    }

    #[test]
    fn loads_material_animations_into_vec() {
        let effect = |world: &mut World| {
            let texture_index_offset = 10;
            let test_sequences = test_sequences();
            let sprite_sheets = sprite_sheets();
            let animation_handles = MaterialAnimationLoader::load_into_vec(
                world,
                test_sequences.values(),
                texture_index_offset,
                &sprite_sheets,
            );
            world.add_resource(EffectReturn(animation_handles));
        };
        let assertion = |world: &mut World| {
            let animation_handles = &world
                .read_resource::<EffectReturn<Vec<Handle<Animation<Material>>>>>()
                .0;

            // Verify animation is loaded
            verify_animation_handle(world, animation_handles.first());
        };

        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::render_base("loads_material_animations_into_vec", false)
                .with_effect(effect)
                .with_assertion(assertion)
                .run()
                .is_ok()
        );
    }

    fn verify_animation_handle(
        world: &World,
        animation_handle: Option<&Handle<Animation<Material>>>,
    ) {
        assert!(animation_handle.is_some());

        let animation_handle = animation_handle.unwrap();
        let animation_store = world.read_resource::<AssetStorage<Animation<Material>>>();
        let animation = animation_store.get(animation_handle);
        assert!(animation.is_some());

        let animation = animation.unwrap();
        assert_eq!(2, animation.nodes.len());

        let node_0 = &animation.nodes[0];
        assert_eq!(0, node_0.0);
        assert_eq!(MaterialChannel::AlbedoTexture, node_0.1);

        let node_1 = &animation.nodes[1];
        assert_eq!(0, node_1.0);
        assert_eq!(MaterialChannel::AlbedoOffset, node_1.1);

        // Verify animation samplers
        let texture_sampler_handle = &node_0.2;
        let texture_sampler_store =
            world.read_resource::<AssetStorage<Sampler<MaterialPrimitive>>>();
        let texture_sampler = texture_sampler_store.get(texture_sampler_handle);
        assert!(texture_sampler.is_some());

        let texture_sampler = texture_sampler.unwrap();
        assert_eq!(vec![0.0, 1.0, 4.0, 6.0], texture_sampler.input);
        assert_eq!(
            vec![
                MaterialPrimitive::Texture(10),
                MaterialPrimitive::Texture(11),
                MaterialPrimitive::Texture(10),
                MaterialPrimitive::Texture(10),
            ],
            texture_sampler.output
        );
        assert_eq!(InterpolationFunction::Step, texture_sampler.function);

        let offset_sampler_handle = &node_1.2;
        let offset_sampler_store =
            world.read_resource::<AssetStorage<Sampler<MaterialPrimitive>>>();
        let offset_sampler = offset_sampler_store.get(offset_sampler_handle);
        assert!(offset_sampler.is_some());

        let offset_sampler = offset_sampler.unwrap();
        assert_eq!(vec![0.0, 1.0, 4.0, 6.0], offset_sampler.input);
        assert_eq!(
            vec![
                MaterialPrimitive::Offset((0., 0.5), (1., 0.5)),
                MaterialPrimitive::Offset((1., 0.75), (0.5, 0.75)),
                MaterialPrimitive::Offset((0., 0.5), (1., 0.5)),
                MaterialPrimitive::Offset((0., 0.5), (1., 0.5)),
            ],
            offset_sampler.output
        );
        assert_eq!(InterpolationFunction::Step, offset_sampler.function);
    }

    fn test_sequences() -> HashMap<TestSequenceId, TestSequence> {
        // Sheet, Sprite, Wait
        let frames = vec![
            TestFrame::new(0, 0, 0), // TU: 0 to 1
            TestFrame::new(1, 0, 2), // TU: 1 to 4
            TestFrame::new(0, 0, 1), // TU: 4 to 6
        ];
        let sequence = TestSequence::new(frames);
        let mut sequences = HashMap::new();
        sequences.insert(TestSequenceId::Boo, sequence);
        sequences
    }

    fn sprite_sheets() -> Vec<SpriteSheet> {
        vec![
            SpriteSheet {
                texture_id: 10,
                sprites: vec![[0., 0.5, 1., 0.5].into()],
            },
            SpriteSheet {
                texture_id: 11,
                sprites: vec![[1., 0.75, 0.5, 0.75].into()],
            },
        ]
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
        frames: Vec<TestFrame>,
    }
    impl AnimationSequence for TestSequence {
        type Frame = TestFrame;
        fn frames(&self) -> &[TestFrame] {
            &self.frames
        }
    }
    #[derive(Debug, new)]
    struct TestFrame {
        sheet: usize,
        sprite: usize,
        wait: u32,
    }
    impl AnimationFrame for TestFrame {
        fn texture_index(&self) -> usize {
            self.sheet
        }
        fn sprite_index(&self) -> usize {
            self.sprite
        }
        fn wait(&self) -> u32 {
            self.wait
        }
    }
}
