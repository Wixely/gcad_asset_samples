# Procedural normal maps

Run `sh tools/normal-maps/regenerate.sh` from the missile-launcher directory.
Alternatively use `cargo run --release --locked --manifest-path tools/normal-maps/Cargo.toml -- .`.

This regenerates 28 shared 512-square positive-Y tangent-space normal maps,
`normal-map-manifest.csv` and `normal-bindings.gasset.txt`. It does not change
the asset declarations automatically. These maps are linear numeric data;
do not import them as sRGB colour images.

The generator provides surface microdetail, not a sculpt bake. Paint stencils
do not become raised lettering. UV seams can interrupt the procedural detail.
Dependencies are pinned by Cargo.lock. Generated build files belong in target/.

The repository's VS Code launch configuration builds and debugs the generator
on Windows using the Microsoft C/C++ debugger extension. Running it regenerates
the maps in this sample. Debugger launch itself has not been verified; ordinary
Cargo build and the existing generation workflow are independent of the editor.
