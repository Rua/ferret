# Ferret

Ferret is a game engine that aims to be compatible with the original Doom, and possibly Quake in the distant future. It's mostly just a hobby project that I am trying out for myself. The goal is for me to learn and become more experienced in game programming, rather than to make something amazing. It's not intended to be 100% compatible down to the exact details, meaning it will not be capable of networking with other engines or playing demo files, but it should be close enough to give the same gameplay experience.

The project is still in development and far from finished, so expect missing features and crashes.

## Building

Ferret is only available as source code for now, so you'll need to build it yourself. Download Ferret's source code via the green button on the main page. Unpack the file somewhere. Before you're able to build it, you need to get some things set up.

Ferret is written in Rust. The Rust compiler and build system can be downloaded and installed from https://www.rust-lang.org/tools/install.

Ferret uses [shaderc-rs](https://github.com/google/shaderc-rs) to compile shaders. During the overall build process, it will either use the shaderc library if it can find it on your system, or try to download and compile it from scratch. Compiling shaderc requires you to install additional packages, including a C compiler and Python, and also slows down the build process, so it's highly recommended to install shaderc on your system before building Ferret. Shaderc is included in the [Vulkan SDK](https://www.lunarg.com/vulkan-sdk/), and installing it is the easiest way to get everything set up correctly. Alternatively, you can download readily-built files for shaderc at https://github.com/google/shaderc/blob/master/downloads.md, but you'll have to put them in the correct place yourself.

Once you have everything set up, open a terminal/command prompt, and use `cd` to go to the location where you unpacked Ferret's source code. Then, type `cargo build --release` to build. You can leave out `--release`, which will build quicker but Ferret itself will run less efficiently.

## Running

The easiest way to run Ferret, once built, is with `cargo run --release`. If you'd prefer to run the executable directly, it is located in `target/release` after building. The executable expects to find `console.hex` in its current directory when it runs, so ensure that is the case.

Ferret uses Vulkan for rendering, so to run it, you need to have a Vulkan-capable graphics card and the appropriate drivers installed. The drivers must support Vulkan 1.1 at minimum.

A variety of locations are used to load and store data:

| OS              | Linux                 | Windows                          |
|-----------------|-----------------------|----------------------------------|
| Settings, saves | ~/.config/ferret/     | C:\Users\\(user)\AppData\Roaming |
| Screenshots     | ~/Pictures            | C:\Users\\(user)\Pictures        |
| WADs            | ~/.local/share/ferret | C:\Users\\(user)\AppData\Roaming |

Ferret requires a Doom IWAD to be placed in the WADs directory in order to run. The shareware version of Doom, `doom1.wad`, can be downloaded for free [here](https://distro.ibiblio.org/slitaz/sources/packages/d/doom1.wad). WADs for the paid versions, e.g. `doom.wad`, `doom2.wad` and the expansions `plutonia.wad` and `tnt.wad`, will also work, but you will have to acquire them yourself from the source of your preference. Ferret can run directly from these WADs, but it may be buggy and have small holes in floors and ceilings. It is highly recommended to run [glBSP](http://glbsp.sourceforge.net/) on the WAD files, which will produce a `.gwa` file with the same name.

## Contributing

Since this is a learning project for myself, I'm not looking for contributions from others. Issues may be submitted, but keep in mind the early state of the project; a bug may actually be one of the many features that is yet to be implemented. If you have constructive suggestions to improve the code, those are also welcome, but try to explain how it works and why it's an improvement so I can learn from it as well.

## Internals

Internally, Ferret is designed completely differently from the original engine, and doesn't use any of the original code. As its main framework it uses the [Legion](https://github.com/amethyst/legion) Entity Component System, which separates operations into well-defined "systems" and "components" and allows for easy data parallelism. The engine is multithreaded to a limited degree.

For rendering, Ferret uses the [Vulkano](https://github.com/vulkano-rs/vulkano) library, which is a high-level safe wrapper around the cross-platform Vulkan API. This means that the rendering is done in hardware, using triangles and shaders instead of the classic software renderer. The framebuffer is high resolution, and can be any size; the game content will reposition itself automatically. The shaders use the same lighting calculation as the original game, but in full 32-bit colour, so there are no sudden changes in brightness at fixed distances, and no loss of colour precision at low brightnesses.

The physics engine is very different, and takes its inspiration from the Quake engines, which use polygonal collision brushes for the map and axis-aligned bounding boxes for entities. Movement and collision is calculated in full 3D, rather than horizontal first and then vertical as a separate step. Because of this, Ferret does not have some of the bugs (or features?) of the original engine: there is no wallrunning, no infinite-height monsters or explosions, no linedef skips, no run-grabbing of items that should be out of reach. Custom maps that rely on these features to work properly will not be fully playable in Ferret.

## Why "Ferret"?

Because Ferret is a Dook engine! Dook is the sound a ferret makes when it's happy and playful. A few existing Doom engine names are puns on the original name, like "Boom" and "Vavoom", so I started with "dook" and then went a step further.
