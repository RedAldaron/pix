// format.rs     Pixel format.
//
// Copyright (c) 2018-2020  Douglas P Lau
// Copyright (c) 2019-2020  Jeron Aldaron Lau
//
use crate::alpha::{self, Mode as _};
use crate::gamma::{self, Mode as _};
use crate::Channel;
use std::any::{Any, TypeId};

/// Pixel format determines [color model], bit depth, [alpha mode] and
/// [gamma mode].
///
/// [alpha mode]: alpha/trait.Mode.html
/// [color model]: trait.ColorModel.html
/// [gamma mode]: gamma/trait.Mode.html
///
/// The naming scheme for type aliases goes:
///
/// * Gamma: `S` for [sRGB](gamma/struct.Srgb.html) colorspace;
///   [linear](gamma/struct.Linear.html) if omitted.
/// * Color model: [Gray](struct.Gray.html) / `GrayAlpha` /
///   [Rgb](struct.Rgb.html) / `Rgba` / [Mask](struct.Mask.html).
/// * Bit depth: `8` / `16` / `32` for 8-bit integer, 16-bit integer, and 32-bit
///   floating-point [channels](trait.Channel.html).
/// * Alpha mode: `p` for [premultiplied](alpha/struct.Premultiplied.html);
///   [straight](alpha/struct.Straight.html) if omitted.
///
/// The following types are defined:
///
/// * Opaque, linear gamma:
///   [Gray8](type.Gray8.html),
///   [Gray16](type.Gray16.html),
///   [Gray32](type.Gray32.html),
///   [Rgb8](type.Rgb8.html),
///   [Rgb16](type.Rgb16.html),
///   [Rgb32](type.Rgb32.html)
/// * Opaque, sRGB gamma:
///   [SGray8](type.SGray8.html),
///   [SGray16](type.SGray16.html),
///   [SGray32](type.SGray32.html),
///   [SRgb8](type.SRgb8.html),
///   [SRgb16](type.SRgb16.html),
///   [SRgb32](type.SRgb32.html)
/// * Translucent (straight alpha), linear gamma:
///   [GrayAlpha8](type.GrayAlpha8.html),
///   [GrayAlpha16](type.GrayAlpha16.html),
///   [GrayAlpha32](type.GrayAlpha32.html)
///   [Rgba8](type.Rgba8.html),
///   [Rgba16](type.Rgba16.html),
///   [Rgba32](type.Rgba32.html)
/// * Translucent (premultiplied alpha), linear gamma:
///   [GrayAlpha8p](type.GrayAlpha8p.html),
///   [GrayAlpha16p](type.GrayAlpha16p.html),
///   [GrayAlpha32p](type.GrayAlpha32p.html)
///   [Rgba8p](type.Rgba8p.html),
///   [Rgba16p](type.Rgba16p.html),
///   [Rgba32p](type.Rgba32p.html)
/// * Translucent (straight alpha), sRGB gamma:
///   [SGrayAlpha8](type.SGrayAlpha8.html),
///   [SGrayAlpha16](type.SGrayAlpha16.html),
///   [SGrayAlpha32](type.SGrayAlpha32.html)
///   [SRgba8](type.SRgba8.html),
///   [SRgba16](type.SRgba16.html),
///   [SRgba32](type.SRgba32.html)
/// * Translucent (premultiplied alpha), sRGB gamma:
///   [SGrayAlpha8p](type.SGrayAlpha8p.html),
///   [SGrayAlpha16p](type.SGrayAlpha16p.html),
///   [SGrayAlpha32p](type.SGrayAlpha32p.html)
///   [SRgba8p](type.SRgba8p.html),
///   [SRgba16p](type.SRgba16p.html),
///   [SRgba32p](type.SRgba32p.html)
/// * Alpha mask:
///   [Mask8](type.Mask8.html),
///   [Mask16](type.Mask16.html),
///   [Mask32](type.Mask32.html)
///
pub trait Format: Any + Clone + Copy + Default + PartialEq {
    /// `Channel` type
    type Chan: Channel;

    /// Alpha mode
    type Alpha: alpha::Mode;

    /// Gamma mode
    type Gamma: gamma::Mode;

    /// Get *red*, *green*, *blue* and *alpha* `Channel`s
    fn rgba(self) -> [Self::Chan; 4];

    /// Make a pixel with given RGBA `Channel`s
    fn with_rgba(rgba: [Self::Chan; 4]) -> Self;

    /// Get channel-wise difference
    fn difference(self, rhs: Self) -> Self;

    /// Check if all `Channel`s are within threshold
    fn within_threshold(self, rhs: Self) -> bool;

    /// Convert a pixel to another `Format`
    ///
    /// * `D` Destination format.
    fn convert<D>(self) -> D
    where
        D: Format,
        D::Chan: From<Self::Chan>,
    {
        let rgba = self.rgba();
        // Convert to destination bit depth
        let mut rgba = [
            D::Chan::from(rgba[0]),
            D::Chan::from(rgba[1]),
            D::Chan::from(rgba[2]),
            D::Chan::from(rgba[3]),
        ];
        if TypeId::of::<Self::Alpha>() != TypeId::of::<D::Alpha>() ||
           TypeId::of::<Self::Gamma>() != TypeId::of::<D::Gamma>()
        {
            convert_alpha_gamma::<Self, D>(&mut rgba);
        }
        D::with_rgba(rgba)
    }
}

