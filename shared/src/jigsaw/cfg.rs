//! This module defines the sub-elements of the [`JigsawCfg`] struct.
//!
//! [`JigsawCfg`]: super::JigsawCfg

use std::{
    collections::HashMap,
    fmt::Display,
    ops::{Add, Mul},
    str::FromStr,
};

use serde::Deserialize;
use serde_with::{DeserializeFromStr, StringWithSeparator, formats::SpaceSeparator, serde_as};

/// Defines the configuration for the input image(s).
#[derive(Debug, Deserialize)]
pub struct InputCfg {
    /// The [`Grid`] used for the input image(s).
    #[serde(default)]
    pub grid: Grid,

    /// Determines how to cut the jigsaw pieces from the input image(s).
    pub pieces: HashMap<String, PieceCfg>,
}

/// Defines the configuration for the output image(s).
#[serde_as]
#[derive(Debug, Deserialize)]
pub struct OutputCfg {
    /// The size of the output image(s), in pixels.
    pub size: IVec2,

    /// The [`Grid`] used for the output image(s).
    #[serde(default)]
    pub grid: Grid,

    /// A scale factor applied to the output image(s) after all
    /// other processing.
    ///
    /// Defaults to `2.0`.
    #[serde(default = "OutputCfg::default_scale")]
    pub scale: f32,

    /// Determines how to place the [jigsaw pieces][InputConfig::pieces] on the
    /// output image.
    ///
    /// The keys are grid coordinates and the values are space-separated lists
    /// of piece IDs.
    #[serde_as(as = "HashMap<_, StringWithSeparator::<SpaceSeparator, String>>")]
    pub placements: HashMap<IVec2, Vec<String>>,
}

impl OutputCfg {
    #[inline]
    fn default_scale() -> f32 {
        2.0
    }
}

/// Defines a jigsaw piece to cut from the input image(s).
///
/// This type's [`FromStr`] implementation expects the string to be three
/// [`IVec2`]s separated by 1+ whitespace chars each and surrounded by 0+
/// whitespace characters, e.g.: `0:1 2:3 4:5`
#[derive(Debug, DeserializeFromStr)]
pub struct PieceCfg {
    /// The coordinates of this piece's top-left grid cell.
    pub coords: IVec2,

    /// The size of the piece, in pixels.
    pub size: IVec2,

    /// An offset relative to the piece's [placement][PlacementCfg], in pixels.
    ///
    /// This field lets you place multiple pieces at slightly different
    /// positions using the same placement.
    pub offset: IVec2,
}

impl FromStr for PieceCfg {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (coords, remainder) = s
            .trim_start()
            .split_once(' ')
            .ok_or_else(|| crate::Error::parse_piece(s))?;

        let coords = coords.parse().map_err(|_| crate::Error::parse_piece(s))?;

        let (size, offset) = remainder
            .trim_start()
            .split_once(' ')
            .map(|(s, o)| (s, o.trim()))
            .ok_or_else(|| crate::Error::parse_piece(s))?;

        let size = size.parse().map_err(|_| crate::Error::parse_piece(s))?;
        let offset = offset.parse().map_err(|_| crate::Error::parse_piece(s))?;

        Ok(Self {
            coords,
            size,
            offset,
        })
    }
}

/// Defines a rectangular grid of tiles.
#[derive(Debug, Deserialize)]
pub struct Grid {
    /// The size of each grid cell, in pixels. Defaults to `1:1`.
    #[serde(default = "Grid::default_size")]
    pub size: IVec2,

    /// The offset of cell `0:0`, from the top-left of the canvas, in pixels.
    /// Defaults to `0:0` (no offset).
    #[serde(default = "Grid::default_offset")]
    pub offset: IVec2,
}

impl Grid {
    /// Converts a position in tile coordinates to one in pixels.
    pub fn coords_to_pixels(&self, coords: IVec2) -> IVec2 {
        coords * self.size + self.offset
    }

    const fn default_size() -> IVec2 {
        IVec2 { x: 1, y: 1 }
    }

    const fn default_offset() -> IVec2 {
        IVec2 { x: 0, y: 0 }
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            size: IVec2::ONE,
            offset: IVec2::ZERO,
        }
    }
}

/// A 2D vector whose components are [`i32`]s.
///
/// This type's [`FromStr`] implementation expects strings to be in this format:
/// `<x>:<y>`, where `<x>` and `<y>` are valid [`i32`]s, e.g.: `1:2`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DeserializeFromStr)]
pub struct IVec2 {
    /// The vector's X component.
    pub x: i32,

    /// The vector's Y component.
    pub y: i32,
}

impl IVec2 {
    /// A vector whose components are all `0`.
    pub const ZERO: Self = Self::new(0, 0);

    /// A vector whose components are all `1`.
    pub const ONE: Self = Self::new(1, 1);

    /// Creates a new vector with the provided components.
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Add for IVec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Mul for IVec2 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl Display for IVec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.x, self.y)
    }
}

impl FromStr for IVec2 {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s
            .split_once(':')
            .ok_or_else(|| crate::Error::parse_ivec2(s))?;

        let x = x.parse().map_err(|_| crate::Error::parse_ivec2(s))?;
        let y = y.parse().map_err(|_| crate::Error::parse_ivec2(s))?;
        Ok(Self { x, y })
    }
}
