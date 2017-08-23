# zydis
Rust bindings to the zydis disassembler engine

# Building
At least clang 3.7 and any other compiler is required.

See [here](https://rust-lang-nursery.github.io/rust-bindgen/requirements.html) for more information on what is
required by bindgen.

## Building on Ubuntu
Only having clang installed should be enough. If it isn't, then also install gcc.

## Building on Windows
You'll need to download and install clang, and also have msvc installed.

You can set the `LIBCLANG_PATH` environment variable to point this tool to where libclang is installed. You can specify
either the path to `clang.dll` or the path to `libclang.dll`.

Make sure to run the `cargo.exe build` from a visual studio developer prompt.
