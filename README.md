<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/rotex-labs/rotex-assets/55fa2474cb788a445c33b4ccf615eb8bbb9dbe64/brand/github/rotex-readme-banner-dark.svg">
    <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/rotex-labs/rotex-assets/55fa2474cb788a445c33b4ccf615eb8bbb9dbe64/brand/github/rotex-readme-banner-light.svg">
    <img alt="Rotex: One API. Every GPU. Zero cost." src="https://raw.githubusercontent.com/rotex-labs/rotex-assets/55fa2474cb788a445c33b4ccf615eb8bbb9dbe64/brand/github/rotex-readme-banner-light.svg" width="1200">
  </picture>
</p>

# rotex-ext-indirect

Indexed indirect draws as `RhiCommand::Extension`.

> [!WARNING]
> **Beta, pre-1.0.** Rotex is under active development. APIs, behavior, and crate
> boundaries can change in any 0.x release. Do not depend on it for production work yet.

## Overview

`rotex-ext-indirect` is the pattern for later `rotex-ext-*` crates. `draw_indexed_indirect` pushes an `Extension` onto a `CommandEncoder` whose backend implements `Supports<IndirectDraw>`. The native call stays in the backend.

## Responsibilities

- `IndexedIndirectDraw` and the little-endian payload layout
- A helper gated on `Supports<IndirectDraw>`
- Optional `vulkan` and `wgpu` features that check those bridges implement the trait

## Dependencies

`rotex-core` and `rotex-types`. `rotex-vulkan` and `rotex-wgpu` are optional and off by default.

## Status

Version 0.2.0. Backends do not depend on this crate. Any other feature id or opcode returns unsupported inside the backend.

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
* MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.
