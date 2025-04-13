# Building

Run `make` to build, or `make run` to run.

# Necessary dependencies

* `cargo` and all of the Rust build suite
* `qemu-system-x86` (from `apt`)

# Structure

We break this project into two Rust workspaces:

* `os`, which contains code built for the target `x86_64-funcos`.
    This uses `#![no_std]`, and custom builds of necessary parts of Rust's core.
* `run`, which is compiled for the host machine.
