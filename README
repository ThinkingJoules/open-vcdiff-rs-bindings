# open-vcdiff Rust Bindings

This repository contains Rust bindings for [the Open-VCDIFF](https://github.com/google/open-vcdiff) C++ library. It provides tools to encode and decode data using the VCDIFF format.

I couldn't get [the existing Rust bindings](https://github.com/dflemstr/vcdiff-rs) to work, so I decided to create my own. This project is still a work in progress, so please feel free to contribute. This updated lib uses cc instead of gcc for building the C++ code in Rust. I also brought in the latest version of open-vcdiff repo.

At some point it would be nice to implement the streaming API, but for now I just followed pattern used in the previous repo. There is a simple encode and decode function.

I haven't really done much ffi (or any C++), so I'm sure there are some things that could be improved. I'm open to suggestions.

# Prerequisites

Before building the Rust bindings, ensure you have the following installed on your system:

- Rust (latest stable version)
- Cargo (comes with Rust installations)
- C++ compiler (GCC or Clang)

# Getting Started

Follow these steps to set up and build the library:

## 1. Clone the Repository

Clone this repository and its submodules using:

```bash
git clone --recursive https://github.com/thinkingjoules/open-vcdiff-rs-bindings.git
```
## 2. Init the submodules
Navigate to the project directory and init:

```bash
git submodule update --init --recursive
```
## 3. Cargo Build
This should hopefully build the project without errors.
```bash
cargo build
```
## 4. Make sure it works
You can also try the simple tests provided to make sure everything is working as expected:

```bash
cargo test
```

