# Ferret

Ferret is a game engine that aims to be compatible with the original Doom, and possibly Quake in the distant future. It's is mostly just a hobby project that I am trying out for myself. The goal is for me to learn and become more experienced in game programming, rather than to make something amazing. It's not intended to be 100% compatible down to the exact details, meaning it will not be capable of networking with other engines or playing demo files, but it should be close enough to give the same gameplay experience.

The project is still an early work in progress, so it won't do much useful yet.

## Requirements

Ferret is made in [Rust](https://www.rust-lang.org/), so you'll need to install that, then run `cargo run` to build and run.

The engine requires the `doom.wad` file from the original game in order to run. It should be placed in the root directory of the project, next to `doom.gwa` which is already present.

## Contributing

Since this is a learning project for myself, I'm not looking for contributions from others. Issues may be submitted, but keep in mind the early state of the project; a bug may actually be one of the many features that is yet to be implemented. If you have constructive suggestions to improve the code, those are also welcome, but try to explain why it's an improvement so I can learn from it as well.
