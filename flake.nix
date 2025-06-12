{
  description = "A basic Rust devshell for NixOS users developing Leptos";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      nixpkgs,
      rust-overlay,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      with pkgs;
      {
        devShells.default = mkShell {
          nativeBuildInputs = [
            pkg-config
            gobject-introspection
            cargo
            cargo-tauri
            nodejs
          ];

          buildInputs =
            [
              at-spi2-atk
              atkmm
              cacert
              cairo
              cargo-make
              gcc
              gdk-pixbuf
              glib
              gtk3
              harfbuzz
              librsvg
              libsoup_3
              openssl
              pango
              pkg-config
              trunk
              webkitgtk_4_1
              (rust-bin.selectLatestNightlyWith (
                toolchain:
                toolchain.default.override {
                  extensions = [
                    "rust-src"
                    "rust-analyzer"
                  ];
                  targets = [ "wasm32-unknown-unknown" ];
                }
              ))
            ]
            ++ pkgs.lib.optionals pkg.stdenv.isDarwin [
              darwin.apple_sdk.frameworks.SystemConfiguration
            ];

          shellHook = '''';
        };
      }
    );
}
