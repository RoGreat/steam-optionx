# Steam OptionX

An [egui](https://github.com/emilk/egui) application to modify app launch options in Steam's config file.

Only takes publicly listed games from the [Steam API](https://partner.steamgames.com/doc/webapi/ISteamApps#GetAppList) for now.

Works on all platforms.

![Screenshot](assets/screenshot.png)

## Installation

### Linux

#### Nix

Install [NUR](https://github.com/nix-community/NUR) then install the package.

NixOS example:

```nix
# configuration.nix
environment.systemPackages = with pkgs; [
  nur.repos.rogreat.steam-optionx
];
```

## Build

```sh
nix develop
cargo build --release
```
