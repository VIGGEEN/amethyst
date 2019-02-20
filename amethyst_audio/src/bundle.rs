//! ECS audio bundles

use amethyst_assets::Processor;
use amethyst_core::{
    bundle::SystemBundle,
    nalgebra::{alga::general::SubsetOf, Real},
    specs::prelude::DispatcherBuilder,
};
use amethyst_error::Error;
use std::marker::PhantomData;

use crate::{output::Output, source::*, systems::AudioSystem};

/// Audio bundle
///
/// This will only add the audio system and the asset processor for `Source`.
///
/// `DjSystem` must be added separately if you want to use our background music system.
///
/// The generic N type should be the same as the one in `Transform<N>`.
#[derive(Default)]
pub struct AudioBundle<N>(Output, PhantomData<N>);

impl<'a, 'b, N: Real> SystemBundle<'a, 'b> for AudioBundle<N>
where
    N: Real + SubsetOf<f32>,
{
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<(), Error> {
        builder.add(AudioSystem::<N>::new(self.0), "audio_system", &[]);
        builder.add(Processor::<Source>::new(), "source_processor", &[]);
        Ok(())
    }
}
