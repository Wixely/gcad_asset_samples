# gcad asset samples

Editable game-asset examples for [Wixely/gcad](https://github.com/Wixely/gcad),
including source geometry, textures and reproducible rendering recipes.

## Missile launcher

![Missile launcher](missile-launcher/preview/launcher.png)

The [missile launcher sample](missile-launcher/README.md) demonstrates unique UV
layouts, painted colour maps, procedural normal maps, sprite relighting and
texture-density normalization. It is a fictional static visual prop.

[View the looping turntable APNG](missile-launcher/preview/launcher-360.apng).

Use the `game-assets` branch of gcad, tested at
[`7d5b384`](https://github.com/Wixely/gcad/commit/7d5b384409c2641a7af7d3e4b16defb08333ed21),
built with `cargo build --locked --release --features game-assets`.
Native game-asset builds require a C++ toolchain and libclang. See gcad's
[game-asset documentation](https://github.com/Wixely/gcad/blob/game-assets/docs/game-assets.md).

This repository contains samples, not a copy of the gcad application. Commands
below assume that feature build is available as `gcad` on PATH.

```sh
gcad view missile-launcher/platform.gasset
```

Sample sources, artwork and tooling are distributed under [MIT](LICENSE), to the
extent copyright applies. See [asset provenance](missile-launcher/PROVENANCE.md).
Dependencies retain their own licenses.

Reviewed: 2026-11-02. Windows validation is documented with the sample; Linux and
game-engine import remain untested. Next action (owner: sample consumer): preview
the model and choose the desired export density for the target game.
