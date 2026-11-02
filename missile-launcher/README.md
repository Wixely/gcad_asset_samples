# Missile launcher platform

A fictional static game prop, authored as a CAD assembly and a separately
prepared game asset. There are 29 meshes, 168 nodes and no skin or animation rig.
Geometry uses millimetres and Z-up; game export converts to metres and Y-up.

## Source files

- `platform.gasset`: authoritative textured game asset with shared mesh instances.
- `platform.gasm`, other `.gasm` files and `parts/`: editable CAD geometry and assembly.
- `textures/`: generated paint originals, prepared colour maps and procedural normals.
- `guides/`: UV layouts, labelled islands, mesh fingerprints and reference views.
- `*-prompts.txt`: artwork generation records; `paint-families.json`: paint grouping.
- `tools/normal-maps/`: deterministic Rust normal-map generator.
- `relight.sprites.json`, `turntable.sprites.json`: sprite rendering recipes.
- `preview/`: current diagnostic still and 512-square, 120-frame looping APNG.

28 shared colour maps and 28 shared normal maps cover all 29 materials. Contact
hardware has bare-metal finishes; painted structures use olive colour atlases.
Normal maps add restrained microdetail, not relief baked from a sculpted model.
Earlier paint/guide iterations are retained as authoring sources. Current texture
bindings are specified by `platform.gasset`, not by filename chronology.

Run these commands from this directory using a game-assets gcad build:

```sh
gcad check platform.gasm
gcad check platform.gasset
gcad view platform.gasset
mkdir -p output
gcad export platform.gasset output/launcher.glb
gcad asset-sprites platform.gasset relight.sprites.json output/sprites
gcad asset-normalize-textures platform.gasset output/delivery --pixels-per-metre 128 --max-dimension 2048
```

Export destinations must be new. Use `albedo.png` plus linear `normals.png` for
runtime sprite lighting; `sprites.png` contains baked lighting and soft shadows.
The default sprite recipe renders eight views. For a smooth turntable, sample
model angles 0 through 357 in three-degree steps, 512-square cells, 40 frames per
sheet, 8 columns, 2-pixel padding, and `normal_sheet: false`, retaining the relight
recipe's other settings. Crop each cell using its `sprites.json` rectangle, number
the resulting PNGs `frame-000.png` through `frame-119.png`, then encode with:

```sh
ffmpeg -framerate 24 -i output/frames/frame-%03d.png -frames:v 120 -c:v apng -pix_fmt rgba -plays 0 -f apng output/launcher-360.apng
```

For experimental FBX/PNG conversion, configure the reviewed native helper from
gcad, then convert the generated GLB:

```sh
gcad asset-fbx-experimental output/delivery/asset.glb output/fbx --helper /absolute/path/to/gcad_fbx_helper
```

The source textures remain high resolution. At 128 pixels/metre the normalized
launcher has approximately 125–131 pixels/metre after rounding, with texture
widths from 6 to 495 pixels. Inspect tiny islands at the intended game distance.
512 pixels/metre retains more detail but some large parts are source-limited.

Regenerate normal maps with `sh tools/normal-maps/regenerate.sh` from this directory.
After geometry changes, regenerate affected UV guides and repaint as necessary;
old guides are guarded by geometry/UV fingerprints. The separate game-asset
placements must be updated alongside CAD assembly edits.

Validation reviewed 2026-11-02: Windows CAD/game-asset checks, normalized GLB and
experimental FBX export, and sprite rendering have passed. The copied repository
is checked independently before publishing. No engine import or motion-clearance
validation is claimed. Next action (owner: sample consumer): inspect the chosen
texture density and import the exported asset into the target engine.
