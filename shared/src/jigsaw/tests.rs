use image::{Rgba, RgbaImage};

use super::{cfg::Grid, *};

macro_rules! piece {
    ($x:expr, $y:expr; $w:expr, $h:expr) => {
        piece!($x, $y; $w, $h; 0, 0)
    };
    ($x:expr, $y:expr; $w:expr, $h:expr; $ox:expr, $oy:expr) => {
        PieceCfg {
            coords: IVec2::new($x, $y),
            size: IVec2::new($w, $h),
            offset: IVec2::new($ox, $oy),
        }
    };
}

const PIXEL_GRID: Grid = Grid {
    size: IVec2::ONE,
    offset: IVec2::ZERO,
};

// Checks if pieces that at least partially overlap a test image are properly
// extracted by 'safe_view'
#[test]
fn safe_view_valid() {
    const COORDS_EXP_LENGTHS: &[(i32, u32)] = &[
        // (<coord>, <expected length>)
        (-2, 2),
        (0, 4),
        (30, 4),
        (60, 4),
        (62, 2),
    ];

    let img = new_test_image();

    for (y, exp_h) in COORDS_EXP_LENGTHS {
        for (x, exp_w) in COORDS_EXP_LENGTHS {
            eprintln!("testing {x}:{y}");
            let piece = piece!(*x, *y; 4, 4);
            let pos = PIXEL_GRID.coords_to_pixels(piece.coords);

            let res = safe_view(&img, pos, piece.size, piece.offset);
            assert!(res.is_some(), "pos: {pos}\texp size: {:?}", (exp_w, exp_h));

            let res = res.unwrap().0;
            assert_eq!(res.width(), *exp_w, "width (actual vs. expected)");
            assert_eq!(res.height(), *exp_h, "height (actual vs. expected)");
        }
    }
}

// Checks if 'safe_view' properly returns None for pieces with
// non-positive areas
#[test]
fn safe_view_bad_area() {
    const LENGTHS: &[i32] = &[1, 0, -1];
    let img = new_test_image();

    for height in LENGTHS {
        for width in LENGTHS {
            if *width == 1 && *height == 1 {
                continue; // 1x1 is valid; skip it
            }

            let piece = piece!(0, 0; *width, *height);
            let pos = PIXEL_GRID.coords_to_pixels(piece.coords);
            let res = safe_view(&img, pos, piece.size, piece.offset);

            assert!(
                res.is_none(),
                "in: {:?}\tout: {:?}",
                (width, height),
                res.map_or((0, 0), |(v, _)| v.dimensions())
            );
        }
    }
}

// Checks if 'safe_view' properly returns None for pieces that are out-of-bounds
#[test]
fn safe_view_out_of_bounds() {
    const COORDS: &[i32] = &[-4, 30, 64];
    let img = new_test_image();

    for y in COORDS {
        for x in COORDS {
            if *x == 30 && *y == 30 {
                continue; // 30:30 is in-bounds; skip it
            }

            let piece = piece!(*x, *y; 4, 4);
            let pos = PIXEL_GRID.coords_to_pixels(piece.coords);
            let res = safe_view(&img, pos, piece.size, piece.offset);
            assert!(res.is_none(), "pos: {x}:{y}");
        }
    }
}

// Creates a 64x64 red square
fn new_test_image() -> RgbaImage {
    RgbaImage::from_fn(64, 64, |_, _| Rgba([1, 0, 0, 1]))
}
