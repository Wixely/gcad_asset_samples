# Asset provenance

Created 2026-11-02 for the gcad game-asset experiments.

Geometry was procedurally authored in gcad for this fictional prop. Painted
textures were generated using OpenAI image generation against gcad UV guides,
then resized to the guide canvas and padded with `asset-paint-apply`. Prompts and
original generated images are included. Generated painting approximates the UV
layout; seamless artwork is not guaranteed.
Generation-specific identifiers and timestamps were removed from embedded PNG
provenance metadata in the published copies; their pixel data was preserved.
The original working project remains unchanged. Generated origin is recorded here.

Normal maps are produced by the included Rust tool, using procedural paint grain,
machining grooves, composite weave, rubber and glass detail. They are independent
of painted lettering and are not sculpted-detail bakes. Rendered previews derive
from these sources through gcad; the APNG was assembled with FFmpeg.

`LicenseRef-Project-Generated` in the asset declarations identifies this project's
generated artwork. For this repository that content is offered under the root
MIT license, to the extent copyright applies. It does not identify a third-party
stock asset license. No externally sourced model or texture pack is included.
Rust dependencies listed in the tool's Cargo.lock remain separately licensed.
