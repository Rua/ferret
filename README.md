# Ferret

Ferret is a game engine that aims to be compatible with the original Doom, and possibly Quake in the distant future. It's mostly just a hobby project that I am trying out for myself. The goal is for me to learn and become more experienced in game programming, rather than to make something amazing. It's not intended to be 100% compatible down to the exact details, meaning it will not be capable of networking with other engines or playing demo files, but it should be close enough to give the same gameplay experience.

The project is still an early work in progress, so it won't do much useful yet. Error handling is nearly nonexistent, so expect it to crash whenever anything is wrong.

## Instructions

Ferret is only available as source code for now, so you'll need to build it yourself. Download Ferret's source code via the green button on the main page. Unpack the file somewhere. Before you're able to build it, you need to get some things set up.

### Rust

Ferret is written in Rust. The Rust compiler and build system can be downloaded and installed from https://www.rust-lang.org/tools/install.

### Shaderc

Ferret uses [shaderc-rs](https://github.com/google/shaderc-rs) to compile shaders. During the overall build process, it will either use the shaderc library if it can find it on your system, or try to download and compile it from scratch. Compiling shaderc requires you to install additional packages, including a C compiler and Python, and also slows down the build process, so it's highly recommended to install shaderc on your system before building Ferret. 

You can download readily-built files for shaderc at https://github.com/google/shaderc/blob/master/downloads.md. On Linux, it should be unpacked in the `/usr` directory. On Windows, you can place it anywhere you want, but you need to set the environment variable `SHADERC_LIB_DIR` to the location of shaderc's `lib` folder. Information on how to set environment variables can be found on various websites.

### Vulkan

Ferret uses Vulkan for rendering, so to run it, you need to have a Vulkan-capable graphics card and the appropriate drivers installed. The drivers must support Vulkan 1.1 at minimum. The package `mesa-vulkan-drivers` is needed on Linux Mint.

### Doom

Finally, the engine requires the `doom.wad` file from the original game in order to run. It should be placed in the root directory of the project, next to `doom.gwa` which is already present.

### Building Ferret

Once you have everything set up, open a terminal/command prompt, and use `cd` to go to the location where you unpacked Ferret's source code. Then, type `cargo run --release` to build and run Ferret. You can leave out `--release`, which will build quicker but Ferret itself will run much slower.

## Contributing

Since this is a learning project for myself, I'm not looking for contributions from others. Issues may be submitted, but keep in mind the early state of the project; a bug may actually be one of the many features that is yet to be implemented. If you have constructive suggestions to improve the code, those are also welcome, but try to explain how it works and why it's an improvement so I can learn from it as well.

## Why "Ferret"?

A few existing Doom engine names are puns on the original name, like "Boom" and "Vavoom". Thinking of similar options for a name, I came upon "dook", the name for the noise ferrets make when excited. From there, the name became just "Ferret", because ferrets are cute.
