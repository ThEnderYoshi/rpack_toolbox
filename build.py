#!/usr/bin/env python3
"""build.py v1.0.0

    For Python 3.12.3
    By ThEnderYoshi, 2026
    Under the GPL-3.0 license

    SYNOPSIS:
        python3 build.py

    DESCRIPTION:
        Builds RPack Toolbox in release mode and creates zip files for
        each supported target.

        NOTE: Currently compiles the Linux target with cargo instead
        of cross

    DEPENDENCIES:
        Cargo  ^1.95.0 (Rust CLI) <https://doc.rust-lang.org/cargo>
        cross  ^0.2.5  (Rust CLI) <https://github.com/cross-rs/cross>
            rustup
            Docker/podman
        gzip   *       (should come with CPython)
"""


import subprocess
import tarfile
import os

from zipfile import ZipFile


# The targets we're building
# Format: (toolchain, archive_name, win)
TARGETS = [
    ("x86_64-unknown-linux-gnu", "linux_x86_64", False),
    ("x86_64-pc-windows-gnu", "win_x86_64", True),
]

# The files/dirs to add to archives that have static paths
# Format: (path, archive_path)
# Dir paths are always added recursively
STATIC_FILES = [
    ("jigsaw", "jigsaw"),
]


def info(*args, **kwargs) -> None:
    """Alias of 'print' with a common prefix."""

    print("[build.py]", *args, **kwargs)


def compile_for(target: str) -> None:
    """Compiles RPack Toolbox for the specified target."""

    info(f"Compiling target '{target}'...")
    args = ["cross", "build", "--release", "--target", target]

    # HACK: For some reason I can't get cross to compile for the
    # Linux target
    if "linux" in target:
        args[0] = "cargo"

    cmp = subprocess.run(args, text=True)

    if cmp.returncode != 0:
        raise RuntimeError(
            f"'{" ".join(args)}' returned the non-0 code '{cmp.returncode}'",
        )


def write_tar(target: str, path: str) -> None:
    """Writes a release package as a tar file."""

    path += ".tar.gz"
    info(f"Writing '{path}'...")

    with tarfile.open(path, "w:gz") as tar:
        tar.add(f"target/{target}/release/rpack_toolbox", "rpack_toolbox")

        for sf_path, archive_path in STATIC_FILES:
            tar.add(sf_path, archive_path)


def write_zip(target: str, path: str) -> None:
    """Writes a release package as a zip file."""

    path += ".zip"
    info(f"Writing '{path}'...")

    with ZipFile(path, "w") as zip:
        zip.write(
            f"target/{target}/release/rpack_toolbox.exe",
            "rpack_toolbox.exe",
        )

        for sf_path, archive_path in STATIC_FILES:
            if os.path.isfile(sf_path):
                zip.write(sf_path, archive_path)
                continue
            elif not os.path.isdir(sf_path):
                info(f"'{sf_path}': unsupported file type. skipping...")
                continue

            # Recursively add the items of static dirs
            for dir_name, _, files in os.walk(sf_path):
                for file in files:
                    file = os.path.join(dir_name, file)
                    file_rel = os.path.relpath(file, sf_path)
                    zip.write(file, os.path.join(archive_path, file_rel))


def build(target: str, out_suffix: str, win: bool = False) -> None:
    """Combines 'compile_for' and the 'write_*' functions."""

    compile_for(target)

    if win:
        write_zip(target, get_file_name(out_suffix))
    else:
        write_tar(target, get_file_name(out_suffix))


def get_file_name(suffix: str) -> str:
    """Return the full path to one of the release packages."""

    return f".release/rpack_toolbox_{suffix}"


def main() -> None:
    """The main logic of the program."""

    target_count = len(TARGETS)

    for i, args in enumerate(TARGETS):
        info(f"Building target {i + 1}/{target_count}...")
        target, suffix, win = args
        build(target, suffix, win=win)
        info(f"Target {i + 1}/{target_count} complete\n")

    info("All done!")


if __name__ == "__main__":
    main()
