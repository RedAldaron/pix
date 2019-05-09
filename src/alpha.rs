// alpha.rs     Alpha channel handling.
//
// Copyright (c) 2019  Douglas P Lau
//
use crate::channel::Channel;
use std::marker::PhantomData;

/// [Channel](trait.Channel.html) for defining the opacity of pixels.
///
/// It is the inverse of translucency.
pub trait Alpha<C: Channel>: Copy + Default + From<u8> {

    /// Get the alpha channel value.
    ///
    /// *Zero* is fully transparent, and *one* is fully opaque.
    fn value(&self) -> C;

    /// Linear interpolation
    fn lerp(self, rhs: C) -> Self;
}

/// [Alpha](trait.Alpha.html) channel for fully opaque pixels and
/// [Raster](struct.Raster.html)s.
///
/// Pixel [Format](trait.Format.html)s with opaque alpha channels take less
/// memory than those with [translucent](struct.Translucent.html) ones.
#[derive(Clone, Copy, Default)]
pub struct Opaque<C: Channel> {
    value: PhantomData<C>,
}

impl<C: Channel> From<u8> for Opaque<C> {
    /// Convert from a u8 value.
    fn from(_: u8) -> Self {
        Opaque::default()
    }
}

impl<C: Channel> Alpha<C> for Opaque<C> {

    /// Get the alpha channel value.
    ///
    /// *Zero* is fully transparent, and *one* is fully opaque.
    fn value(&self) -> C {
        C::MAX
    }

    /// Linear interpolation
    fn lerp(self, _rhs: C) -> Self {
        Opaque::default()
    }
}

impl<C, A> From<Translucent<A>> for Opaque<C>
    where C: Channel, A: Channel, C: From<Translucent<A>>
{
    /// Convert from a translucent value.
    fn from(_: Translucent<A>) -> Self {
        Opaque::default()
    }
}

/// [Alpha](trait.Alpha.html) channel for translucent or transparent pixels and
/// [Raster](struct.Raster.html)s.
#[derive(Clone, Copy, Default)]
pub struct Translucent<C: Channel> {
    value: C,
}

impl<C: Channel> Translucent<C> {
    /// Create a new translucent alpha value.
    pub fn new(value: C) -> Self {
        Translucent { value }
    }
}

impl<C: Channel> From<u8> for Translucent<C> {
    /// Convert from a u8 value.
    fn from(c: u8) -> Self {
        let value = c.into();
        Translucent { value }
    }
}

impl<C, A> From<Opaque<A>> for Translucent<C>
    where C: Channel, A: Channel
{
    /// Convert from an opaque value.
    fn from(_: Opaque<A>) -> Self {
        Self::new(C::MAX)
    }
}

impl<C: Channel> Alpha<C> for Translucent<C> {

    /// Get the alpha channel value.
    ///
    /// *Zero* is fully transparent, and *one* is fully opaque.
    fn value(&self) -> C {
        self.value
    }

    /// Linear interpolation
    fn lerp(self, rhs: C) -> Self {
        Self::new(self.value.lerp(rhs, rhs))
    }
}
