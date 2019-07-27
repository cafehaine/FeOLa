# What is FeOLa

FeOLa is the FerrousOxideLauncher. It is meant to be a modular clone of
Apple's spotlight, for Linux with Wayland, written in Rust.

However, due to the modular nature of the project, it might be compatible with
much more than just Linux with Wayland, only time will tell.

# Current status

Nothing is done, it's just an idea.

# Modular structure

The project will be split in three parts:

## The sources

The sources are shared objects that get an input string, and return a list of
results. These results could be anything from links, to binaries, to desktop
files, to just text.

## The core

The core will run as a daemon, and will accept input from the fronted, and will
transmit it to all of the loaded sources. It will then send back the results to
the frontend.

## The frontends

The frontends are independent processes that will take care of showing an input
to the user, and executing the result, based on what the core gave in return.

# Disclaimer

I don't know how to code in Rust and I've also never played around with `.so`.

This will be a learning experience for me, so if you think something could be
written in a better way, tell me!
