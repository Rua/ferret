# Ferret

Ferret is a game engine that aims to be compatible with the original Doom, and possibly Quake in the distant future. It's mostly just a hobby project that I am trying out for myself. The goal is for me to learn and become more experienced in game programming, rather than to make something amazing. It's not intended to be 100% compatible down to the exact details, meaning it will not be capable of networking with other engines or playing demo files, but it should be close enough to give the same gameplay experience.

The project is still an early work in progress, so it won't do much useful yet. Error handling is nearly nonexistent, so expect it to crash whenever anything is wrong.

## Requirements

Ferret is made in [Rust](https://www.rust-lang.org/), so you'll need to install that. Some of the dependencies also require support for other languages. On Linux Mint, I needed the following packages:

```
cmake
libc6-dev
g++
python3-distutils
```

Ferret uses Vulkan for rendering, so you need to have a Vulkan-capable graphics card and the appropriate drivers installed. The package `mesa-vulkan-drivers` is needed on Linux Mint.

Finally, the engine requires the `doom.wad` file from the original game in order to run. It should be placed in the root directory of the project, next to `doom.gwa` which is already present.

## Contributing

Since this is a learning project for myself, I'm not looking for contributions from others. Issues may be submitted, but keep in mind the early state of the project; a bug may actually be one of the many features that is yet to be implemented. If you have constructive suggestions to improve the code, those are also welcome, but try to explain how it works and why it's an improvement so I can learn from it as well.

## Why "Ferret"?

A few existing Doom engine names are puns on the original name, like "Boom" and "Vavoom". Thinking of similar options for a name, I came upon "dook", the name for the noise ferrets make when excited. From there, the name became just "Ferret", because ferrets are cute.
