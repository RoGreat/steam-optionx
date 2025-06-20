# Steam OptionX

## Description

An egui application to modify app launch options in Steam's config file.

This is an alpha build. Tested on linux. Should work on all platforms.

![Alpha](assets/steam-optionx-alpha.png)

Some features to try and implement:

- Sort apps by name instead of appid
- List only installed apps
- More launchers???

## Build

```sh
nix develop
cargo build --release
```

## Run

```sh
cargo run --release
```
