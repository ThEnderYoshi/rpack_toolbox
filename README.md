# RPack Toolbox

RPack Toolbox is a tool that aids in the creation of [*Terraria*] [resource
packs](https://terraria.wiki.gg/wiki/Resource_Pack).

> [!NOTE]
> Currently, this program is only available as a command-line tool. However, a
> graphical interface is planned.

## Features

RPack Toolbox's features are divided into Tools.

> [!TIP]
> On the CLI, you can use the `help` command to get more specifc details and
> instructions for each tool.

The tools are as follows:

### Scan

> CLI command: `scan`

Counts how many assets have been replaced, detects invalid assets and displays
other useful information about your pack.

![an example output from the scan tool](repo/demo_scan_cli.png)

Scan's insights can also be dumped to a JSON file for convenient automation.

### Generate

> CLI command: `gen`

Generates the reference files used by the Scan tool from the extracted
game assets.

![an example output from the generate tool](repo/demo_gen_cli.png)

> [!NOTE]
> You only need to run this once every game or major RPack update.

## Installation and Setup

Before you install, you currently need to also install trigger-segfault's
[TConvert](https://github.com/trigger-segfault/TConvert) and
[TerrariaLocalizationPacker](https://github.com/trigger-segfault/TerrariaLocalizationPacker)
tools so you can extract the game's assets. RPack expects the extracted assets
to be layed out in the exact way these programs write them.

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

We'll need those files to generate the reference files used by the Scan tool.

Now, install this program. Just look at the latest release to the right, go down
to the `Assets` section, and download the file named after your system.

Run the program from the command line with the following arguments:

```bash
./rpack_toolbox help gen
```

This command will print instructions on how to use the Generate tool. Follow
them to generate the reference files. From there you can start using the tool!

> [!TIP]
> You can delete the extracted files after using the Generate tool if you want,
> though I'd recommend you keep them around as reference when creating your
> resource packs.

## License

This program is licensed under the [GNU General Public License
version 3.0](LICENSE).

## Junk for Nerds

The remaining sections are mostly relevant to developers. If you're just here to
use the tool, you can stop reading.

### Project Structure

The program is divided into a few crates in a [Cargo] workspace:

```text
shared (the main backend logic)
cli (the CLI frontend)
app (the main binary crate)
```

### Building

If you'd rather build this from the source, clone this repo and build it
with [Cargo]:

```bash
$ git clone https://github.com/ThEnderYoshi/rpack_toolbox.git
$ cd rpack_toolbox
$ cargo build --release
```

The program will be in the `target/release` dir.

<!-- NOTE: Uncomment when features are added
To only build with one of the frontends, enable only one of the `gui` or
`cli` features:

```bash
$ cargo build --release --no-default-features -F gui
$ cargo build --release --no-default-features -F cli
```
-->

<!-- References -->

[*Terraria*]: https://terraria.org
[Cargo]: https://doc.rust-lang.org/cargo
