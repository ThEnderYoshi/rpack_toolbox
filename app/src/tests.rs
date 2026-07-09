use std::{ffi::OsStr, fs, path::PathBuf};

use assert_cmd::Command;
use assert_fs::TempDir;
use image::GenericImageView;

// Runs the Jigsaw tool for each config file in '/jigsaw/' and checks if the
// generated images match up with pre-made ones
#[test]
fn jigsaw_test_built_in_cfgs() {
    let out_dir = TempDir::new().unwrap();
    let output = out_dir.path().join("out.png");

    for entry in fs::read_dir("../jigsaw").unwrap() {
        let entry = entry.unwrap();
        let config = entry.path();

        if !config.is_file() || config.extension() != Some(OsStr::new("toml")) {
            continue;
        }

        eprintln!("Testing cfg '{}'", config.display());
        let input = config.with_extension("png");

        // Run jigsaw command
        let mut cmd = Command::cargo_bin("rpack_toolbox").unwrap();
        cmd.arg("jigsaw").arg(config).arg(&input).arg(&output);
        cmd.assert().success();

        // Compare output image with pre-made image
        let output = image::open(&output).expect("jigsaw should produce an output image");
        let expect = PathBuf::from("../tests/jigsaw_cfgs").join(input.file_name().unwrap());

        let expect =
            image::open(&expect).unwrap_or_else(|_| panic!("'{}' should exist", expect.display()));

        assert_eq!(output.dimensions(), expect.dimensions());

        for y in 0..output.height() {
            for x in 0..output.width() {
                let px_out = output.get_pixel(x, y);
                let px_exp = expect.get_pixel(x, y);
                assert_eq!(px_out, px_exp, "checking image mismatch");
            }
        }
    }

    drop(out_dir); // Just making sure
}
