# RPack Toolbox

RPack Toolbox is a tool that aids in the creation of [*Terraria*] [resource
packs](https://terraria.wiki.gg/wiki/Resource_Pack).

> [!NOTE]
> Currently, this program is only available as a command-line tool. However, a
> graphical interface is planned.

## Features

- `generate`: Generates reference files used by `scan`. You only need to run
  this once every game update.
- `scan`: Counts how many assets have been replaced, detects invalid assets and
  displays other useful information about your pack.
  - `scan`'s insights can also be dumped to a JSON file.

## Installation and Setup

Before you install, you currently need to also install trigger-segfault's
[TConvert](https://github.com/trigger-segfault/TConvert) and
[TerrariaLocalizationPacker](https://github.com/trigger-segfault/TerrariaLocalizationPacker)
tools.

Use both of those tools to extract the game's assets. They can be extracted
wherever you want, just make sure *both of them write files to the same folder*.
The final folder should look something like this:

```plain_text
ExtractedTerraria/
    Fonts/
        <a bunch of png files>
    Images/
        <some folders and a bunch of png files>
    Sounds/
        <a folder and a bunch of wav files>
    <a bunch of wav files>
    <a bunch of json files>
```

We'll need those files to generate the reference files used by the `scan` tool.

Now, install this tool. Just look at the latest release to the right, go down to
the `Assets` section, and download the file named after your system.

Run the program from the command line with the following arguments:

```bash
./rpack_toolbox help gen
```

This will print instructions on how to use the `generate` tool. Follow them to
generate the reference files. From there you can start using the tool!

## Building

If you'd rather build this from the source, clone this repo and build it
with [Cargo](https://doc.rust-lang.org/cargo):

```bash
$ git clone https://github.com/ThEnderYoshi/rpack_toolbox.git
$ cd rpack_toolbox
$ cargo build --release
```

The program will be in the `target/release` dir.

<!-- NOTE: Uncomment when features are added
To only build with one of
the frontends, enable only one of the `gui` or `cli` features:

```bash
$ cargo build --release --no-default-features -F gui
$ cargo build --release --no-default-features -F cli
```
-->

<!-- References -->

[*Terraria*]: https://terraria.org