/// Convert alpha/gamma between two pixel formats
fn convert_alpha_gamma<S, D>(rgba: &mut [D::Chan; 4])
where
    S: Format,
    D: Format,
{
    // Convert to linear gamma
    rgba[0] = S::Gamma::to_linear(rgba[0]);
    rgba[1] = S::Gamma::to_linear(rgba[1]);
    rgba[2] = S::Gamma::to_linear(rgba[2]);
    if TypeId::of::<S::Alpha>() != TypeId::of::<D::Alpha>() {
        // Decode source alpha
        rgba[0] = S::Alpha::decode(rgba[0], rgba[3]);
        rgba[1] = S::Alpha::decode(rgba[1], rgba[3]);
        rgba[2] = S::Alpha::decode(rgba[2], rgba[3]);
        // Encode destination alpha
        rgba[0] = D::Alpha::encode(rgba[0], rgba[3]);
        rgba[1] = D::Alpha::encode(rgba[1], rgba[3]);
        rgba[2] = D::Alpha::encode(rgba[2], rgba[3]);
    }
    // Convert to destination gamma
    rgba[0] = D::Gamma::from_linear(rgba[0]);
    rgba[1] = D::Gamma::from_linear(rgba[1]);
    rgba[2] = D::Gamma::from_linear(rgba[2]);
}

#[cfg(test)]
mod test {
    use super::super::*;
    use super::*;
    #[test]
    fn gray_to_rgb() {
        assert_eq!(SRgb8::new(0xD9, 0xD9, 0xD9), SGray8::new(0xD9).convert());
        assert_eq!(
            SRgb8::new(0x33, 0x33, 0x33),
            SGray16::new(0x337F).convert()
        );
        assert_eq!(SRgb8::new(0x40, 0x40, 0x40), SGray32::new(0.25).convert());
        assert_eq!(
            SRgb16::new(0x2929, 0x2929, 0x2929),
            SGray8::new(0x29).convert()
        );
        assert_eq!(
            SRgb16::new(0x5593, 0x5593, 0x5593),
            SGray16::new(0x5593).convert()
        );
        assert_eq!(
            SRgb16::new(0xFFFF, 0xFFFF, 0xFFFF),
            SGray32::new(1.0).convert()
        );
        assert_eq!(
            SRgb32::new(0.5019608, 0.5019608, 0.5019608),
            SGray8::new(0x80).convert(),
        );
        assert_eq!(
            SRgb32::new(0.75001144, 0.75001144, 0.75001144),
            SGray16::new(0xC000).convert(),
        );
        assert_eq!(SRgb32::new(0.33, 0.33, 0.33), SGray32::new(0.33).convert());
    }
    #[test]
    fn linear_to_srgb() {
        assert_eq!(
            SRgb8::new(0xEF, 0x8C, 0xC7),
            Rgb8::new(0xDC, 0x43, 0x91).convert()
        );
        assert_eq!(
            SRgb8::new(0x66, 0xF4, 0xB5),
            Rgb16::new(0x2205, 0xE699, 0x7654).convert()
        );
        assert_eq!(
            SRgb8::new(0xBC, 0x89, 0xE0),
            Rgb32::new(0.5, 0.25, 0.75).convert()
        );
    }
    #[test]
    fn srgb_to_linear() {
        assert_eq!(
            Rgb8::new(0xDC, 0x43, 0x92),
            SRgb8::new(0xEF, 0x8C, 0xC7).convert(),
        );
        assert_eq!(
            Rgb8::new(0x22, 0xE7, 0x76),
            SRgb16::new(0x6673, 0xF453, 0xB593).convert(),
        );
        assert_eq!(
            Rgb8::new(0x37, 0x0D, 0x85),
            SRgb32::new(0.5, 0.25, 0.75).convert(),
        );
    }
    #[test]
    fn straight_to_premultiplied() {
        assert_eq!(
            Rgba8p::with_alpha(0x10, 0x20, 0x40, 0x80),
            Rgba8::with_alpha(0x20, 0x40, 0x80, 0x80).convert(),
        );
        assert_eq!(
            Rgba8p::with_alpha(0x04, 0x10, 0x20, 0x40),
            Rgba16::with_alpha(0x1000, 0x4000, 0x8000, 0x4000).convert(),
        );
        assert_eq!(
            Rgba8p::with_alpha(0x60, 0xBF, 0x8F, 0xBF),
            Rgba32::with_alpha(0.5, 1.0, 0.75, 0.75).convert(),
        );
    }
    #[test]
    fn premultiplied_to_straight() {
        assert_eq!(
            Rgba8::with_alpha(0x40, 0x80, 0xFF, 0x80),
            Rgba8p::with_alpha(0x20, 0x40, 0x80, 0x80).convert(),
        );
        assert_eq!(
            Rgba8::with_alpha(0x40, 0xFF, 0x80, 0x40),
            Rgba16p::with_alpha(0x1000, 0x4000, 0x2000, 0x4000).convert(),
        );
        assert_eq!(
            Rgba8::with_alpha(0xAB, 0x55, 0xFF, 0xBF),
            Rgba32p::with_alpha(0.5, 0.25, 0.75, 0.75).convert(),
        );
    }
    #[test]
    fn straight_to_premultiplied_srgb() {
        assert_eq!(
            SRgba8p::with_alpha(0x16, 0x2A, 0x5C, 0x80),
            SRgba8::with_alpha(0x20, 0x40, 0x80, 0x80).convert(),
        );
        assert_eq!(
            SRgba8p::with_alpha(0x0D, 0x1C, 0x40, 0x40),
            SRgba16::with_alpha(0x2000, 0x4000, 0x8000, 0x4000).convert(),
        );
        assert_eq!(
            SRgba8p::with_alpha(0x70, 0xE0, 0xA7, 0xBF),
            SRgba32::with_alpha(0.5, 1.0, 0.75, 0.75).convert(),
        );
    }
}
