// alpha.rs     Alpha channel handling.
//
// Copyright (c) 2019-2020  Douglas P Lau
// Copyright (c) 2019-2020  Jeron Aldaron Lau
//
//! Module for alpha channel items
use crate::private::Sealed;
use crate::{Ch16, Ch32, Ch8, Channel};
use std::any::Any;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::Mul;

/// Alpha [channel](../trait.Channel.html) for defining the opacity of pixels.
///
/// It is the inverse of translucency.
///
/// This trait is *sealed*, and cannot be implemented outside of this crate.
pub trait AChannel:
    Any + Copy + Debug + Default + Mul<Output = Self> + PartialEq + Sealed
{
    /// `Channel` type
    type Chan: Channel;

    /// Get the alpha `Channel` value.
    ///
    /// [Channel::MIN](../trait.Channel.html#associatedconstant.MIN) is fully
    /// transparent, and
    /// [Channel::MAX](../trait.Channel.html#associatedconstant.MAX) is fully
    /// opaque.
    fn value(&self) -> Self::Chan;
}

/// [Alpha channel](trait.AChannel.html) for fully opaque pixels and
/// [Raster](../struct.Raster.html)s.
///
/// [Pixel](../trait.Pixel.html) formats with `Opaque` alpha channels take less
/// memory than those with [translucent](struct.Translucent.html) ones.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Opaque<C> {
    value: PhantomData<C>,
}

/// [Alpha channel](trait.AChannel.html) for translucent or transparent pixels
/// and [Raster](../struct.Raster.html)s.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Translucent<C: Channel> {
    value: C,
}

/// Trait for handling straight versus premultiplied alpha.
///
/// This trait is *sealed*, and cannot be implemented outside of this crate.
pub trait Mode:
    Any + Copy + Clone + Debug + Default + PartialEq + Sealed
{
    /// Encode one `Channel` using the alpha mode.
    fn encode<C: Channel>(c: C, a: C) -> C;
    /// Decode one `Channel` using the alpha mode.
    fn decode<C: Channel>(c: C, a: C) -> C;
}

/// Each `Channel` is "straight" (not premultiplied with alpha)
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Straight;

/// Each `Channel` is premultiplied, or associated, with alpha
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Premultiplied;

impl<C, H> From<H> for Opaque<C>
where
    C: Channel + From<H>,
    H: Channel,
{
    fn from(_value: H) -> Self {
        Opaque::default()
    }
}
impl<C: Channel> From<Opaque<C>> for Ch8 {
    fn from(_value: Opaque<C>) -> Self {
        Ch8::MAX
    }
}
impl<C: Channel> From<Opaque<C>> for Ch16 {
    fn from(_value: Opaque<C>) -> Self {
        Ch16::MAX
    }
}
impl<C: Channel> From<Opaque<C>> for Ch32 {
    fn from(_value: Opaque<C>) -> Self {
        Ch32::MAX
    }
}

impl<C, A> From<Translucent<A>> for Opaque<C>
where
    C: Channel,
    A: Channel,
{
    /// Convert from a `Translucent` value.
    fn from(_: Translucent<A>) -> Self {
        Opaque::default()
    }
}

impl<C: Channel> Mul<Self> for Opaque<C> {
    type Output = Self;
    fn mul(self, _rhs: Self) -> Self {
        self
    }
}

impl<C: Channel> AChannel for Opaque<C> {
    type Chan = C;

    /// Get the alpha `Channel` value.
    ///
    /// Always returns
    /// [Channel::MAX](../trait.Channel.html#associatedconstant.MAX) (fully
    /// opaque).
    fn value(&self) -> C {
        C::MAX
    }
}

impl<C, H> From<H> for Translucent<C>
where
    C: Channel + From<H>,
    H: Channel,
{
    fn from(value: H) -> Self {
        let value = value.into();
        Translucent { value }
    }
}
impl From<u8> for Translucent<Ch8> {
    fn from(value: u8) -> Self {
        Ch8::new(value).into()
    }
}
impl From<u16> for Translucent<Ch16> {
    fn from(value: u16) -> Self {
        Ch16::new(value).into()
    }
}
impl From<f32> for Translucent<Ch32> {
    fn from(value: f32) -> Self {
        Ch32::new(value).into()
    }
}

impl<C: Channel> Mul<Self> for Translucent<C> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let value = self.value * rhs.value;
        Translucent { value }
    }
}

impl<C: Channel> Translucent<C> {
    /// Create a new `Translucent` alpha value.
    pub fn new(value: C) -> Self {
        Translucent { value }
    }
}

impl<C, A> From<Opaque<A>> for Translucent<C>
where
    C: Channel,
    A: Channel,
{
    /// Convert from an `Opaque` value.
    fn from(_: Opaque<A>) -> Self {
        Self::new(C::MAX)
    }
}

impl<C: Channel> AChannel for Translucent<C> {
    type Chan = C;

    /// Get the alpha `Channel` value.
    ///
    /// [Channel::MIN](../trait.Channel.html#associatedconstant.MIN) is fully
    /// transparent, and
    /// [Channel::MAX](../trait.Channel.html#associatedconstant.MAX) is fully
    /// opaque.
    fn value(&self) -> C {
        self.value
    }
}

impl Mode for Straight {
    /// Encode one `Channel` using the alpha mode.
    fn encode<C: Channel>(c: C, _a: C) -> C {
        c
    }
    /// Decode one `Channel` using the alpha mode.
    fn decode<C: Channel>(c: C, _a: C) -> C {
        c
    }
}

impl Mode for Premultiplied {
    /// Encode one `Channel` using the alpha mode.
    fn encode<C: Channel>(c: C, a: C) -> C {
        c * a
    }
    /// Decode one `Channel` using the alpha mode.
    fn decode<C: Channel>(c: C, a: C) -> C {
        c / a
    }
}
