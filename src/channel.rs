// channel.rs       Color channels
//
// Copyright (c) 2019-2020  Douglas P Lau
// Copyright (c) 2019-2020  Jeron Aldaron Lau
//
use crate::gamma::SrgbValue;
use std::any::Any;
use std::cmp::Ordering;
use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Sub};

/// A [color model] *component* which determines *bit depth*.
///
/// Existing `Channel`s are [Ch8], [Ch16] and [Ch32].
///
/// This trait is *sealed*, and cannot be implemented outside of this crate.
///
/// [Ch8]: struct.Ch8.html
/// [Ch16]: struct.Ch16.html
/// [Ch32]: struct.Ch32.html
/// [color model]: trait.ColorModel.html
pub trait Channel:
    Any
    + Copy
    + Debug
    + Default
    + From<f32>
    + Ord
    + Add<Output = Self>
    + Div<Output = Self>
    + Mul<Output = Self>
    + Sub<Output = Self>
    + SrgbValue
{
    /// Minimum intensity (*zero*)
    const MIN: Self;

    /// Maximum intensity (*one*)
    const MAX: Self;

    /// Raise to given power
    fn powf(self, g: f32) -> Self;
}

/// 8-bit color [Channel](trait.Channel.html).
///
/// The `Channel` is represented by a `u8`, but multiplication and division
/// treat values as though they range between 0 and 1.
///
/// ```
/// # use pix::*;
/// let c: Ch8 = std::u8::MIN.into();
/// assert_eq!(c, Ch8::MIN);
/// let c: Ch16 = c.into();
/// assert_eq!(c, Ch16::MIN);
/// let c: Ch8 = std::u8::MAX.into();
/// assert_eq!(c, Ch8::MAX);
/// let c: Ch32 = c.into();
/// assert_eq!(c, Ch32::MAX);
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ch8(u8);

/// 16-bit color [Channel](trait.Channel.html).
///
/// The `Channel` is represented by a `u16`, but multiplication and division
/// treat values as though they range between 0 and 1.
///
/// ```
/// # use pix::*;
/// let c: Ch16 = std::u16::MIN.into();
/// assert_eq!(c, Ch16::MIN);
/// let c: Ch8 = c.into();
/// assert_eq!(c, Ch8::MIN);
/// let c: Ch16 = std::u16::MAX.into();
/// assert_eq!(c, Ch16::MAX);
/// let c: Ch32 = c.into();
/// assert_eq!(c, Ch32::MAX);
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ch16(u16);

/// 32-bit color [Channel](trait.Channel.html).
///
/// The `Channel` is represented by an `f32`, but values are guaranteed to be
/// between 0 and 1, inclusive.
///
/// ```
/// # use pix::*;
/// let c: Ch32 = 0.0.into();
/// assert_eq!(c, Ch32::MIN);
/// let c: Ch8 = c.into();
/// assert_eq!(c, Ch8::MIN);
/// let c: Ch32 = 1.0.into();
/// assert_eq!(c, Ch32::MAX);
/// let c: Ch16 = c.into();
/// assert_eq!(c, Ch16::MAX);
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Ch32(f32);

impl Ch8 {
    /// Create a new 8-bit `Channel` value.
    pub fn new(value: u8) -> Self {
        Ch8(value)
    }
}

impl Channel for Ch8 {
    /// Minimum intensity (*zero*)
    const MIN: Ch8 = Ch8(0);

    /// Maximum intensity (*one*)
    const MAX: Ch8 = Ch8(0xFF);

    /// Raise to given power
    fn powf(self, g: f32) -> Self {
        let v = f32::from(Ch32::from(self)).powf(g);
        Ch32::new(v).into()
    }
}

impl From<u8> for Ch8 {
    fn from(value: u8) -> Self {
        Ch8(value)
    }
}

impl From<f32> for Ch8 {
    fn from(value: f32) -> Self {
        Ch32::new(value).into()
    }
}

impl From<Ch8> for u8 {
    fn from(c: Ch8) -> u8 {
        c.0
    }
}

impl<R> Add<R> for Ch8
where
    Self: From<R>,
{
    type Output = Self;
    fn add(self, rhs: R) -> Self {
        let rhs = Self::from(rhs);
        Ch8(self.0.saturating_add(rhs.0))
    }
}

impl<R> Sub<R> for Ch8
where
    Self: From<R>,
{
    type Output = Self;
    fn sub(self, rhs: R) -> Self {
        let rhs = Self::from(rhs);
        Ch8(self.0.saturating_sub(rhs.0))
    }
}

