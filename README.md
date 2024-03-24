![](/screenshots/demo1.png?raw=true)

## Description
This is a very simple project I made to visualize the Mandelbrot set using Rust. The Mandelbrot set is a fractal defined by iterating $z_{i+1} = z_i^2 + c$ (where $z_0 = 0$ and $c$ is a coordinate in complex space) until the value escapes a disk of radius $2$ ($|z_i| \geq 2$). If it does escape, the point is not in the set, otherwise, it is. We can color each pixel by how many iterations it takes break this condition to make a nice visualization.

The default controls are left-clicking to zoom, right-clicking to go back to the previous view, and `space` to toggle a box which shows the next zoom selection.

## Installation

### Dependencies for [pix-engine](https://crates.io/crates/pix-engine):
#### macOS, Linux, or Windows 10 Subsystem for Linux (homebrew)
```shell
brew install sdl2 sdl2_gfx sdl2_image sdl2_mixer sdl2_ttf
```

#### Linux (package manager)
Note: The minimum SDL2 version is 2.0.20. Some package managers may not have the latest versions available.

- Ubuntu:
```shell
sudo apt-get install libsdl2-dev libsdl2-gfx-dev libsdl2-image-dev
libsdl2-mixer-dev libsdl2-ttf-dev
```

- Fedora:
```shell
sudo dnf install SDL2-devel SDL2_gfx-devel SDL2_image-devel SDL2_mixer-devel SDL2_ttf-devel
```

- Arch:
```shell
sudo pacman -S sdl2 sdl2_gfx sdl2_image sdl2_mixer sdl2_ttf
```

#### Windows (MSVC)
  1. Download the latest SDL2 MSVC development libraries from [https://www.libsdl.org/download-2.0.php](https://www.libsdl.org/download-2.0.php) e.g. (SDL2-devel-2.0.20-VC.zip).
  2. Download the latest SDL2_image, SDL2_mixer, and SDL2_ttf MSVC development libraries from [https://www.libsdl.org/projects/](https://www.libsdl.org/projects/). e.g. (SDL2_image-devel-2.0.5-VC.zip).
  3. Unzip each .zip file into a folder.
  4. Copy library files:
     - from: `lib\x64\`
     - to: `C:\Users\{Username}\.rustup\toolchains\{current toolchain}\lib\rustlib\{current toolchain}\lib` where `{current toolchain}` is likely `stable-x86_64-pc-windows-msvc`.
            Note: If you don’t use rustup, See [`rust-sdl2`](https://github.com/Rust-SDL2/rust-sdl2#sdl20-development-libraries) for more info on Windows installation.
  5. Copy all dll files:
     - from: `lib\x64\`
     - to: your cargo project next to `Cargo.toml`.
  MSVC binaries for SDL2 are also present in this repository under the lib folder.

### Installation with `cargo`:
After the dependencies are installed, you can run:
```shell
cargo install --git https://github.com/denehoffman/mandelbrot.git
```
This will add the executable `mandelbrot` to your path. Run `mandelbrot` for the default viewer, or run `mandelbrot --help` for more options, including changing window size and colorscheme.
