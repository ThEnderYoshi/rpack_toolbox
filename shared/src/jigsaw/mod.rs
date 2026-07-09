//! This module defines the "Jigsaw" tool.
//!
//! See [`run_job`] for more information.

use std::{
    collections::{HashMap, HashSet},
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
};

use image::{
    GenericImageView, ImageBuffer, Pixel, SubImage,
    imageops::{self, FilterType},
};
use log::{error, info, warn};
use serde::Deserialize;
use walkdir::WalkDir;

use crate::jigsaw::cfg::{IVec2, PieceCfg};

pub mod cfg;

#[cfg(test)]
mod tests;

/// Represents a parsed Jigsaw config file.
#[derive(Debug, Deserialize)]
pub struct JigsawCfg {
    /// The file's display name, shown in logs and the GUI.
    pub name: String,

    /// The config for the input image(s), see [`InputCfg`] for more info.
    #[serde(alias = "in")]
    pub input: cfg::InputCfg,

    /// The config for the output image(s), see [`OutputCfg`] for more info.
    #[serde(alias = "out")]
    pub output: cfg::OutputCfg,
}

impl JigsawCfg {
    /// Applies this configuration on the provided `input` image, returning the
    /// corresponding output image.
    pub fn convert<P, S>(
        &self,
        input: &impl GenericImageView<Pixel = P>,
    ) -> crate::Result<ImageBuffer<P, Vec<S>>>
    where
        P: Pixel<Subpixel = S> + 'static,
        S: 'static,
    {
        // Collect pieces
        let mut pieces = HashMap::new();
        let mut invalid_pieces = HashSet::new();

        for (id, piece) in &self.input.pieces {
            let PieceCfg {
                coords,
                size,
                offset,
            } = piece;

            let pos = self.input.grid.coords_to_pixels(*coords);

            if let Some(piece_offset) = safe_view(input, pos, *size, *offset) {
                pieces.insert(id, piece_offset);
                continue;
            }

            invalid_pieces.insert(id);

            if size.x > 0 && size.y > 0 {
                warn!("Piece '{id}' is out of bounds");
            } else {
                warn!("Piece '{id}' has 0 or negative area");
            }
        }

        // Create output image
        let out_width = self.output.size.x as u32;
        let out_height = self.output.size.y as u32;
        let mut out_buf = ImageBuffer::new(out_width, out_height);

        for (coords, ids) in &self.output.placements {
            let pos = self.output.grid.coords_to_pixels(*coords);

            for id in ids {
                let Some((piece, offset)) = pieces.get(id) else {
                    if !invalid_pieces.contains(id) {
                        error!("Placement '{coords}' has undefined piece '{id}'");
                    }

                    continue;
                };

                // Paste the piece pixel-by-pixel. This lets us safely skip any
                // OOB positions
                let start = pos + *offset;
                let width = piece.width() as i32;
                let height = piece.height() as i32;
                let end = start + IVec2::new(width, height);

                for y in (start.y)..(end.y) {
                    if y < 0 || y >= self.output.size.y {
                        continue;
                    }

                    let y_relative = (y - start.y) as u32;

                    for x in (start.x)..(end.x) {
                        if x < 0 || x >= self.output.size.x {
                            continue;
                        }

                        let x_relative = (x - start.x) as u32;
                        let pixel = piece.get_pixel(x_relative, y_relative);
                        out_buf.put_pixel(x as u32, y as u32, pixel);
                    }
                }
            }
        }

        if self.output.scale <= 0.0 {
            error!("'out.scale' is not positive, image will not be scaled");
            return Ok(out_buf);
        } else if self.output.scale == 1.0 {
            return Ok(out_buf);
        }

        let width = (out_width as f32 * self.output.scale).ceil() as u32;
        let height = (out_height as f32 * self.output.scale).ceil() as u32;
        let out_buf = imageops::resize(&out_buf, width, height, FilterType::Nearest);
        Ok(out_buf)
    }
}

pub async fn run_job(config: PathBuf, input: PathBuf, output: PathBuf) -> crate::Result<()> {
    let config: JigsawCfg = toml::from_str(&fs::read_to_string(config)?)?;
    info!("Loaded config '{}'", config.name);

    if input.is_file() {
        let is_new_dir = !output.exists() && output.extension().is_none();

        // Add input file name to output if it is a dir or doesn't exist (and
        // doesn't have a file extension)
        let output = if is_new_dir || output.is_dir() {
            let file_name = input.file_name().unwrap_or(OsStr::new("out.png"));
            output.join(file_name)
        } else {
            output
        };

        if is_new_dir {
            fs::create_dir_all(&output)?;
        }

        convert_file(&config, &input, &output)?;
    } else if input.is_dir() {
        if !output.exists() {
            fs::create_dir_all(&output)?;
        }

        for (i, entry) in WalkDir::new(input).max_depth(1).into_iter().enumerate() {
            let entry = entry?;

            if !entry.file_type().is_file() || entry.path().extension() != Some(OsStr::new("png")) {
                continue;
            }

            let input = entry.path();

            let file_name = input
                .file_name()
                .map(|f| f.to_os_string())
                .unwrap_or_else(|| format!("out_{}.png", i + 1).into());

            let output = output.join(file_name);
            convert_file(&config, input, &output)?;
        }
    }

    info!("All done!");
    Ok(())
}

fn convert_file(config: &JigsawCfg, input: &Path, output: &Path) -> crate::Result<()> {
    info!(
        "Converting '{}' to '{}'...",
        input.display(),
        output.display(),
    );

    let input = image::open(input)?;
    let out_buf = config.convert(&input)?;
    out_buf.save_with_format(output, image::ImageFormat::Png)?;
    Ok(())
}

/// Safely cuts a piece from `src` and returns it, as well as an offset value to
/// make sure it's corretly placed in the output image.
///
/// Returns [`None`] instead if the piece has zero/negative area or is
/// completely outside of `src`.
fn safe_view<V: GenericImageView>(
    src: &V,
    mut pos: IVec2,
    mut size: IVec2,
    mut offset: IVec2,
) -> Option<(SubImage<&V>, IVec2)> {
    if size.x <= 0 || size.y <= 0 {
        return None;
    }

    let width = src.width() as i32;

    if pos.x + size.x > width {
        size.x -= pos.x + size.x - width;

        if size.x <= 0 {
            return None;
        }
    }

    if pos.x < 0 {
        size.x += pos.x;

        if size.x <= 0 {
            return None;
        }

        offset.x += pos.x;
        pos.x = 0;
    }

    let height = src.height() as i32;

    if pos.y + size.y > height {
        size.y -= pos.y + size.y - height;

        if size.y <= 0 {
            return None;
        }
    }

    if pos.y < 0 {
        size.y += pos.y;

        if size.y <= 0 {
            return None;
        }

        offset.y += pos.y;
        pos.y = 0;
    }

    let view = src.view(pos.x as u32, pos.y as u32, size.x as u32, size.y as u32);
    Some((view, offset))
}