impl<R> Mul<R> for Ch8
where
    Self: From<R>,
{
    type Output = Self;
    fn mul(self, rhs: R) -> Self {
        let rhs = Self::from(rhs);
        let l = u32::from(self.0);
        let l = (l << 4) | (l >> 4);
        let r = u32::from(rhs.0);
        let r = (r << 4) | (r >> 4);
        let value = ((l * r) >> 16) as u8;
        Ch8(value)
    }
}

impl<R> Div<R> for Ch8
where
    Self: From<R>,
{
    type Output = Self;
    fn div(self, rhs: R) -> Self {
        #![allow(clippy::single_match, clippy::suspicious_arithmetic_impl)]
        let rhs = Self::from(rhs);
        if rhs.0 > 0 {
            let ss = u32::from(self.0) << 8;
            let rr = u32::from(rhs.0);
            let value = (ss / rr).min(255) as u8;
            Ch8(value)
        } else {
            Ch8(0)
        }
    }
}

impl Ch16 {
    /// Create a new 16-bit `Channel` value.
    pub fn new(value: u16) -> Self {
        Ch16(value)
    }
}

impl Channel for Ch16 {
    /// Minimum intensity (*zero*)
    const MIN: Ch16 = Ch16(0);

    /// Maximum intensity (*one*)
    const MAX: Ch16 = Ch16(0xFFFF);

    /// Raise to given power
    fn powf(self, g: f32) -> Self {
        let v = f32::from(Ch32::from(self)).powf(g);
        Ch32::new(v).into()
    }
}

impl From<Ch8> for Ch16 {
    fn from(c: Ch8) -> Self {
        let value = u16::from(c.0);
        Ch16(value << 8 | value)
    }
}

impl From<u16> for Ch16 {
    fn from(value: u16) -> Self {
        Ch16(value)
    }
}

impl From<f32> for Ch16 {
    fn from(value: f32) -> Self {
        Ch32::new(value).into()
    }
}

impl From<Ch16> for u16 {
    fn from(c: Ch16) -> u16 {
        c.0
    }
}

impl From<Ch16> for Ch8 {
    fn from(c: Ch16) -> Self {
        Ch8::new((c.0 >> 8) as u8)
    }
}

impl<R> Add<R> for Ch16
where
    Self: From<R>,
{
    type Output = Self;
    fn add(self, rhs: R) -> Self {
        let rhs = Self::from(rhs);
        Ch16(self.0.saturating_add(rhs.0))
    }
}

impl<R> Sub<R> for Ch16
where
    Self: From<R>,
{
    type Output = Self;
    fn sub(self, rhs: R) -> Self {
        let rhs = Self::from(rhs);
        Ch16(self.0.saturating_sub(rhs.0))
    }
}

impl<R> Mul<R> for Ch16
where
    Self: From<R>,
{
    type Output = Self;
    fn mul(self, rhs: R) -> Self {
        let rhs = Self::from(rhs);
        let l = u64::from(self.0);
        let l = (l << 8) | (l >> 8);
        let r = u64::from(rhs.0);
        let r = (r << 8) | (r >> 8);
        let value = ((l * r) >> 32) as u16;
        Ch16(value)
    }
}

impl<R> Div<R> for Ch16
where
    Self: From<R>,
{
    type Output = Self;
    fn div(self, rhs: R) -> Self {
        #![allow(clippy::single_match, clippy::suspicious_arithmetic_impl)]
        let rhs = Self::from(rhs);
        if rhs.0 > 0 {
            let ss = u64::from(self.0) << 16;
            let rr = u64::from(rhs.0);
            let value = (ss / rr).min(65535) as u16;
            Ch16(value)
        } else {
            Ch16(0)
        }
    }
}

impl Ch32 {
    /// Create a new 32-bit `Channel` value.
    ///
    /// Returns [MIN](trait.Channel.html#associatedconstant.MIN) if value is
    ///         less than 0.0, or `NaN`.
    /// Returns [MAX](trait.Channel.html#associatedconstant.MAX) if value is
    ///         greater than 1.0.
    pub fn new(value: f32) -> Self {
        let v = if value.is_nan() || value < 0.0 {
            0.0
        } else if value > 1.0 {
            1.0
        } else {
            value
        };
        Ch32(v)
    }
}

impl Channel for Ch32 {
    /// Minimum intensity (*zero*)
    const MIN: Ch32 = Ch32(0.0);

