# Bevy heart

This is a beating heart animation created with bevy and wgsl shader for my personal learning purpose.

It use `isosurface` to generate heart surface vertex with [this equation](https://www.wolframalpha.com/input/?i=Taubin%27s+heart+surface+Cartesian+equation)

The heart beating animation is using `sin(t + sin(t) + sin(t)) * 0.5 + 0.5` equation.

## How it looks

![til](./heart.gif)

## Build

Look at `Cargo.toml` for package it requires, also if you want build `wasm` you might want wasm-bindgen-cli to be installed.

check out `makefile` for how to build with nginx docker for web.
