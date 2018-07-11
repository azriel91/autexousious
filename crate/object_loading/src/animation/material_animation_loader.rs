use std::collections::HashMap;

use amethyst::{
    animation::{Animation, InterpolationFunction, MaterialChannel, MaterialPrimitive, Sampler},
    assets::{Handle, Loader},
    prelude::*,
    renderer::{Material, SpriteSheet},
};
use object_model::config::{
    object::{Sequence, SequenceId},
    ObjectDefinition,
};

use error::Result;

/// Loads `Animation`s from character sequences.
#[derive(Debug)]
pub(crate) struct MaterialAnimationLoader;

impl MaterialAnimationLoader {
    /// Loads `Material` animations from the object definition, and returns their handles.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` to load animations into.
    /// * `object_definition`: Sequences of the `Object`
    /// * `texture_index_offset`: Offset of the texture IDs in the `MaterialTextureSet`.
    /// * `sprite_sheets`: `SpriteSheet`s that contain the texture coordinates for sprites.
    pub(crate) fn load<SeqId: SequenceId>(
        world: &World,
        object_definition: &ObjectDefinition<SeqId>,
        texture_index_offset: u64,
        sprite_sheets: &[SpriteSheet],
    ) -> Result<HashMap<SeqId, Handle<Animation<Material>>>> {
        let animation_handles = object_definition
            .sequences
            .iter()
            .map(|(id, sequence)| {
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
            .map(|(id, animation)| {
                let loader = world.read_resource::<Loader>();
                let animation_handle = loader.load_from_data(animation, (), &world.read_resource());
                (*id, animation_handle)
            })
            .collect::<HashMap<SeqId, Handle<Animation<Material>>>>();

        Ok(animation_handles)
    }

    /// Maps a `Sequence` into an Amethyst `Animation`.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` to store the `Animation`s.
    /// * `texture_index_offset`: Offset of the texture IDs in the `MaterialTextureSet`.
    /// * `sprite_sheets`: `SpriteSheet`s that contain the texture coordinates for sprites.
    /// * `sequence`: `Sequence` to create the animation from.
    fn sequence_to_animation<SeqId: SequenceId>(
        world: &World,
        texture_index_offset: u64,
        sprite_sheets: &[SpriteSheet],
        sequence: &Sequence<SeqId>,
    ) -> Animation<Material> {
        let mut input = Vec::with_capacity(sequence.frames.len() + 1);
        let mut tick_counter = 0.;
        for frame in &sequence.frames {
            input.push(tick_counter);
            tick_counter += 1. + frame.wait as f32;
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

    fn texture_sampler<SeqId: SequenceId>(
        texture_index_offset: u64,
        sequence: &Sequence<SeqId>,
        input: Vec<f32>,
    ) -> Sampler<MaterialPrimitive> {
        let mut output = sequence
            .frames
            .iter()
            .map(|frame| MaterialPrimitive::Texture(texture_index_offset + frame.sheet as u64))
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

    fn sprite_offset_sampler<SeqId: SequenceId>(
        sprite_sheets: &[SpriteSheet],
        sequence: &Sequence<SeqId>,
        input: Vec<f32>,
    ) -> Sampler<MaterialPrimitive> {
        let mut output = sequence
            .frames
            .iter()
            .map(|frame| (&sprite_sheets[frame.sheet].sprites[frame.sprite]).into())
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
    use object_model::config::{
        object::{Frame, Sequence, SequenceId},
        ObjectDefinition,
    };

    use super::MaterialAnimationLoader;

    #[test]
    fn loads_material_animations() {
        let effect = |world: &mut World| {
            let texture_index_offset = 10;
            let animation_handles = MaterialAnimationLoader::load(
                world,
                &object_definition(),
                texture_index_offset,
                &sprite_sheets(),
            ).expect("Failed to load animations");
            world.add_resource(EffectReturn(animation_handles));
        };
        let assertion = |world: &mut World| {
            let animation_handles = &world
                .read_resource::<EffectReturn<HashMap<TestSeqId, Handle<Animation<Material>>>>>()
                .0;

            // Verify animation is loaded
            let animation_handle = animation_handles.get(&TestSeqId::Boo);
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
            assert_eq!(4, texture_sampler.output.len());
            // Sad, `MaterialPrimitive` doesn't derive `PartialEq`
            // TODO: Pending <https://github.com/amethyst/amethyst/pull/809>
            // assert_eq!(
            //     vec![
            //         MaterialPrimitive::Texture(10),
            //         MaterialPrimitive::Texture(11),
            //         MaterialPrimitive::Texture(10),
            //         MaterialPrimitive::Texture(10),
            //     ],
            //     texture_sampler.output
            // );
            assert_eq!(InterpolationFunction::Step, texture_sampler.function);

            let offset_sampler_handle = &node_1.2;
            let offset_sampler_store =
                world.read_resource::<AssetStorage<Sampler<MaterialPrimitive>>>();
            let offset_sampler = offset_sampler_store.get(offset_sampler_handle);
            assert!(offset_sampler.is_some());

            let offset_sampler = offset_sampler.unwrap();
            assert_eq!(vec![0.0, 1.0, 4.0, 6.0], offset_sampler.input);
            assert_eq!(4, offset_sampler.output.len());
            // TODO: Pending <https://github.com/amethyst/amethyst/pull/809>
            // assert_eq!(
            //     vec![
            //         MaterialPrimitive::Offset((0., 0.5), (1., 0.5)),
            //         MaterialPrimitive::Offset((1., 0.75), (0.5, 0.75)),
            //         MaterialPrimitive::Offset((0., 0.5), (1., 0.5)),
            //         MaterialPrimitive::Offset((0., 0.5), (1., 0.5)),
            //     ],
            //     offset_sampler.output
            // );
            assert_eq!(InterpolationFunction::Step, offset_sampler.function);
        };

        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::render_base("loads_material_animations", false)
                .with_effect(effect)
                .with_assertion(assertion)
                .run()
                .is_ok()
        );
    }

    fn object_definition() -> ObjectDefinition<TestSeqId> {
        // Sheet, Sprite, Wait
        let frames = vec![
            Frame::new(0, 0, 0), // TU: 0 to 1
            Frame::new(1, 0, 2), // TU: 1 to 4
            Frame::new(0, 0, 1), // TU: 4 to 6
        ];
        let sequence = Sequence::new(Some(TestSeqId::Boo), frames);
        let mut sequences = HashMap::new();
        sequences.insert(TestSeqId::Boo, sequence);
        ObjectDefinition::new(sequences)
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

    #[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Hash)]
    enum TestSeqId {
        Boo,
    }
    impl Default for TestSeqId {
        fn default() -> Self {
            TestSeqId::Boo
        }
    }
    impl SequenceId for TestSeqId {}
}