    /// Maximum intensity (*one*)
    const MAX: Ch32 = Ch32(1.0);

    /// Raise to given power
    fn powf(self, g: f32) -> Self {
        Ch32::new(self.0.powf(g))
    }
}

impl From<Ch8> for Ch32 {
    fn from(c: Ch8) -> Self {
        Ch32(f32::from(c.0) / 255.0)
    }
}

impl From<f32> for Ch32 {
    fn from(value: f32) -> Self {
        Ch32::new(value)
    }
}

impl From<Ch32> for f32 {
    fn from(c: Ch32) -> f32 {
        c.0
    }
}

impl From<Ch32> for Ch8 {
    fn from(c: Ch32) -> Self {
        let value = c.0;
        debug_assert!(value >= 0.0 && value <= 1.0);
        // this cast is not UB since the value is guaranteed
        // to be between 0.0 and 1.0 (see bug #10184)
        Ch8::new((value * 255.0).round() as u8)
    }
}

impl From<Ch32> for Ch16 {
    fn from(c: Ch32) -> Self {
        let value = c.0;
        debug_assert!(value >= 0.0 && value <= 1.0);
        // this cast is not UB since the value is guaranteed
        // to be between 0.0 and 1.0 (see bug #10184)
        Ch16::new((value * 65535.0).round() as u16)
    }
}

impl From<Ch16> for Ch32 {
    fn from(c: Ch16) -> Self {
        Ch32(f32::from(c.0) / 65535.0)
    }
}

impl Eq for Ch32 {}

