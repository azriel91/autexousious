use amethyst::animation::{
    Animation, InterpolationFunction, MaterialChannel, MaterialPrimitive, Sampler,
};
use amethyst::assets::{Handle, Loader};
use amethyst::prelude::*;
use amethyst::renderer::{Material, SpriteSheet};
use object_model::config::object::Sequence;
use object_model::config::ObjectDefinition;

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
    pub(crate) fn load<SeqId>(
        world: &World,
        object_definition: &ObjectDefinition<SeqId>,
        texture_index_offset: usize,
        sprite_sheets: &[SpriteSheet],
    ) -> Result<Vec<Handle<Animation<Material>>>> {
        let animation_handles = object_definition
            .sequences
            .iter()
            .map(|sequence| {
                Self::into_animation(world, texture_index_offset, sprite_sheets, sequence)
            })
            .map(|animation| {
                let loader = world.read_resource::<Loader>();
                loader.load_from_data(animation, (), &world.read_resource())
            })
            .collect::<Vec<Handle<Animation<Material>>>>();

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
    fn into_animation<SeqId>(
        world: &World,
        texture_index_offset: usize,
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
        let texture_animation_handle =
            loader.load_from_data(texture_sampler, (), &world.read_resource());
        let sampler_animation_handle =
            loader.load_from_data(sprite_offset_sampler, (), &world.read_resource());

        Animation {
            nodes: vec![
                (0, MaterialChannel::AlbedoTexture, texture_animation_handle),
                (0, MaterialChannel::AlbedoOffset, sampler_animation_handle),
            ],
        }
    }

    fn texture_sampler<SeqId>(
        texture_index_offset: usize,
        sequence: &Sequence<SeqId>,
        input: Vec<f32>,
    ) -> Sampler<MaterialPrimitive> {
        let mut output = sequence
            .frames
            .iter()
            .map(|frame| MaterialPrimitive::Texture(texture_index_offset + frame.sheet as usize))
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

    fn sprite_offset_sampler<SeqId>(
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
