# Scan Dump File Format

When running the Scan tool with the `--dump` option, a JSON file will be written
with the scan information.

The file is minified, and follows this format:

## Root

```json
{
  "<asset_kind>": {
    "replaced": "<int>",
    "total": "<int>",
    "problems": "<Problem[]>"
  }
}
```

Where:

- `<asset_kind>`: The type of asset scanned. One of:
  - `Fonts`
  - `Images`
  - `Music`
  - `Sounds`
  - `Localization` (for text problems not attached to any specific language)
  - `English Text`
  - `German Text`
  - `Italian Text`
  - `French Text`
  - `Spanish Text`
  - `Simpl. Chinese Text`
  - `Trad. Chinese Text`
  - `Portuguese Text`
  - `Polish Text`
  - `Japanese Text`
  - `Korean Text`
- `replaced`: The amount of assets of this kind in the resource pack.
- `total`: The total amount of assets in the game.
- `<Problem[]>`: An array of [`Problem`]s.

## Problem

```json
{
  "path": "<string>",
  "msg": "<string>"
}
```

Where:

- `path`: The path to the invalid file in question, relative to the asset kind's
  root folder (e.g. an invalid image's path will be relative to
  `Content/Images`).
- `msg`: A plain-text message detailing the problem.

<!-- References -->

[`Problem`]: #problem
