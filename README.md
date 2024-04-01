# A Simple Sandpile in Rust + Pyxel

A sample Rust + Pyxel app showing a small pile of sand evolving according
to Abelian Sandpile rules [1].

Keyboard map:
 - SPACE: add more grains
 - Z: erase all sand
 - LEFT/RIGHT: modify update speed

## Building

Just use `cargo run` to run locally.

On Linux you might have an issue with SDL.h PATH while building
pyxel-platform. To fix it, let BINDGEN_EXTRA_CLANG_ARGS point
to the proper directory. For Ubuntu/Debian systems:
`BINDGEN_EXTRA_CLANG_ARGS=-I/usr/include/SDL2 cargo run`


## References
 1. https://en.wikipedia.org/wiki/Abelian_sandpile_model
