# Jigsaw Config File Format

The Jigsaw tool uses config files to determine how to cut the input images into
pieces and then rearrange those pieces into the output images.

These config files are written as [TOML](https://toml.io) files, which are
ordinary text files that use the `.toml` file extension instead of `.txt` and
are formatted in a way both humans and computers can understand. Try opening the
`.toml` files in the `jigsaw` folder in a text editor to see what a full config
file looks like!

---

This document explains how each part of a config file is structured and what
kind of data is expected. There is also a [short example config](#example) at
the bottom of this document.

## Root

These items should be placed at the file's top-level (before any `[...]` or
`[[...]]` header).

```toml
name = "<string>"
in = "<Input>"
out = "<Output>"
```

Where:

- `name`: The name of the config, displayed in the GUI or in the printed output.
- `in`: An [`Input`] table. Configs that affect the input images go here.
- `out`: An [`Output`] table. Configs that affect the input images go here.

> [!NOTE]
> You usually write the `in` and `out` tables in the header format:
> 
> ```toml
> name = "..."
> 
> [in]
> # ...
> 
> [out]
> # ...
> ```

## Input

[`Input`]: #input

This table stores configs related to the input image(s), mainly how to cut them
up into pieces.

```toml
grid = "<Grid>"
pieces = { "<PieceId>" = "<Piece>" }
```

Where:

- `grid` (optional): A [`Grid`] table. Defaults to
  `{ size = "1:1", offset = "0:0" }`.

  This grid is used as the coordinates of the pieces.

- `pieces`: Defines the pieces that will be cut out of the input images.

  Each key is the ID of a piece and each value is a [`Piece`] table.

> [!NOTE]
> You usually write the `pieces` table in the header format:
> 
> ```toml
> [in.pieces]
> # ...
> ```

### Piece

[`Piece`]: #piece

Represents a piece cut from an input image.

Each piece is defined as a string of 3 space-separated [`IVec2`]s:

```
<Pos> <Size> <Offset>
```

Where:

- `<Pos>`: The position of the top-left corner of the piece, in *grid
  coordinates*.

  `0:0` is the top-left grid cell, +X moves rightward and +Y moves downard.

- `<Size>`: The size of the piece, in *pixels*.

  Both the width and height must be at least `1`, otherwise the piece will be
  considered invalid.

- `<Offset>`: When this piece is placed (see [`Placement`]), its final position
  will be offset from the placement's position by this value, in *pixels*.

  This allows multiple pieces to be used in the same placement
  without overlapping.

## Outout

This table stores configs related to the output image(s).

[`Output`]: #output

```toml
size = "<IVec2>"
grid = "<Grid>"
scale = "<number>"
placements = { "<IVec2>" = "<string>" }
```

Where:

- `size`: A [`IVec2`]. This will be the size of the output image, in pixels.

  The width and height must be at least `1`, otherwise you'll get an error!

- `grid` (optional): A [`Grid`] table. Defaults to
  `{ size = "1:1", offset = "0:0" }`.

  This grid is used as the coordinates of the placements.

- `scale` (optional): A scale factor applied to the output images after all
  other processes. Defaults to `2.0`.

  A scale of `1` means "no scaling", `2.0` means "twice as large", `0.5` means
  "half as large", etc.

- `placements`: Defines where the pieces defined in [`in.pieces`][`Input`]
  are placed.

  Each key is an [`IVec2`] that represents the coordinates of the placement, in
  grid coordinates.

  Each value is a string that contains a space-separated list of the IDs of the
  pieces that should be placed here, e.g. `"piece1 piece2 piece3"`.

> [!NOTE]
> You usually write the `placements` table in the header format:
> 
> ```toml
> [out.placements]
> # ...
> ```

## Grid

Defines a set of grid coordinates for positioning pieces in input images and
placements in output images.

[`Grid`]: #grid

```toml
size = "<IVec2>"
offset = "<IVec2>"
```

Where:

- `size` (optional): The size of each grid cell, in pixels. Defaults to `"1:1"`.

  The width and height must be at least `1`, otherwise the grid will be invalid.

- `offset` (optional): Determines the position of the top-left corner of cell
  `0:0`, in pixels. Defaults to `0:0`, which is the top-left corner of the
  image itself.

  +X moves rightward and +Y moves downard.

## IVec2

[`IVec2`]: #ivec2

A 2D vector. It represents a position, size, or some other pair of whole
numbers. The name is short for "Integer Vector 2D".

`IVec2`s are written as a string containing two whole numbers separated by a
colon (`:`):

```
<X>:<Y>
```

Where:

- `<X>`: The X component of the vector. (e.g. horizontal position, width, etc.)
- `<Y>`: The Y component of the vector. (e.g. vertical position, height, etc.)

## Example

The code below is an example of a full config file, which doesn't really do
anything useful (for "real world" examples, see the the `.toml` files in the
`jigsaw` folder).

```toml
name = "Some Config"

[in]
# Terraria usually deals with 8x8 tiles, so you'll usually be dealing with 8x8
# (or 4x4) tiles as well
grid = { size = "8:8", offset = "8:8" }

[in.pieces]
# While this is often not recommended in other cases, here we're giving the
# pieces shortened names. This makes it harder to infer their meaning, but it
# will make the placement strings a lot more readable!
#
# TIP: I like to put the full name of the piece as a comment to make it easier
# to tell what its ID means
sp = "0:0 8:8 0:0" # Some Piece
op = "1:0 4:8 4:0" # Other Piece

[out]
size = "16:16"

# Many of Terraria's tilesets have a 1px gap between tiles (after downscaling).
# We can simulate this here by adding 1px to the grid size
grid = { size = "9:9" }
# You can also write the above line as 'grid.size = "9:9"'

# We're using the default value for 'scale' (2.0) so we don't need to specify
# it here

[out.placements]
"0:0" = "sp op" # Place pieces 'sp' and 'op' in the grid cell '0:0'
"1:0" = "sp"    # Place the piece 'sp' in the grid cell '1:0'
"0:1" = "op"    # And so on
```