impl Ord for Ch32 {
    fn cmp(&self, other: &Ch32) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<R> Add<R> for Ch32
where
    f32: From<R>,
{
    type Output = Self;
    fn add(self, rhs: R) -> Self {
        let value = self.0 + f32::from(rhs);
        Ch32(value.min(1.0))
    }
}

impl<R> Sub<R> for Ch32
where
    f32: From<R>,
{
    type Output = Self;
    fn sub(self, rhs: R) -> Self {
        let value = self.0 - f32::from(rhs);
        Ch32(value.max(0.0))
    }
}

impl<R> Mul<R> for Ch32
where
    f32: From<R>,
{
    type Output = Self;
    fn mul(self, rhs: R) -> Self {
        Ch32(self.0 * f32::from(rhs))
    }
}

impl<R> Div<R> for Ch32
where
    f32: From<R>,
{
    type Output = Self;
    fn div(self, rhs: R) -> Self {
        let v = f32::from(rhs);
        if v > 0.0 {
            Ch32((self.0 / v).min(1.0))
        } else {
            Ch32(0.0)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn ch8_into() {
        assert_eq!(Ch8::new(255), 255.into());
        assert_eq!(Ch8::new(128), 128.into());
        assert_eq!(Ch8::new(64), 64.into());
        assert_eq!(Ch8::new(32), 32.into());
        for i in 0..=255 {
            let c8 = Ch8::new(i);
            let c16: Ch16 = c8.into();
            assert_eq!(c8, c16.into());
        }
        assert_eq!(Ch16::new(0), Ch8::new(0).into());
        assert_eq!(Ch8::new(128), Ch16::new(32768).into());
        assert_eq!(Ch16::new(65535), Ch8::new(255).into());
        assert_eq!(Ch32::new(0.0), Ch8::new(0).into());
        assert_eq!(Ch8::new(128), Ch32::new(0.5).into());
        assert_eq!(Ch32::new(1.0), Ch8::new(255).into());
    }
    #[test]
    fn ch16_into() {
        assert_eq!(Ch16::new(65535), 65535.into());
        assert_eq!(Ch16::new(32768), 32768.into());
        assert_eq!(Ch16::new(16384), 16384.into());
        assert_eq!(Ch16::new(8192), 8192.into());
    }
    #[test]
    fn ch32_into() {
        assert_eq!(Ch32::new(1.0), 1.0.into());
        assert_eq!(Ch32::new(0.5), 0.5.into());
        assert_eq!(Ch32::new(0.25), 0.25.into());
        assert_eq!(Ch32::new(0.125), 0.125.into());
    }
    #[test]
    fn ch8_mul() {
        assert_eq!(Ch8::new(255), Ch8::new(255) * 1.0);
        assert_eq!(Ch8::new(128), Ch8::new(255) * 0.5);
        assert_eq!(Ch8::new(64), Ch8::new(255) * 0.25);
        assert_eq!(Ch8::new(32), Ch8::new(255) * 0.125);
        assert_eq!(Ch8::new(16), Ch8::new(255) * 0.0625);
        assert_eq!(Ch8::new(64), Ch8::new(128) * 0.5);
        assert_eq!(Ch8::new(32), Ch8::new(128) * 0.25);
        assert_eq!(Ch8::new(16), Ch8::new(128) * 0.125);
        assert_eq!(Ch8::new(8), Ch8::new(128) * 0.0625);
    }
    #[test]
    fn ch8_div() {
        assert_eq!(Ch8::new(255), Ch8::new(255) / 1.0);
        assert_eq!(Ch8::new(255), Ch8::new(128) / 0.5);
        assert_eq!(Ch8::new(255), Ch8::new(64) / 0.25);
        assert_eq!(Ch8::new(255), Ch8::new(32) / 0.125);
        assert_eq!(Ch8::new(255), Ch8::new(16) / 0.0625);
        assert_eq!(Ch8::new(128), Ch8::new(128) / 1.0);
        assert_eq!(Ch8::new(128), Ch8::new(64) / 0.5);
        assert_eq!(Ch8::new(128), Ch8::new(32) / 0.25);
        assert_eq!(Ch8::new(128), Ch8::new(16) / 0.125);
        assert_eq!(Ch8::new(64), Ch8::new(64) / 1.0);
        assert_eq!(Ch8::new(64), Ch8::new(32) / 0.5);
        assert_eq!(Ch8::new(64), Ch8::new(16) / 0.25);
    }
    #[test]
    fn ch16_mul() {
        assert_eq!(Ch16::new(65535), Ch16::new(65535) * 1.0);
        assert_eq!(Ch16::new(32768), Ch16::new(65535) * 0.5);
        assert_eq!(Ch16::new(16384), Ch16::new(65535) * 0.25);
        assert_eq!(Ch16::new(8192), Ch16::new(65535) * 0.125);
        assert_eq!(Ch16::new(4096), Ch16::new(65535) * 0.0625);
        assert_eq!(Ch16::new(16384), Ch16::new(32768) * 0.5);
        assert_eq!(Ch16::new(8192), Ch16::new(32768) * 0.25);
        assert_eq!(Ch16::new(4096), Ch16::new(32768) * 0.125);
        assert_eq!(Ch16::new(2048), Ch16::new(32768) * 0.0625);
    }
    #[test]
    fn ch16_div() {
        assert_eq!(Ch16::new(65535), Ch16::new(65535) / 1.0);
        assert_eq!(Ch16::new(65535), Ch16::new(32768) / 0.5);
        assert_eq!(Ch16::new(65535), Ch16::new(16384) / 0.25);
        assert_eq!(Ch16::new(65535), Ch16::new(8192) / 0.125);
        assert_eq!(Ch16::new(65535), Ch16::new(4096) / 0.0625);
        assert_eq!(Ch16::new(32768), Ch16::new(32768) / 1.0);
        assert_eq!(Ch16::new(32768), Ch16::new(16384) / 0.5);
        assert_eq!(Ch16::new(32768), Ch16::new(8192) / 0.25);
        assert_eq!(Ch16::new(32768), Ch16::new(4096) / 0.125);
        assert_eq!(Ch16::new(16384), Ch16::new(16384) / 1.0);
        assert_eq!(Ch16::new(16384), Ch16::new(8192) / 0.5);
        assert_eq!(Ch16::new(16384), Ch16::new(4096) / 0.25);
    }
    #[test]
    fn ch32_mul() {
        assert_eq!(Ch32::new(1.0), Ch32::new(1.0) * 1.0);
        assert_eq!(Ch32::new(0.5), Ch32::new(1.0) * 0.5);
        assert_eq!(Ch32::new(0.25), Ch32::new(1.0) * 0.25);
        assert_eq!(Ch32::new(0.125), Ch32::new(1.0) * 0.125);
        assert_eq!(Ch32::new(0.0625), Ch32::new(1.0) * 0.0625);
        assert_eq!(Ch32::new(0.25), Ch32::new(0.5) * 0.5);
        assert_eq!(Ch32::new(0.125), Ch32::new(0.5) * 0.25);
        assert_eq!(Ch32::new(0.0625), Ch32::new(0.5) * 0.125);
        assert_eq!(Ch32::new(0.03125), Ch32::new(0.5) * 0.0625);
    }
}
